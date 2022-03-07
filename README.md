# Commands Send Event

An extension trait for Commands that allows you to send Bevy Events generically.

Supports Bevy 0.6

## Limitations

* Events won't be dispatched immediately, but at the next Stage boundary when the queued commands are applied to the World.

* Bevy events are very lightweight and efficient. ```send_event``` is much more expensive than the regular EventWriter API.

## Usage

Add to your Cargo.toml ```[dependencies]``` section
```
commands_send_event = "0.1"
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
The /examples folder has a complete working example.
