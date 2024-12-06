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
mod flock;

pub use boid::{Boid, *};
pub use flock::{Flock, *};

use rustc_hash::FxBuildHasher;

type FxIndexMap<K, V> = IndexMap<K, V, FxBuildHasher>;

const SINGLETON_NAME: &str = "Boids";

fn get_singleton() -> Gd<Boids> {
    Engine::singleton()
        .get_singleton(SINGLETON_NAME)
        .unwrap()
        .cast()
}

struct BoidsExtension;

#[gdextension]
unsafe impl ExtensionLibrary for BoidsExtension {
    fn on_level_init(level: InitLevel) {
        match level {
            InitLevel::Scene => {
                let singleton = Boids::new_alloc();
                Engine::singleton().register_singleton(SINGLETON_NAME, &singleton);
            }
            _ => (),
        }
    }

    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            // Get the `Engine` instance and `StringName` for your singleton.
            let mut engine = Engine::singleton();

            // We need to retrieve the pointer to the singleton object,
            // as it has to be freed manually - unregistering singleton
            // doesn't do it automatically.
            let singleton = engine
                .get_singleton(SINGLETON_NAME)
                .expect("cannot retrieve the singleton");

            // Unregistering singleton and freeing the object itself is needed
            // to avoid memory leaks and warnings, especially for hot reloading.
            engine.unregister_singleton(SINGLETON_NAME);
            singleton.free();
        }
    }
}

#[derive(GodotClass)]
#[class(init, base=Node)]
/// Node that will make calls automatically to process 2D/3D boids, providing some configuration options.
/// It's best to use this as an autoload singleton. The plugin will register an autoload by default so you don't have to set this up yourself.
pub struct BoidsProcess {
    #[export]
    #[init(val = true)]
    /// Whether to process 2D boids or not.
    process_2d: bool,
    #[export]
    #[init(val = true)]
    /// Whether to process 3D boids or not.
    process_3d: bool,
    #[export]
    #[init(val = 1)]
    /// Process boids per N physics ticks.
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
            let (process_2d, process_3d) = (self.process_2d, self.process_3d);
            let mut s = self.get_boids_singleton().bind_mut();
            if process_2d {
                s.process_boids_2d();
            }
            if process_3d {
                s.process_boids_3d();
            }
        }
    }
}

#[derive(GodotClass)]
#[class(init, base=Object)]
/// Singleton that holds all boids and flocks and manages them.
struct Boids {
    flocks2d: FxIndexMap<InstanceId, Gd<Flock2D>>,
    boids2d: FxIndexMap<InstanceId, Gd<Boid2D>>,
    flocks3d: FxIndexMap<InstanceId, Gd<Flock3D>>,
    boids3d: FxIndexMap<InstanceId, Gd<Boid3D>>,
    base: Base<Object>,
}

impl Boids {
    fn register_flock_2d(&mut self, flock_id: InstanceId) {
        let flock = Gd::from_instance_id(flock_id);
        self.flocks2d.insert(flock_id, flock);
        godot_print!("[Boids] flock {flock_id} registered");
    }

    fn unregister_flock_2d(&mut self, flock_id: InstanceId) {
        self.flocks2d.shift_remove(&flock_id);
        godot_print!("[Boids] flock {flock_id} unregistered");
    }

    #[inline(always)]
    fn register_boid_2d(&mut self, boid_id: InstanceId, boid: Gd<Boid2D>) {
        self.boids2d.insert(boid_id, boid);
    }

    #[inline(always)]
    fn unregister_boid_2d(&mut self, boid_id: InstanceId) {
        self.boids2d.shift_remove(&boid_id);
    }

    fn register_flock_3d(&mut self, flock_id: InstanceId) {
        let flock = Gd::from_instance_id(flock_id);
        self.flocks3d.insert(flock_id, flock);
        godot_print!("[Boids] flock {flock_id} registered");
    }

    fn unregister_flock_3d(&mut self, flock_id: InstanceId) {
        self.flocks3d.shift_remove(&flock_id);
        godot_print!("[Boids] flock {flock_id} unregistered");
    }

    #[inline(always)]
    fn register_boid_3d(&mut self, boid_id: InstanceId, boid: Gd<Boid3D>) {
        self.boids3d.insert(boid_id, boid);
    }

    #[inline(always)]
    fn unregister_boid_3d(&mut self, boid_id: InstanceId) {
        self.boids3d.shift_remove(&boid_id);
    }
}

#[godot_api]
impl Boids {
    #[func]
    #[inline(always)]
    /// Process all 2D boids once.
    /// NOTE: This function is not intended to be manually called. Prefer using `BoidsProcess` as an autoload singleton where possible.
    fn process_boids_2d(&mut self) {
        process_boids(&mut self.boids2d, &self.flocks2d)
    }

    #[func]
    #[inline(always)]
    /// Process all 3D boids once.
    /// NOTE: This function is not intended to be manually called. Prefer using `BoidsProcess` as an autoload singleton where possible.
    fn process_boids_3d(&mut self) {
        process_boids(&mut self.boids3d, &self.flocks3d)
    }

    #[func]
    #[inline(always)]
    /// Gets the total 2D boid count.
    fn get_total_boid_2d_count(&self) -> i64 {
        self.boids2d.len() as i64
    }

    #[func]
    #[inline(always)]
    /// Gets the total 2D flock count.
    fn get_total_flock_2d_count(&self) -> i64 {
        self.flocks2d.len() as i64
    }

    #[func]
    #[inline(always)]
    /// Gets the total 3D boid count.
    fn get_total_boid_3d_count(&self) -> i64 {
        self.boids3d.len() as i64
    }

    #[func]
    #[inline(always)]
    /// Gets the total 3D flock count.
    fn get_total_flock_3d_count(&self) -> i64 {
        self.flocks3d.len() as i64
    }
}

#[inline(always)]
const fn to_glam_vec(godot_vec: Vector3) -> Vec3 {
    vec3(godot_vec.x, godot_vec.y, godot_vec.z)
}

#[inline(always)]
fn process_boids<F, B>(
    boids: &mut FxIndexMap<InstanceId, Gd<B>>,
    flocks: &FxIndexMap<InstanceId, Gd<F>>,
) where
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
        "[Boids] preparing all calculations took {} micros",
        time.elapsed().as_micros()
    );

    #[cfg(feature = "stats")]
    let time = std::time::Instant::now();
    let forces: Vec<(InstanceId, Vec3)> = calc_funcs
        .into_par_iter()
        .fold(
            || Vec::<(InstanceId, Vec3)>::with_capacity(total_boid_count),
            |mut acc, (boid_id, calc_fn)| {
                let force = calc_fn();
                acc.push((boid_id, force));
                acc
            },
        )
        .reduce(
            || Vec::<(InstanceId, Vec3)>::with_capacity(total_boid_count),
            |mut left, mut right| {
                left.append(&mut right);
                left
            },
        );
    #[cfg(feature = "stats")]
    godot_print!(
        "[Boids] calculating all boids took {} micros",
        time.elapsed().as_micros()
    );

    #[cfg(feature = "stats")]
    let time = std::time::Instant::now();
    for (boid_id, force) in forces {
        let boid = unsafe { boids.get_mut(&boid_id).unwrap_unchecked() };
        boid.bind_mut().apply_force(force);
    }
    #[cfg(feature = "stats")]
    godot_print!(
        "[Boids] applying forces took {} micros",
        time.elapsed().as_micros()
    );
}
