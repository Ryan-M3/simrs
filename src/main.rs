#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

#[cfg(feature = "graphics")]
use bevy::prelude::DefaultPlugins;
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_time::{Real, Time};

mod baby_spawner;
mod gregslist;
mod hiring_manager;
mod graph;
mod game_events;
mod inventory;
mod jobs;
mod mortality;
mod person;
mod personality;
mod records;
#[cfg(feature = "graphics")]
mod view;

use crate::baby_spawner::{BabySpawnerConfig, BabySpawnerPlugin};
use crate::inventory::InventoryPlugin;
use crate::mortality::system::apply_mortality_with_rate;
use crate::records::{rolling_mean::RollingMean, Records, VacancyTextPlugin};
use jobs::Job;

const SEC: f64 = 1.0;
const MIN: f64 = 60.0 * SEC;
const HR: f64 = 60.0 * MIN;
const DAY: f64 = 24.0 * HR;
const YR: f64 = 365.0 * DAY;
const SPEED: f64 = DAY; // 1 sec realtime = 1 day gametime

const BIRTHS_PER_YEAR: f64 = 1_000.0;
const AVERAGE_LIFESPAN_YEARS: f64 = 65.0;

fn debug_years(time: Res<Time<Real>>) {
    let weeks = (time.elapsed_secs_f64() / (DAY * 7.0) * SPEED) as u64;
    static mut LAST: u64 = 0;
    unsafe {
        if weeks > LAST {
            println!("{} weeks have passed", weeks);
            LAST = weeks;
        }
    }
}

fn spawn_jobs(mut commands: Commands) {
    let school = Job::builder()
        .add_role(20, 200)
        .age_lt(18) // students
        .add_role(1, 10)
        .age_gte(18) // teachers
        .build();

    commands.spawn(school);
}

fn main() {
    let mut app = App::new();
    #[cfg(feature = "graphics")]
    {
        app.add_plugins(DefaultPlugins)
            .add_plugins(view::ViewPlugin);
    }
    #[cfg(not(feature = "graphics"))]
    {
        app.add_plugins(bevy_time::TimePlugin::default());
    }
    app.add_plugins(BabySpawnerPlugin)
        .add_plugins(records::RecordsPlugin)
        .add_plugins(mortality::MortalityPlugin)
        .add_plugins(jobs::JobsPlugin)
        .add_plugins(gregslist::GregslistPlugin::new(60.0))
        .add_plugins(hiring_manager::HiringManagerPlugin::new(8));
    #[cfg(feature = "graphics")]
    app.add_plugins(VacancyTextPlugin);
    app.add_systems(Startup, spawn_jobs)
        //.add_systems(Startup, |mut time: ResMut<Time<Real>>| {
        //    time.set_relative_speed(DAY as f32);
        //})
        .insert_resource(BabySpawnerConfig {
            per_sec: BIRTHS_PER_YEAR / YR * SPEED,
        })
        .insert_resource(Records {
            births: 0,
            deaths: 0,
            birth_rate: RollingMean::new(DAY),
            death_rate: RollingMean::new(DAY),
            employment_rate: 0.0,
        })
        .add_systems(Update, {
            let deaths_per_sec_per_person = SPEED / (AVERAGE_LIFESPAN_YEARS * YR);
            apply_mortality_with_rate(deaths_per_sec_per_person)
        })
        .add_systems(Update, debug_years)
        .run();
}
