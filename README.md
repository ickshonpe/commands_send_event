# Commands Send Event

An extension trait for Commands that allows you to send events from a system without having to retrieve a typed EventWriter SystemParam.

* version 0.9 supports Bevy 0.9
* version 0.5 supports Bevy 0.8
* version 0.3 & 0.4 support Bevy 0.7
* versions <0.3 support Bevy 0.6

## Limitations

* Events won't be dispatched immediately, but at the next Stage boundary when the queued commands are applied to the World.

#

## Usage
Add to your project with the command
```
cargo add commands_send_event
```

or add directly add the dependency to your Cargo.toml
```toml
[dependencies.commands_send_event]
version = "0.6"
```

then the ```send_event``` method is available on Commands:

```rust
use commands_send_event::CommandsSendEvent;

#[derive(Component)]
struct MyEventA(String);

#[derive(Component)]
struct MyEventB(i32);

fn sender(
    mut commands: Commands
) {
    commands.send_event(MyEventA("Hello, World"));
    commands.send_event(MyEventB(42));
}
```
The `/examples` folder has two examples you can run with:
```
cargo run --example basic_usage
cargo run --example schedule
```

AnyEventWriter is a facade over Commands that implements SystemParam. 

#
## Note

This crate is a bit redundant now since Bevy 0.8 as sending events using ```World``` is very easy. With ```commands.add``` you can queue a closure to dispatch an event like so:
```rust
commands.add(|world: &mut World| 
    world.send_event(MyEvent)
);
```