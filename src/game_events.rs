use bevy_ecs::prelude::*;

pub fn game_event_system<E, T, R>(
    mut trigger: T,
    mut resolve: R,
) -> impl FnMut(&mut World) + Send + Sync + 'static
where
    E: 'static,
    T: FnMut(&World, &mut Vec<E>) + Send + Sync + 'static,
    R: FnMut(&mut World, E) + Send + Sync + 'static,
{
    move |world: &mut World| {
        let mut events = Vec::<E>::new();
        trigger(world, &mut events);
        for ev in events.drain(..) {
            resolve(world, ev);
        }
    }
}
