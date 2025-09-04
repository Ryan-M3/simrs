use crate::records::Records;
use bevy::prelude::*;
use bevy::time::{Time, Real};

#[derive(Resource)]
pub struct PopulationText(pub Entity);

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
