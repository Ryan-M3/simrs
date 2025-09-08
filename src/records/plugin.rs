#[cfg(feature = "graphics")]
use crate::records::ui::{
    spawn_employment_text, spawn_population_text, update_employment_text, update_population_text,
};
use crate::records::Records;
use crate::records::{record_births, record_deaths, record_employment_rate};
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_time::prelude::*;

pub struct RecordsPlugin;

impl Plugin for RecordsPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "graphics")]
        app.add_systems(Startup, (spawn_population_text, spawn_employment_text))
            .add_systems(Update, (update_population_text, update_employment_text));

        app.add_systems(
            Update,
            (record_births, record_deaths, record_employment_rate),
        );
    }
}
