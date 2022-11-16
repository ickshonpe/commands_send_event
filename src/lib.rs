use bevy::ecs::system::Command;
use bevy::prelude::*;

struct SendEvent<E>
where
    E: 'static + Send + Sync,
{
    event: E,
}

impl<E> Command for SendEvent<E>
where
    E: 'static + Send + Sync,
{
    fn write(self, world: &mut World) {
        world.send_event(self.event)
    }
}

pub trait CommandsSendEvent {
    fn send_event<E>(&mut self, event: E)
    where
        E: 'static + Send + Sync;
}

impl CommandsSendEvent for Commands<'_, '_> {
    /// Queue an event to be dispatched
    /// at the next stage boundary.
    fn send_event<E>(&mut self, event: E)
    where
        E: 'static + Send + Sync,
    {
        self.add(SendEvent { event });
    }
}
