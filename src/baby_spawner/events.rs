use bevy::prelude::*;

#[derive(Event, Debug, Clone, Copy)]
pub struct BabyBorn {
    pub entity: Entity,
}
