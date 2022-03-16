# Commands Send Event

An extension trait for Commands that allows you to send Bevy Events generically.

Supports Bevy 0.6

## Limitations

* Events won't be dispatched immediately, but at the next Stage boundary when the queued commands are applied to the World.

* Bevy events are very lightweight and efficient. ```send_event``` is much more expensive than the regular EventWriter API.

## Usage

Add to your Cargo.toml ```[dependencies]``` section
```
commands_send_event = "0.2"
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

## 0.2 update

Now there is also AnyEventWriter:

```rust
use commands_send_event::AnyEventWriter;

#[derive(Component)]
struct MyEventA(String);

#[derive(Component)]
struct MyEventB(i32);

fn sender(
    mut event_writer: AnyEventWriter,
) {
    event_writer.send(MyEventA("Hello, World"));
    event_writer.send(MyEventB(42));
}
```

AnyEventWriter is a facade over Commands that implememnts SystemParam. It's more ergonomic, but has the same drawbacks with a similar or even worse performance profile.