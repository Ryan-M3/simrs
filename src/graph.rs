use bevy_ecs::prelude::*;
use std::collections::HashMap;

#[derive(Component, Debug)]
pub struct Graph<V> {
    pub edges: HashMap<Entity, V>,
}

impl<V> Default for Graph<V> {
    fn default() -> Self {
        Self { edges: HashMap::new() }
    }
}

impl<V> Graph<V> {
    pub fn set(&mut self, to: Entity, value: V) {
        self.edges.insert(to, value);
    }

    pub fn get(&self, to: &Entity) -> Option<&V> {
        self.edges.get(to)
    }

    pub fn get_mut(&mut self, to: &Entity) -> Option<&mut V> {
        self.edges.get_mut(to)
    }

    pub fn ensure_with<F: FnOnce() -> V>(&mut self, to: Entity, make: F) -> &mut V {
        self.edges.entry(to).or_insert_with(make)
    }

    pub fn upsert_with<U: FnOnce() -> V>(
        &mut self,
        to: Entity,
        init: U,
        update: impl FnOnce(&mut V),
    ) {
        update(self.edges.entry(to).or_insert_with(init));
    }

    pub fn remove(&mut self, to: &Entity) -> Option<V> {
        self.edges.remove(to)
    }
}

