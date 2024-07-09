use bevy::prelude::*;

#[derive(Component, Debug)]
struct Point {
    value: Vec2,
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
    commands.spawn(Point {
        value: Vec2::new(10.0, 10.0),
    });
}

fn print_positions(mut _commands: Commands, q: Query<&Point>) {
    for point in q.iter() {
        info!("Hello Position: {:?}", point.value);
    }
}
