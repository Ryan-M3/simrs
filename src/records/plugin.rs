use crate::records::ui::{spawn_population_text, update_population_text};
use crate::records::Records;
use crate::records::{record_births, record_deaths};
use bevy::prelude::*;

pub struct RecordsPlugin;

impl Plugin for RecordsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_population_text).add_systems(
            Update,
            (record_births, record_deaths, update_population_text),
        );
    }
}
