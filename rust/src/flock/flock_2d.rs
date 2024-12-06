use glam::*;
use godot::prelude::*;

use crate::{get_singleton, Boid, Boid2D, BoidProperties, FlockProperties, FxIndexMap};

use super::Flock;

#[derive(GodotClass)]
#[class(init, base=Node2D)]
/// A flock that holds 2D boids.
/// Adding `Boid2D` as a child of this node will register the boid.
pub struct Flock2D {
    #[export]
    /// Properties of this flock.
    /// Note: this cannot be changed in runtime, aside from removing and readding the node.
    properties: Option<Gd<FlockProperties>>,
    props: FlockProperties,
    #[export]
    /// A target node for the flock to follow.
    target: Option<Gd<Node2D>>,
    pub boids: FxIndexMap<InstanceId, Gd<Boid2D>>,
    base: Base<Node2D>,
}

impl Flock2D {
    pub fn register_boid(&mut self, boid_id: InstanceId) {
        let boid: Gd<Boid2D> = Gd::from_instance_id(boid_id);
        self.boids.insert(boid_id, boid.clone());
        get_singleton().bind_mut().register_boid_2d(boid_id, boid);
        let flock_id = self.get_id();
        godot_print!("[Flock2D:{flock_id}] boid {boid_id} registered");
    }

    pub fn unregister_boid(&mut self, boid_id: InstanceId) {
        self.boids.shift_remove(&boid_id);
        get_singleton().bind_mut().unregister_boid_2d(boid_id);
        let flock_id = self.get_id();
        godot_print!("[Flock2D:{flock_id}] boid {boid_id} unregistered");
    }
}

#[godot_api]
impl INode2D for Flock2D {
    fn enter_tree(&mut self) {
        get_singleton().bind_mut().register_flock_2d(self.get_id())
    }

    fn ready(&mut self) {
        if let Some(props) = self.properties.as_ref() {
            self.props = props.bind().clone();
        }
    }

    fn exit_tree(&mut self) {
        get_singleton()
            .bind_mut()
            .unregister_flock_2d(self.get_id())
    }
}

#[godot_api]
impl Flock2D {
    #[func]
    #[inline(always)]
    /// Retrieve the ID of this flock.
    pub fn get_id(&self) -> InstanceId {
        self.base().instance_id()
    }
}

impl Flock for Flock2D {
    #[inline(always)]
    fn get_flock_properties(&self) -> &FlockProperties {
        &self.props
    }

    #[inline(always)]
    fn get_target_position(&self) -> Option<Vec3> {
        self.target.as_ref().map(|t| {
            let pos = t.get_position();
            vec3(pos.x, pos.y, 0.0)
        })
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
    fn get_boids(&self) -> impl Iterator<Item = (&InstanceId, (Vec3, Vec3, BoidProperties))> {
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
