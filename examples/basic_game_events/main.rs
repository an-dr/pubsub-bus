mod commands;
mod input;
mod player;
mod topic_ids;

use commands::Commands;
use input::Input;
use player::Player;
use pubsub_bus::*;
use topic_ids::TopicIds;

fn main() {
    // Create a bus
    let bus: EventBus<Commands, TopicIds> = EventBus::new();

    // Create players, input, and attach to the bus
    let player1 = Player { id: 1 };
    let player2 = Player { id: 2 };
    let mut input = Input::new();

    bus.add_subscriber(player1);
    bus.add_subscriber(player2);
    bus.add_publisher(&mut input, Some(85)).unwrap();

    // Send some events
    input.publish(Commands::Move { dx: 1.0, dy: 2.0 }, Some(TopicIds::Player2));
    input.publish(Commands::Move { dx: 1.0, dy: 2.0 }, Some(TopicIds::Player1));
    input.publish(Commands::Atack, Some(TopicIds::Player2));
    input.publish(Commands::Atack, Some(TopicIds::Player1));
}
