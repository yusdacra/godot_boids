use glam::*;
use godot::prelude::*;

use crate::{
    get_singleton, to_glam_vec, Boid, Boid3D, BoidProperties, FlockProperties, FxIndexMap,
};

use super::Flock;

#[derive(GodotClass)]
#[class(init, base=Node3D)]
pub struct Flock3D {
    #[export]
    /// Properties of this flock.
    /// Note: this cannot be changed in runtime, aside from removing and readding the node.
    properties: Gd<FlockProperties>,
    props: FlockProperties,
    #[export]
    /// A target node for the flock to follow.
    target: Option<Gd<Node3D>>,
    pub boids: FxIndexMap<i64, Gd<Boid3D>>,
    base: Base<Node3D>,
}

impl Flock3D {
    pub fn register_boid(&mut self, boid_id: i64) {
        let boid: Gd<Boid3D> = godot::global::instance_from_id(boid_id).unwrap().cast();
        self.boids.insert(boid_id, boid.clone());
        get_singleton().bind_mut().register_boid_3d(boid_id, boid);
        let flock_id = self.get_id();
        godot_print!("[Flock3D:{flock_id}] boid {boid_id} registered");
    }

    pub fn unregister_boid(&mut self, boid_id: i64) {
        self.boids.shift_remove(&boid_id);
        get_singleton().bind_mut().unregister_boid_3d(boid_id);
        let flock_id = self.get_id();
        godot_print!("[Flock3D:{flock_id}] boid {boid_id} unregistered");
    }
}

#[godot_api]
impl INode3D for Flock3D {
    fn enter_tree(&mut self) {
        get_singleton().bind_mut().register_flock_3d(self.get_id())
    }

    fn ready(&mut self) {
        self.props = self.properties.bind().clone();
    }

    fn exit_tree(&mut self) {
        get_singleton()
            .bind_mut()
            .unregister_flock_3d(self.get_id())
    }
}

#[godot_api]
impl Flock3D {
    #[func]
    #[inline(always)]
    pub fn get_id(&self) -> i64 {
        self.base().instance_id().to_i64()
    }
}

impl Flock for Flock3D {
    #[inline(always)]
    fn get_flock_properties(&self) -> &FlockProperties {
        &self.props
    }

    #[inline(always)]
    fn get_target_position(&self) -> Option<Vec3> {
        self.target.as_ref().map(|t| to_glam_vec(t.get_position()))
    }

    #[inline(always)]
    fn get_boids_posvel(&self) -> Vec<(Vec3, Vec3)> {
        let boid_count = self.boids.len();
        let mut result = Vec::with_capacity(boid_count);
        result.extend(self.boids.values().map(|b| {
            let b = b.bind();
            (b.get_boid_position(), b.get_boid_velocity())
        }));
        result
    }

    #[inline(always)]
    fn get_boids(&self) -> impl Iterator<Item = (&i64, (Vec3, Vec3, BoidProperties))> {
        self.boids.iter().map(|(id, boid)| {
            let boid = boid.bind();
            (
                id,
                (
                    boid.get_boid_position(),
                    boid.get_boid_velocity(),
                    boid.get_boid_properties().clone(),
                ),
            )
        })
    }
}