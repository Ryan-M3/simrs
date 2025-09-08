use crate::records::Records;
use bevy::prelude::*;
use bevy::time::{Real, Time};

use crate::hiring_manager::component::Unemployed;
use crate::person::Person;

#[derive(Resource)]
pub struct PopulationText(pub Entity);

#[derive(Resource)]
pub struct EmploymentText(pub Entity);
pub fn spawn_population_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let e = commands
        .spawn((
            // the UI node (so it renders on the screen)
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(8.0),
                left: Val::Px(8.0),
                ..default()
            },
            // the text itself (string lives here)
            Text::new("Population: 0\nBirth rate: 0.0000\nDeath rate: 0.0000"),
            // font & size
            TextFont {
                font: asset_server.load("fonts/SourceCodePro-Regular.otf"),
                font_size: 30.0,
                ..default()
            },
            // color
            TextColor(Color::WHITE),
        ))
        .id();

    commands.insert_resource(PopulationText(e));
}

pub fn spawn_employment_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let e = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(40.0),
                left: Val::Px(10.0),
                ..default()
            },
            Text::new("Employed: 0"),
            TextFont {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ))
        .id();
    commands.insert_resource(EmploymentText(e));
}

pub fn update_population_text(
    time: Res<Time<Real>>,
    mut records: ResMut<Records>,
    mut q_text: Query<&mut Text>,
    text_entity: Res<PopulationText>,
) {
    let now = time.elapsed_secs_f64();

    // maintain rolling windows before reading avgs
    records.birth_rate.prune(now);
    records.death_rate.prune(now);

    // build once; don't overwrite three times
    let s = format!(
        "Population: {}\nBirth rate: {:.4}\nDeath rate: {:.4}",
        records.population(),
        records.birth_rate.avg(),
        records.death_rate.avg(),
    );

    if let Ok(mut text) = q_text.get_mut(text_entity.0) {
        // Text is a tuple struct; assign its String
        text.0 = s;
    }
}

pub fn update_employment_text(
    mut q_text: Query<&mut Text>,
    text_entity: Res<EmploymentText>,
    people: Query<Entity, With<Person>>,
    unemployed: Query<Entity, With<Unemployed>>,
) {
    if let Ok(mut text) = q_text.get_mut(text_entity.0) {
        let total = people.iter().count();
        let jobless = unemployed.iter().count();
        let employed = total.saturating_sub(jobless);
        text.0 = format!("Employed: {}", employed);
    }
}
