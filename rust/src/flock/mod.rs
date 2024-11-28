use glam::*;
use godot::prelude::*;

use crate::BoidProperties;

pub mod flock_properties;
pub mod flock_2d;
pub mod flock_3d;

pub use flock_properties::*;
pub use flock_2d::*;
pub use flock_3d::*;

pub trait Flock {
    fn get_flock_properties(&self) -> &FlockProperties;
    fn get_target_position(&self) -> Option<Vec3>;
    fn get_boids(&self) -> impl Iterator<Item = (&InstanceId, (Vec3, Vec3, BoidProperties))>;
    fn get_boids_posvel(&self) -> Vec<(Vec3, Vec3)>;
}
