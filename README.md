# pubsub-bus

[![GitHub Release](https://img.shields.io/github/v/release/an-dr/pubsub-bus)](https://github.com/an-dr/pubsub-bus/releases)

Thread-safe one-to-many event system. Simple and easy to use. It just works (hopefully).

- [pubsub-bus](#pubsub-bus)
    - [⚙️ What it does (Without words)](#️-what-it-does-without-words)
    - [🚀 Quick Start](#-quick-start)
        - [1. Add the dependency to your `Cargo.toml`](#1-add-the-dependency-to-your-cargotoml)
        - [2. Create your events and a bus](#2-create-your-events-and-a-bus)
        - [3. Implement the Subscriber trait for your struct and subscribe it to the bus](#3-implement-the-subscriber-trait-for-your-struct-and-subscribe-it-to-the-bus)
        - [4. Create a Publisher and pass the bus to it](#4-create-a-publisher-and-pass-the-bus-to-it)
        - [5. Send events](#5-send-events)
    - [📖 Examples](#-examples)

## ⚙️ What it does (Without words)

![Publishing](docs/README/structure.drawio.svg)

## 🚀 Quick Start

### 1. Add the dependency to your `Cargo.toml`

```toml
pubsub-bus = "1.1.0"
```

### 2. Create your events and a bus

```rust
pub enum Commands {
    Atack { player_id: u32 },
    Move { player_id: u32, x: f32, y: f32 },
}

let bus: Arc<EventBus<Commands>> = Arc::new(EventBus::new());;
```

### 3. Implement the Subscriber trait for your struct and subscribe it to the bus

```rust
impl Subscriber<Commands> for Player {
    fn on_event(&mut self, event: &Event<Commands>) {
        // Handle the event
    }
}

...

let player1 = Arc::new(Mutex::new(Player { id: 1 }));
bus.add_subscriber(player1);
```

### 4. Create a Publisher and pass the bus to it

```rust
pub struct Input {
    emitter: EventEmitter<Commands>,
}

impl Publisher<Commands> for Input {
    fn get_mut_emitter(&mut self) -> &mut EventEmitter<Commands> {
        &mut self.emitter
    }
}

...

let mut  input = Input::new();
bus.add_publisher(&mut input);

```

### 5. Send events

```rust
impl Input {
    pub fn send_move(&self, player_id: u32, x: f32, y: f32) {
        self.emitter.publish(Commands::Move { player_id, x, y });
    }
}
```

## 📖 Examples

The following example demonstrates how to exchange events between players and an input system.

```rust
fn main() {
    // Create a bus
    let bus: Arc<Mutex<EventBus<Commands>>> = Arc::new(Mutex::new(EventBus::new()));

    // Create players and subscribe them to the bus
    let player1 = Arc::new(Mutex::new(Player { id: 1 }));
    let player2 = Arc::new(Mutex::new(Player { id: 2 }));
    let mut  input = Input::new();

    bus.add_subscriber(player1);
    bus.add_subscriber(player2);
    bus.add_publisher(&mut input);

    // Send some events
    input.send_move(1, 1.0, 2.0);
    input.send_atack(2);
}
```

For the full example, see the [examples/basic_game_events](examples/basic_game_events) directory.

💡 NOTE: check out my other crate to work with `Arc<Mutex<T>>` more conveniently: <https://crates.io/crates/shared-type>
