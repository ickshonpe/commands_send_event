use bevy::ecs::event::Events;
use bevy::ecs::system::Command;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

#[derive(SystemParam)]
pub struct AnyEventWriter<'w, 's> {
    commands: Commands<'w, 's>
}

impl AnyEventWriter<'_, '_> {
    /// Queue an event to be dispatched 
    /// at the next stage boundary.
    pub fn send<E>(&mut self, event: E) 
    where
        E: 'static + Send + Sync
    {
        self.commands.send_event(event);
    }
}

struct SendEvent<E> 
where
    E: 'static + Send + Sync
{
    event: E
}

impl <E> Command for SendEvent<E> 
where
    E: 'static + Send + Sync
{
    fn write(self, world: &mut World) {
        world.resource_mut::<Events<E>>().send(self.event)
    }
}

pub trait CommandsSendEvent {
    fn send_event<E>(&mut self, event: E)
    where E: 'static + Send + Sync;
}

impl CommandsSendEvent for Commands<'_, '_> {
    /// Queue an event to be dispatched 
    /// at the next stage boundary.
    fn send_event<E>(&mut self, event: E) 
    where
       E: 'static + Send + Sync
    {   
         self.add(SendEvent { event });
    }
}