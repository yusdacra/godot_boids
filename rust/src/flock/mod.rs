use crate::{BoidProperties, FlockProperties};

use glam::*;

pub mod flock_2d;

pub trait Flock {
    fn get_flock_properties(&self) -> &FlockProperties;
    fn get_target_position(&self) -> Option<Vec3>;
    fn get_boids(&self) -> impl Iterator<Item = (&i64, (Vec3, Vec3, BoidProperties))>;
    fn get_boids_posvel(&self) -> Vec<(Vec3, Vec3)>;
}
