use bevy::ecs::system::Command;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;

pub struct SendEvent<E> 
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
        SystemState::<EventWriter<E>>::new(world)
        .get_mut(world)
        .send(self.event)
    }
}

pub trait CommandsSendEvent {
    fn send_event<E>(&mut self, event: E)
    where E: 'static + Send + Sync;
}

impl CommandsSendEvent for Commands<'_, '_> {
    fn send_event<E>(&mut self, event: E) 
    where
       E: 'static + Send + Sync
    {   
         self.add(SendEvent { event });
    }
}