use std::sync::Arc;

use glam::*;
use godot::{
    classes::Engine,
    obj::{bounds::DeclUser, Bounds},
    prelude::*,
};
use indexmap::IndexMap;
use rayon::prelude::*;

mod boid;
mod boid_properties;
mod flock;
mod flock_properties;

pub use boid::{boid_2d::*, boid_3d::*, Boid};
pub use boid_properties::BoidProperties;
pub use flock::{flock_2d::*, Flock};
pub use flock_properties::FlockProperties;

use rustc_hash::FxBuildHasher;

type FxIndexMap<K, V> = IndexMap<K, V, FxBuildHasher>;

const SINGLETON_NAME: &str = "Boids";

fn get_singleton_name() -> StringName {
    StringName::from(SINGLETON_NAME)
}

fn get_singleton() -> Gd<Boids> {
    Engine::singleton()
        .get_singleton(get_singleton_name())
        .unwrap()
        .cast()
}

struct BoidsExtension;

#[gdextension]
unsafe impl ExtensionLibrary for BoidsExtension {
    fn on_level_init(level: InitLevel) {
        match level {
            InitLevel::Scene => {
                let singleton = Boids::new_alloc().upcast::<Object>();
                Engine::singleton().register_singleton(get_singleton_name(), singleton);
            }
            _ => (),
        }
    }

    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            // Get the `Engine` instance and `StringName` for your singleton.
            let mut engine = Engine::singleton();
            let singleton_name = get_singleton_name();

            // We need to retrieve the pointer to the singleton object,
            // as it has to be freed manually - unregistering singleton
            // doesn't do it automatically.
            let singleton = engine
                .get_singleton(singleton_name.clone())
                .expect("cannot retrieve the singleton");

            // Unregistering singleton and freeing the object itself is needed
            // to avoid memory leaks and warnings, especially for hot reloading.
            engine.unregister_singleton(singleton_name);
            singleton.free();
        }
    }
}

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct BoidsProcess {
    #[export]
    process_2d: bool,
    #[export]
    process_3d: bool,
    #[export]
    #[init(val = 1)]
    process_per_tick: i64,
    boids: Option<Gd<Boids>>,
    engine: Option<Gd<Engine>>,
}

impl BoidsProcess {
    #[inline(always)]
    fn get_boids_singleton(&mut self) -> &mut Gd<Boids> {
        unsafe { self.boids.as_mut().unwrap_unchecked() }
    }

    #[inline(always)]
    fn get_engine_singleton(&self) -> &Gd<Engine> {
        unsafe { self.engine.as_ref().unwrap_unchecked() }
    }
}

#[godot_api]
impl INode for BoidsProcess {
    #[inline(always)]
    fn ready(&mut self) {
        self.boids = Some(get_singleton());
        self.engine = Some(Engine::singleton());
    }

    #[inline(always)]
    fn physics_process(&mut self, _: f64) {
        if self.get_engine_singleton().get_physics_frames() % (self.process_per_tick as u64) == 0 {
            if self.process_2d {
                self.get_boids_singleton().bind_mut().process_boids_2d();
            }
        }
    }
}

#[derive(GodotClass)]
#[class(init, base=Object)]
struct Boids {
    flocks2d: FxIndexMap<i64, Gd<Flock2D>>,
    boids2d: FxIndexMap<i64, Gd<Boid2D>>,
    base: Base<Object>,
}

impl Boids {
    fn register_flock_2d(&mut self, flock_id: i64) {
        let flock = godot::global::instance_from_id(flock_id).unwrap().cast();
        self.flocks2d.insert(flock_id, flock);
        godot_print!("[Boids] flock {flock_id} registered");
    }

    fn unregister_flock_2d(&mut self, flock_id: i64) {
        self.flocks2d.shift_remove(&flock_id);
        godot_print!("[Boids] flock {flock_id} unregistered");
    }

    #[inline(always)]
    fn register_boid_2d(&mut self, boid_id: i64, boid: Gd<Boid2D>) {
        self.boids2d.insert(boid_id, boid);
    }

    #[inline(always)]
    fn unregister_boid_2d(&mut self, boid_id: i64) {
        self.boids2d.shift_remove(&boid_id);
    }
}

#[godot_api]
impl Boids {
    #[func]
    #[inline(always)]
    fn process_boids_2d(&mut self) {
        process_boids(&mut self.boids2d, &self.flocks2d)
    }

    #[func]
    #[inline(always)]
    fn get_total_boid_count(&self) -> i64 {
        self.boids2d.len() as i64
    }
}

#[inline(always)]
const fn to_glam_vec(godot_vec: Vector3) -> Vec3 {
    vec3(godot_vec.x, godot_vec.y, godot_vec.z)
}

#[inline(always)]
fn process_boids<F, B>(boids: &mut FxIndexMap<i64, Gd<B>>, flocks: &FxIndexMap<i64, Gd<F>>)
where
    F: Flock + GodotClass,
    F: Bounds<Declarer = DeclUser>,
    B: Boid + GodotClass,
    B: Bounds<Declarer = DeclUser>,
{
    #[cfg(feature = "stats")]
    let time = std::time::Instant::now();
    let total_boid_count = boids.len();
    let mut calc_funcs = Vec::with_capacity(total_boid_count);
    for (_, flock) in flocks.iter() {
        let flock = flock.bind();
        let flock_props = flock.get_flock_properties();
        let target_position = flock.get_target_position();
        let boids = Arc::new(flock.get_boids_posvel());
        for (boid_id, (boid_pos, boid_vel, boid_props)) in flock.get_boids() {
            let boid_id = *boid_id;
            let flock_props = flock_props.clone();
            let target_position = target_position.clone();
            let boids = boids.clone();
            calc_funcs.push((boid_id, move || {
                boid::calculate_boid(
                    boid_pos,
                    boid_vel,
                    boid_props,
                    flock_props,
                    boids,
                    target_position,
                )
            }));
        }
    }
    #[cfg(feature = "stats")]
    godot_print!(
        "[Boids] preparing all calculations took {} ms",
        time.elapsed().as_millis()
    );

    #[cfg(feature = "stats")]
    let time = std::time::Instant::now();
    let forces: Vec<(i64, Vec3)> = calc_funcs
        .into_par_iter()
        .fold(
            || Vec::<(i64, Vec3)>::with_capacity(total_boid_count / 10),
            |mut acc, (boid_id, calc_fn)| {
                let force = calc_fn();
                acc.push((boid_id, force));
                acc
            },
        )
        .reduce(
            || Vec::<(i64, Vec3)>::with_capacity(total_boid_count / 10),
            |mut left, mut right| {
                left.append(&mut right);
                left
            },
        );
    #[cfg(feature = "stats")]
    godot_print!(
        "[Boids] calculating all boids took {} ms",
        time.elapsed().as_millis()
    );

    #[cfg(feature = "stats")]
    let time = std::time::Instant::now();
    for (boid_id, force) in forces {
        let boid = boids.get_mut(&boid_id).unwrap();
        boid.bind_mut().apply_force(force);
    }
    #[cfg(feature = "stats")]
    godot_print!(
        "[Boids] applying forces took {} ms",
        time.elapsed().as_millis()
    );
}
