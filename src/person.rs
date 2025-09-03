use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_time::prelude::*;

#[derive(Component)]
pub struct Person {
    pub age: f32, // years; will be updated by an aging system later
}

impl Person {
    pub fn new() -> Self {
        Self { age: 0.0 }
    }
}
