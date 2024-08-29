use godot::prelude::*;

#[derive(Default, Clone, Debug, GodotClass)]
#[class(tool, init, base=Resource)]
pub struct FlockProperties {
    #[export]
    #[init(val = 625.0)]
    /// squared
    pub goal_seperation: f32,
    #[export]
    #[init(val = 2500.0)]
    /// squared
    pub goal_alignment: f32,
    #[export]
    #[init(val = 2500.0)]
    /// squared
    pub goal_cohesion: f32,
}