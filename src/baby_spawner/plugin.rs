use bevy::prelude::*;

use crate::baby_spawner::{
    config::BabySpawnerConfig, events::BabyBorn, system::spawn_babies, system::GameRNG,
};

pub struct BabySpawnerPlugin;

impl Plugin for BabySpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BabySpawnerConfig>()
            .init_resource::<GameRNG>()
            .add_event::<BabyBorn>()
            .add_systems(Update, spawn_babies);
    }
}
