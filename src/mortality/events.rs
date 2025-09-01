use bevy::prelude::*;

#[derive(Event, Debug, Clone, Copy)]
pub struct Death {
    pub entity: Entity,
}
