use bevy::prelude::*;
use commands_send_event::CommandsSendEvent;

#[derive(Component)]
struct MyEventA(String);

#[derive(Component)]
struct MyEventB(i32);

fn writer(mut commands: Commands) {
    commands.send_event(MyEventA("Hello, world".into()));
    commands.send_event(MyEventB(42));
}

fn reader(
    mut events_a: EventReader<MyEventA>,
    mut events_b: EventReader<MyEventB>
) {
    for MyEventA(message) in events_a.iter() {
        println!("{message}");
    }

    for MyEventB(message) in events_b.iter() {
        println!("{message}");
    }
}

pub fn main() {
    App::new()
    .add_event::<MyEventA>()
    .add_event::<MyEventB>()
    .add_startup_system(writer)
    .add_system(reader)
    .run();
}