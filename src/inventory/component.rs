use bevy::prelude::*;

#[derive(Component, Debug, Default)]
pub struct Inventory {
    pub items: Vec<Entity>,
}

/// Add `item` to `container` if it's not already present.
pub fn inv_add(mut q_inv: Query<&mut Inventory>, container: Entity, item: Entity) {
    if let Ok(mut inv) = q_inv.get_mut(container) {
        if !inv.items.iter().any(|&e| e == item) {
            inv.items.push(item);
        }
    }
}

/// Remove `item` from `container`. Returns true if removed.
pub fn inv_remove(mut q_inv: Query<&mut Inventory>, container: Entity, item: Entity) -> bool {
    if let Ok(mut inv) = q_inv.get_mut(container) {
        if let Some(i) = inv.items.iter().position(|&e| e == item) {
            inv.items.swap_remove(i);
            return true;
        }
    }
    false
}

/// Move `item` from `from` (if provided) to `to`. Skips duplicate adds.
pub fn inv_move(mut q_inv: Query<&mut Inventory>, from: Option<Entity>, to: Entity, item: Entity) {
    // remove from source if given
    if let Some(src) = from {
        if let Ok(mut inv) = q_inv.get_mut(src) {
            if let Some(i) = inv.items.iter().position(|&e| e == item) {
                inv.items.swap_remove(i);
            }
        }
    }
    // add to destination if not already there
    if let Ok(mut inv) = q_inv.get_mut(to) {
        if !inv.items.iter().any(|&e| e == item) {
            inv.items.push(item);
        }
    }
}

/// True if `item` is in `container`.
pub fn inv_contains(q_inv: &Query<&Inventory>, container: Entity, item: Entity) -> bool {
    q_inv
        .get(container)
        .map(|inv| inv.items.iter().any(|&e| e == item))
        .unwrap_or(false)
}

/// Copy of all items in `container`.
pub fn inv_list(q_inv: &Query<&Inventory>, container: Entity) -> Vec<Entity> {
    q_inv
        .get(container)
        .map(|inv| inv.items.clone())
        .unwrap_or_default()
}
