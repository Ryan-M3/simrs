use bevy_ecs::prelude::*;

#[derive(Component, Clone, Copy, Debug)]
pub struct Personality {
    pub openness: f64,
    pub conscientiousness: f64,
    pub extraversion: f64,
    pub agreeableness: f64,
    pub neuroticism: f64,
    pub intelligence: f64,
}

impl Personality {
    pub fn as_array(&self) -> [f64; 6] {
        [
            self.openness,
            self.conscientiousness,
            self.extraversion,
            self.agreeableness,
            self.neuroticism,
            self.intelligence,
        ]
    }
}
