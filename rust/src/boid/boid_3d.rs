use super::*;
use godot::prelude::*;

use crate::{to_glam_vec, BoidProperties, Flock3D};

#[derive(GodotClass)]
#[class(init, base=Node3D)]
/// A 3D boid.
/// Doesn't do anything on it's own, must be a child of a `Flock3D`.
pub struct Boid3D {
    #[export]
    /// The properties of this boid.
    /// Note: this cannot be changed in runtime, aside from removing and readding the node.
    properties: Gd<BoidProperties>,
    props: BoidProperties,
    vel: Vec3,
    flock_id: i64,
    base: Base<Node3D>,
}

#[godot_api]
impl Boid3D {
    #[func]
    #[inline(always)]
    /// Get the current velocity of this boid.
    fn get_velocity(&self) -> Vector3 {
        Vector3::new(self.vel.x, self.vel.y, self.vel.z)
    }

    #[func]
    #[inline(always)]
    /// Set the current velocity of this boid.
    fn set_velocity(&mut self, new_velocity: Vector3) {
        self.vel = to_glam_vec(new_velocity);
    }

    #[func]
    #[inline(always)]
    /// Get the ID of this boid.
    pub fn get_id(&self) -> i64 {
        self.base().instance_id().to_i64()
    }

    #[func]
    #[inline(always)]
    /// Get the flock ID of this boid.
    pub fn get_flock_id(&self) -> i64 {
        self.flock_id
    }
}

#[godot_api]
impl INode3D for Boid3D {
    fn enter_tree(&mut self) {
        let Some(mut flock) = self
            .to_gd()
            .get_parent()
            .and_then(|gd| gd.try_cast::<Flock3D>().ok())
        else {
            let boid_id = self.get_id();
            godot_error!("[Boid3D:{boid_id}] boids parent isn't a Flock3D, or has no parent");
            return;
        };
        let mut flock = flock.bind_mut();
        flock.register_boid(self.get_id());
        self.flock_id = flock.get_id();
    }

    fn ready(&mut self) {
        self.props = self.properties.bind().clone();
    }

    fn exit_tree(&mut self) {
        let mut flock = godot::global::instance_from_id(self.get_flock_id())
            .unwrap()
            .cast::<Flock3D>();
        flock.bind_mut().unregister_boid(self.get_id());
    }
}

impl Boid for Boid3D {
    #[inline(always)]
    fn apply_force(&mut self, force: Vec3) {
        self.vel += force;
        self.vel = self.vel.clamp_length_max(self.props.max_speed);
        let force_to_apply = Vector3::new(self.vel.x, self.vel.y, self.vel.z);
        self.base_mut().translate(force_to_apply);
    }

    #[inline(always)]
    fn get_boid_position(&self) -> Vec3 {
        to_glam_vec(self.base().get_position())
    }

    #[inline(always)]
    fn get_boid_velocity(&self) -> Vec3 {
        self.vel
    }

    #[inline(always)]
    fn get_boid_properties(&self) -> &BoidProperties {
        &self.props
    }

    #[inline(always)]
    fn get_flock_id(&self) -> i64 {
        self.get_flock_id()
    }
}
