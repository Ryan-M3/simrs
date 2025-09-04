use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_time::{Time, Real};
use rand::Rng;

use crate::baby_spawner::system::GameRNG; // your RNG resource
use crate::mortality::events::Death;
use crate::person::Person;

/// Every time step, with a given probability, kill an entity.
pub fn apply_mortality_with_rate(
    rate_per_sec_per_person: f64,
) -> impl FnMut(Res<Time<Real>>, ResMut<GameRNG>, Query<Entity, With<Person>>, EventWriter<Death>)
       + Send
       + Sync
       + 'static {
    move |time: Res<Time<Real>>,
          mut rng: ResMut<GameRNG>,
          people: Query<Entity, With<Person>>,
          mut writer: EventWriter<Death>| {
        let p = rate_per_sec_per_person * time.delta_secs_f64(); // per-frame death prob
        if p <= 0.0 {
            return;
        }
        for e in people.iter() {
            if rng.0.random_bool(p) {
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
