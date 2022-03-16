use bevy::prelude::*;
use commands_send_event::CommandsSendEvent;

#[derive(Component)]
struct MyEvent(String);

fn writer(
    mut event: EventWriter<MyEvent>,
    mut commands: Commands) {
    
    println!("Dispatching events!\n\n");
    
    // will be available to all systems immediately.
    event.send(MyEvent("Sent with EventWriter".to_string()));

    // won't be available until after next stage boundary
    commands.send_event(MyEvent("Sent went CommandsSentEvent::send_event".to_string()));
}

fn reader(
    mut events: EventReader<MyEvent>,
) {
    println!("Messages Recieved:");
    for MyEvent(message) in events.iter() {
        println!("\t{message}");
    }
    println!("End of Messages.\n\n");
}


pub fn main() {
    App::new()
    .add_event::<MyEvent>()
    .add_system(
        (|| println!("* CoreStage::Update *"))
        .label("print stage")
    )
    .add_system(reader.after("print stage").before("writer"))
    .add_system(writer.label("writer"))
    .add_system(reader.after("writer"))
    .add_system_to_stage(
        CoreStage::PostUpdate, 
        (|| println!("* CoreStage::PostUpdate *"))
        .label("print stage")
    )
    .add_system_to_stage(
        CoreStage::PostUpdate, 
        reader.after("print stage")
    )
    .run();
}