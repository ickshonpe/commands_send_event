// Demonstrates basic syntax
// Should output the following to the console:
// ```
// Hello, World
// 42
// ```

use bevy::prelude::*;
use commands_send_event::*;

struct MyEventA(String);

struct MyEventB(i32);

fn writer(
    mut commands: Commands,
    mut event_writer: AnyEventWriter
) {
    // send_event adds events to the Commands CommandQueue,
    // to be dispatched at the next stage boundary
    // when the CommandQueue is processed.
    commands.send_event(MyEventA("Hello, world".into()));
    commands.send_event(MyEventB(42));

    event_writer.send(MyEventA("Just any other event.".into()));
}

fn reader(
    mut events_a: EventReader<MyEventA>,
    mut events_b: EventReader<MyEventB>
) {
    // EventReaders work as normal.

    for MyEventA(message) in events_a.iter() {
        println!("{message}");
    }

    for MyEventB(message) in events_b.iter() {
        println!("{message}");
    }
}

pub fn main() {
    App::new()                      

    // We don't include any plugins as we only
    // want the app to run each system once
    // and then exit.

    .add_event::<MyEventA>()
    .add_event::<MyEventB>()
    .add_startup_system(writer)     // Added to StartupStage  
    .add_system(reader)             // Added to CoreStage::Update

    // writer and reader are added to different stages because
    // events dispatched by commands.sent_event will not be 
    // available to any EventReader until after the Commands 
    // CommandQueue is processed at the next Stage boundary.

    .run();                         
}