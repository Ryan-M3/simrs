use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_time::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand_distr::{Distribution, Poisson};

use crate::baby_spawner::{config::BabySpawnerConfig, events::BabyBorn};
use crate::person::Person;

#[derive(Resource)]
pub struct GameRNG(pub StdRng);

impl FromWorld for GameRNG {
    fn from_world(_: &mut World) -> Self {
        Self(StdRng::seed_from_u64(1))
    }
}

pub fn spawn_babies(
    mut commands: Commands,
    time: Res<Time<Virtual>>,
    cfg: Res<BabySpawnerConfig>,
    mut rng: ResMut<GameRNG>,
    mut writer: EventWriter<BabyBorn>,
) {
    let dt = time.delta_secs_f64();
    let lambda = cfg.per_sec * dt;
    if lambda <= 0.0 {
        return;
    }

    let n = Poisson::new(lambda).unwrap().sample(&mut rng.0) as usize;
    for _ in 0..n {
        let entity = commands.spawn(Person::new()).id();
        writer.write(BabyBorn { entity });
    }
}
