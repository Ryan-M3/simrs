use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_time::prelude::*;

#[derive(Resource)]
pub struct BabySpawnerConfig {
    pub per_sec: f64,
}

impl Default for BabySpawnerConfig {
    fn default() -> Self {
        Self { per_sec: 0.0 }
    }
}
