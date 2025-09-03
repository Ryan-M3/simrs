use crate::baby_spawner::BabyBorn;
use crate::mortality::Death;
use crate::records::RollingMean;
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_time::prelude::*;

#[derive(Resource, Debug, Clone)]
pub struct Records {
    pub births: usize,
    pub deaths: usize,
    pub birth_rate: RollingMean,
    pub death_rate: RollingMean,
}

impl Records {
    pub fn population(&self) -> usize {
        self.births.saturating_sub(self.deaths)
    }
}

pub fn record_births(
    time: Res<Time<Virtual>>,
    mut records: ResMut<Records>,
    mut born: EventReader<BabyBorn>,
) {
    let now = time.elapsed_secs_f64();
    for _ in born.read() {
        records.births = records.births.saturating_add(1);
        records.birth_rate.push(now);
    }
}

pub fn record_deaths(
    time: Res<Time<Virtual>>,
    mut records: ResMut<Records>,
    mut deaths: EventReader<Death>,
) {
    let now = time.elapsed_secs_f64();
    for _ in deaths.read() {
        records.deaths = records.deaths.saturating_add(1);
        records.death_rate.push(now);
    }
}
