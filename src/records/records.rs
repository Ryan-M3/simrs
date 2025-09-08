use crate::baby_spawner::BabyBorn;
use crate::hiring_manager::component::Unemployed;
use crate::mortality::Death;
use crate::person::Person;
use crate::records::RollingMean;
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_time::{Real, Time};

#[derive(Resource, Debug, Clone)]
pub struct Records {
    pub births: usize,
    pub deaths: usize,
    pub birth_rate: RollingMean,
    pub death_rate: RollingMean,
    pub employment_rate: f32,
}

impl Records {
    pub fn population(&self) -> usize {
        self.births.saturating_sub(self.deaths)
    }
}

pub fn record_births(
    time: Res<Time<Real>>,
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
    time: Res<Time<Real>>,
    mut records: ResMut<Records>,
    mut deaths: EventReader<Death>,
) {
    let now = time.elapsed_secs_f64();
    for _ in deaths.read() {
        records.deaths = records.deaths.saturating_add(1);
        records.death_rate.push(now);
    }
}

pub fn record_employment_rate(
    mut records: ResMut<Records>,
    people: Query<Entity, With<Person>>,
    unemployed: Query<Entity, With<Unemployed>>,
) {
    let total = people.iter().count();
    let jobless = unemployed.iter().count();
    if total > 0 {
        let employed = total.saturating_sub(jobless);
        records.employment_rate = employed as f32 / total as f32;
    } else {
        records.employment_rate = 0.0;
    }
}
