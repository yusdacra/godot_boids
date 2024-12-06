use godot::prelude::*;

#[derive(Default, Clone, Debug, GodotClass)]
#[class(tool, init, base=Resource)]
/// Properties for a 2D/3D flock.
/// 
/// Note that these properties are used as is and do not get scaled / translated between 2D / 3D units.
/// So copying 2D properties to a 3D flock will *not* behave the same.
pub struct FlockProperties {
    #[export]
    #[init(val = 625.0)]
    /// Distance (squared) to apply seperation force between boids in a flock.
    pub goal_seperation: f32,
    #[export]
    #[init(val = 2500.0)]
    /// Distance (squared) to apply alignment force between boids in a flock.
    pub goal_alignment: f32,
    #[export]
    #[init(val = 2500.0)]
    /// Distance (squared) to apply cohesion force between boids in a flock.
    pub goal_cohesion: f32,
}
