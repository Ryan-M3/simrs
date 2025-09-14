use crate::{gregslist::Gregslist, records::Records};
use bevy::prelude::*;
use bevy::time::{Real, Time};

#[derive(Resource)]
pub struct PopulationText(pub Entity);

#[derive(Resource)]
pub struct EmploymentText(pub Entity);

#[derive(Component)]
pub struct VacancyText;
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
            Text::new("Employment: 0%"),
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

pub fn spawn_vacancy_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        VacancyText,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(72.0),
            left: Val::Px(10.0),
            ..default()
        },
        Text::new("Open roles: 0"),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::WHITE),
    ));
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
    records: Res<Records>,
    mut q_text: Query<&mut Text>,
    text_entity: Res<EmploymentText>,
) {
    if let Ok(mut text) = q_text.get_mut(text_entity.0) {
        text.0 = format!("Employment: {:.0}%", records.employment_rate * 100.0);
    }
}

pub fn update_vacancy_text(
    greg: Res<Gregslist>,
    mut q_text: Query<&mut Text, With<VacancyText>>,
) {
    if let Some(mut text) = q_text.iter_mut().next() {
        text.0 = format!("Open roles: {}", greg.ads.len());
    }
}

pub struct VacancyTextPlugin;

impl Plugin for VacancyTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_vacancy_text)
            .add_systems(Update, update_vacancy_text);
    }
}
