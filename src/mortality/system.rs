use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use rand::rngs::StdRng;
use rand::{Rng, RngCore, SeedableRng};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::baby_spawner::system::GameRNG; // your RNG resource
use crate::mortality::events::Death;
use crate::person::Person;

#[derive(Resource, Default)]
pub struct MortalityTick(pub u64);

pub fn hazard(_age: u16) -> f32 {
    0.01
}

/// Every time step, with a given probability, kill an entity.
pub fn apply_mortality_with_rate(
    _rate_per_tick: f64,
) -> impl FnMut(ResMut<GameRNG>, ResMut<MortalityTick>, Query<(Entity, &Person)>, EventWriter<Death>)
       + Send
       + Sync
       + 'static {
    move |mut rng: ResMut<GameRNG>,
          mut tick: ResMut<MortalityTick>,
          people: Query<(Entity, &Person)>,
          mut writer: EventWriter<Death>| {
        let seed = rng.0.next_u64();
        let current_tick = tick.0;
        tick.0 = tick.0.wrapping_add(1);
        for (e, person) in people.iter() {
            let h = hazard(person.age as u16) as f64;
            if h <= 0.0 {
                continue;
            }
            let mut hasher = DefaultHasher::new();
            seed.hash(&mut hasher);
            e.to_bits().hash(&mut hasher);
            current_tick.hash(&mut hasher);
            let sub_seed = hasher.finish();
            let mut r = StdRng::seed_from_u64(sub_seed);
            let u: f32 = r.random();
            if (u as f64) < h {
                writer.write(Death { entity: e });
            }
        }
    }
}

pub fn despawn_on_death(mut cmds: Commands, mut deaths: EventReader<Death>) {
    for d in deaths.read() {
        if let Ok(mut e_cmd) = cmds.get_entity(d.entity) {
            e_cmd.despawn();
        }
    }
}
