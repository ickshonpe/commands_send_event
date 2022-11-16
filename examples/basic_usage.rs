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

fn writer(mut commands: Commands) {
    println!("* dispatching messages with Commands::send_event *");
    // send_event adds events to the Commands CommandQueue,
    // to be dispatched at the next stage boundary
    // when the CommandQueue is processed.
    commands.send_event(MyEventA("Hello, world".into()));
    commands.send_event(MyEventB(42));
}

fn reader(mut events_a: EventReader<MyEventA>, mut events_b: EventReader<MyEventB>) {
    println!("* reading messages *");
    // EventReaders work as normal except, again, remember that
    // events aren't dispatched until the next stage boundary.
    // If both the sending and recieving systems are in CoreStage::Update,
    // this will mean that `EventReader`s won't recieve events dispatched
    // by `commands.send_event` until the next frame.

    for MyEventA(message) in events_a.iter() {
        println!("\t {message}");
    }

    for MyEventB(message) in events_b.iter() {
        println!("\t {message}");
    }

    println!();
}

pub fn main() {
    App::new()
        // We don't include any plugins as we only
        // want the app to run each system once
        // and then exit.
        // Events have to registered before use with `Commands::send_event`, just as with `EventWriter`.
        .add_event::<MyEventA>()
        .add_event::<MyEventB>()
        // Add our systems to seperate stages
        .add_startup_system(writer) // `writer` system added to `StartupStage`
        .add_system(reader) // `reader` system added to `CoreStage::Update`
        // writer and reader are added to different stages because
        // events dispatched by commands.sent_event will not be
        // available to any EventReader until after the Commands
        // CommandQueue is processed at the next Stage boundary.
        .run();
}
