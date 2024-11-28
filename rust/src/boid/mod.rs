use std::ops::Sub;
use std::sync::Arc;

use glam::*;
use godot::prelude::*;
use rayon::prelude::*;

use crate::FlockProperties;

pub mod boid_2d;
pub mod boid_3d;
pub mod boid_properties;

pub use boid_2d::*;
pub use boid_3d::*;
pub use boid_properties::*;

pub trait Boid {
    fn apply_force(&mut self, force: Vec3);
    fn get_boid_position(&self) -> Vec3;
    fn get_boid_velocity(&self) -> Vec3;
    fn get_boid_properties(&self) -> &BoidProperties;

    fn get_flock_id(&self) -> InstanceId;
}

struct CalcArgs {
    steer: Vec3,
    align: Vec3,
    cohere: Vec3,

    steer_count: i32,
    align_count: i32,
    cohere_count: i32,
}

impl CalcArgs {
    const fn identity() -> Self {
        Self {
            steer: Vec3::ZERO,
            align: Vec3::ZERO,
            cohere: Vec3::ZERO,
            steer_count: 0,
            align_count: 0,
            cohere_count: 0,
        }
    }
}

pub fn calculate_boid(
    boid_pos: Vec3,
    boid_vel: Vec3,
    boid_props: BoidProperties,
    flock_props: FlockProperties,
    other_boids: Arc<Vec<(Vec3, Vec3)>>,
    target_position: Option<Vec3>,
) -> Vec3 {
    //godot::godot_print!("[Boids] executing from thread {:?}", rayon::current_thread_index());

    let mut calced = other_boids
        .par_iter()
        .fold(CalcArgs::identity, |mut acc, (aboid_pos, aboid_vel)| {
            let dist = boid_pos.distance_squared(*aboid_pos);
            if dist > f32::EPSILON {
                if dist < flock_props.goal_seperation {
                    let diff = (boid_pos.sub(*aboid_pos)).normalize() / f32::sqrt(dist);
                    acc.steer += diff;
                    acc.steer_count += 1;
                }
                if dist < flock_props.goal_alignment {
                    acc.align += *aboid_vel;
                    acc.align_count += 1;
                }
                if dist < flock_props.goal_cohesion {
                    acc.cohere += *aboid_pos;
                    acc.cohere_count += 1;
                }
            }
            acc
        })
        .reduce(CalcArgs::identity, |mut left, right| {
            left.steer += right.steer;
            left.align += right.align;
            left.cohere += right.cohere;
            left.steer_count += right.steer_count;
            left.align_count += right.align_count;
            left.cohere_count += right.cohere_count;
            left
        });

    if calced.steer_count > 0 {
        calced.steer /= calced.steer_count as f32;
    }
    if calced.align_count > 0 {
        calced.align /= calced.align_count as f32;
    }
    if calced.cohere_count > 0 {
        calced.cohere /= calced.cohere_count as f32;
        calced.cohere -= boid_pos;
    }

    let max_speed = boid_props.max_speed;
    let max_force = boid_props.max_force;
    if calced.align.length_squared() > 0.0 {
        calced.align =
            (calced.align.normalize() * max_speed - boid_vel).clamp_length_max(max_force);
    }
    if calced.steer.length_squared() > 0.0 {
        calced.steer =
            (calced.steer.normalize() * max_speed - boid_vel).clamp_length_max(max_force);
    }
    if calced.cohere.length_squared() > 0.0 {
        calced.cohere =
            (calced.cohere.normalize() * max_speed - boid_vel).clamp_length_max(max_force);
    }

    let target = target_position.map_or(Vec3::ZERO, |target_position| {
        ((target_position - boid_pos) - boid_vel).clamp_length_max(max_force)
    });

    let steer_force = calced.steer * boid_props.seperation;
    let align_force = calced.align * boid_props.alignment;
    let cohere_force = calced.cohere * boid_props.cohesion;
    let target_force = target * boid_props.targeting;
    let force = steer_force + align_force + cohere_force + target_force;

    force
}
