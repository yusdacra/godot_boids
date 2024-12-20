use godot::prelude::*;

#[derive(Default, Clone, Debug, GodotClass)]
#[class(init, base=Resource)]
/// Properties for a 2D/3D boid.
/// 
/// Note that these properties are used as is and do not get scaled / translated between 2D / 3D units.
/// So copying 2D properties to a 3D boid will *not* behave the same.
pub struct BoidProperties {
    #[export]
    #[init(val = 4.0)]
    /// Max speed of this boid.
    pub max_speed: f32,
    #[export]
    #[init(val = 1.0)]
    /// Max force that will be applied to this boid at once.
    pub max_force: f32,
    #[export]
    #[init(val = 1.5)]
    /// How much to align with other boids.
    pub alignment: f32,
    #[export]
    #[init(val = 1.0)]
    /// How much to cohere to other boids.
    pub cohesion: f32,
    #[export]
    #[init(val = 1.2)]
    /// How much to seperate from other boids.
    pub seperation: f32,
    #[export]
    #[init(val = 0.8)]
    /// How much to follow a flock target (if there is one).
    pub targeting: f32,
}
