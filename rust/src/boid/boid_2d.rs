use super::*;
use godot::prelude::*;

use crate::{BoidProperties, Flock2D};

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct Boid2D {
    #[export]
    properties: Gd<BoidProperties>,
    props: BoidProperties,
    vel: Vec2,
    flock_id: i64,
    base: Base<Node2D>,
}

#[godot_api]
impl Boid2D {
    #[func]
    #[inline(always)]
    fn get_velocity(&self) -> Vector2 {
        Vector2::new(self.vel.x, self.vel.y)
    }

    #[func]
    #[inline(always)]
    pub fn get_id(&self) -> i64 {
        self.base().instance_id().to_i64()
    }

    #[func]
    #[inline(always)]
    pub fn get_flock_id(&self) -> i64 {
        self.flock_id
    }
}

#[godot_api]
impl INode2D for Boid2D {
    fn enter_tree(&mut self) {
        let Some(mut flock) = self
            .to_gd()
            .get_parent()
            .and_then(|gd| gd.try_cast::<Flock2D>().ok())
        else {
            let boid_id = self.get_id();
            godot_error!("[Boid2D:{boid_id}] boids parent isn't a Flock2D, or has no parent");
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
            .cast::<Flock2D>();
        flock.bind_mut().unregister_boid(self.get_id());
    }
}

impl Boid for Boid2D {
    #[inline(always)]
    fn apply_force(&mut self, force: Vec3) {
        self.vel += force.xy();
        let new_vel = self.vel.clamp_length_max(self.props.max_speed);
        self.vel = new_vel;
        self.base_mut().translate(Vector2::new(new_vel.x, new_vel.y));
    }

    #[inline(always)]
    fn get_boid_position(&self) -> Vec3 {
        let pos = self.base().get_position();
        vec3(pos.x, pos.y, 0.0)
    }

    #[inline(always)]
    fn get_boid_velocity(&self) -> Vec3 {
        vec3(self.vel.x, self.vel.y, 0.0)
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
