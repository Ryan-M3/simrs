use bevy::prelude::*;

pub struct ViewPlugin;

impl Plugin for ViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ui_camera);
    }
}

fn spawn_ui_camera(mut commands: Commands) {
    commands.spawn((Camera2d,));
}
