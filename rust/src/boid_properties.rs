use godot::prelude::*;

#[derive(Default, Clone, Debug, GodotClass)]
#[class(tool, init, base=Resource)]
pub struct BoidProperties {
    #[export]
    #[init(val = 4.0)]
    pub max_speed: f32,
    #[export]
    #[init(val = 1.0)]
    pub max_force: f32,
    #[export]
    #[init(val = 1.5)]
    pub alignment: f32,
    #[export]
    #[init(val = 1.0)]
    pub cohesion: f32,
    #[export]
    #[init(val = 1.2)]
    pub seperation: f32,
    #[export]
    #[init(val = 0.8)]
    pub targeting: f32,
}