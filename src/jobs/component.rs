use bevy::prelude::*;

/// Declarative constraints for a role. Pure data.
#[derive(Clone, Debug)]
pub enum Constraint {
    AgeLessThan(u8),
    AgeAtLeast(u8),
    // add more later (HasTrait(...), LivesIn(...), etc.)
}

/// A role *definition* inside a Job: seats + constraints.
#[derive(Clone, Debug)]
pub struct RoleSpec {
    pub min: u32,
    pub max: u32,
    pub constraints: Vec<Constraint>,
}

/// A Job is a list of role definitions paired with their members.
#[derive(Component, Default)]
pub struct Job {
    pub roles: Vec<(RoleSpec, Vec<Entity>)>, // (spec, members)
}

/// Fluent builder so you can write ontology-like lines in `main`.
pub struct JobBuilder {
    roles: Vec<(RoleSpec, Vec<Entity>)>,
    current: Option<usize>, // index of the most recently added role
}

impl Job {
    pub fn builder() -> JobBuilder {
        JobBuilder {
            roles: Vec::new(),
            current: None,
        }
    }
}

impl JobBuilder {
    /// Start a new role with seat bounds.
    pub fn add_role(mut self, min: u32, max: u32) -> Self {
        self.roles.push((
            RoleSpec {
                min,
                max,
                constraints: Vec::new(),
            },
            Vec::new(),
        ));
        self.current = Some(self.roles.len() - 1);
        self
    }

    /// Attach a constraint to the most recently added role.
    pub fn with_constraint(mut self, c: Constraint) -> Self {
        if let Some(i) = self.current {
            self.roles[i].0.constraints.push(c); // .0 = RoleSpec
        }
        self
    }

    // Sugar: ontologically readable methods (you can add more later)
    pub fn age_lt(self, n: u8) -> Self {
        self.with_constraint(Constraint::AgeLessThan(n))
    }
    pub fn age_gte(self, n: u8) -> Self {
        self.with_constraint(Constraint::AgeAtLeast(n))
    }

    /// Finalize into a Job component.
    pub fn build(self) -> Job {
        Job { roles: self.roles }
    }
}

pub fn vacancy(role: &(RoleSpec, Vec<Entity>)) -> u32 {
    role.0.min.saturating_sub(role.1.len() as u32)
}
