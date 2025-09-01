use crate::mortality::events::Death;
use crate::mortality::system::despawn_on_death;
use bevy::prelude::*;

pub struct MortalityPlugin;

impl Plugin for MortalityPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Death>()
            .add_systems(Update, despawn_on_death);
    }
}
