use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_time::prelude::*;

#[derive(Event, Debug, Clone, Copy)]
pub struct BabyBorn {
    pub entity: Entity,
}
