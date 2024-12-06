use super::*;

use crate::{BoidProperties, Flock2D};

#[derive(GodotClass)]
#[class(init, base=Node2D)]
/// A 2D boid.
/// Doesn't do anything on it's own, must be a child of a `Flock2D`.
pub struct Boid2D {
    #[export]
    /// The properties of this boid.
    /// Note: this cannot be changed in runtime, aside from removing and readding the node.
    properties: Option<Gd<BoidProperties>>,
    props: BoidProperties,
    vel: Vec2,
    flock_id: Option<InstanceId>,
    base: Base<Node2D>,
}

#[godot_api]
impl Boid2D {
    #[func]
    #[inline(always)]
    /// Get the current velocity of this boid.
    fn get_velocity(&self) -> Vector2 {
        Vector2::new(self.vel.x, self.vel.y)
    }

    #[func]
    #[inline(always)]
    /// Set the current velocity of this boid.
    fn set_velocity(&mut self, new_velocity: Vector2) {
        self.vel.x = new_velocity.x;
        self.vel.y = new_velocity.y;
    }

    #[func]
    #[inline(always)]
    /// Get the ID of this boid.
    pub fn get_id(&self) -> InstanceId {
        self.base().instance_id()
    }

    #[func]
    #[inline(always)]
    /// Get the flock ID of this boid.
    pub fn get_flock_id(&self) -> InstanceId {
        self.flock_id.expect("no flock id set... this is a bug!")
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
        self.flock_id = Some(flock.get_id());
    }

    fn ready(&mut self) {
        if let Some(props) = self.properties.as_ref() {
            self.props = props.bind().clone();
        }
    }

    fn exit_tree(&mut self) {
        let mut flock: Gd<Flock2D> =
            Gd::from_instance_id(self.get_flock_id());
        flock.bind_mut().unregister_boid(self.get_id());
    }
}

impl Boid for Boid2D {
    #[inline(always)]
    fn apply_force(&mut self, force: Vec3) {
        self.vel += force.xy();
        self.vel = self.vel.clamp_length_max(self.props.max_speed);
        let force_to_apply = Vector2::new(self.vel.x, self.vel.y);
        self.base_mut().translate(force_to_apply);
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
    fn get_flock_id(&self) -> InstanceId {
        self.get_flock_id()
    }
}
