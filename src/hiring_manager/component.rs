use bevy_app::prelude::*;
use bevy_ecs::prelude::*;

// Minimal applicant traits used by constraints
#[derive(Component)]
pub struct Age {
    pub years: u8,
}

#[derive(Component)]
pub struct Unemployed;

// Applications are queued here for evaluation each frame (drained after use)
#[derive(Resource, Default)]
pub struct ApplicationInbox {
    pub resumes: Vec<Resume>,
}

pub struct Resume {
    pub applicant: Entity,
    pub job: Entity,
    pub role_index: usize,
}

// Hiring behavior knobs (kept minimal)
#[derive(Resource)]
pub struct HiringConfig {
    pub max_hires_per_role_per_cycle: u32,
}

