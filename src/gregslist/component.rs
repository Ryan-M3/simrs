use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use std::collections::HashSet;

#[derive(Resource, Default)]
pub struct Gregslist {
    pub ads: Vec<Advert>,
    pub index: HashSet<(Entity, usize)>, // (job, role_index) -> existence guard
}

#[derive(Clone)]
pub struct Advert {
    pub job: Entity,
    pub role_index: usize,
    pub date_posted: f32,
}

#[derive(Resource)]
pub struct GregslistConfig {
    pub expiry_secs: f32,
}
#[derive(Event)]
pub struct VacancyDirty {
    pub job: Entity,
}
