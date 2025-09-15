use crate::mortality::events::Death;
use crate::mortality::system::{despawn_on_death, MortalityTick};
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;

pub struct MortalityPlugin;

impl Plugin for MortalityPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MortalityTick>()
            .add_event::<Death>()
            .add_systems(Update, despawn_on_death);
    }
}
