use bevy::prelude::*;

#[derive(Component, Debug)]
struct Position {
    x: f32,
    y: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, print_positions)
        .run();
}

fn setup(mut commands: Commands) {
    // add some points
    commands.spawn(Position { x: 10.0, y: 10.0 });
}

fn print_positions(mut _commands: Commands, q: Query<&Position>) {
    for position in q.iter() {
        info!("Hello Position: {:?}", position);
    }
}
