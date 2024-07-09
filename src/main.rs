use crate::quadtree::Point;
use bevy::prelude::*;
use quadtree::QuadTree;

mod quadtree;

const WORLD_HEIGHT: f32 = 1000.0;
const WORLD_WIDTH: f32 = 1000.0;

#[derive(Resource, Debug, Default, Deref, DerefMut)]
struct WorldTree(pub QuadTree<Point>);

fn main() {
    App::new()
        .init_resource::<WorldTree>()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, print_positions)
        .run();
}

fn setup(mut commands: Commands, mut world_tree: ResMut<WorldTree>) {
    // initialise world tree. Centered to 0.0, 0.0
    let origin: Vec2 = Vec2::new(0.0, 0.0);
    let half_size: Vec2 = Vec2::new(WORLD_WIDTH / 2.0, WORLD_HEIGHT / 2.0);
    *world_tree = WorldTree(QuadTree::new(origin, half_size, 4));

    world_tree.add_child(Point::new(12.0, 14.0));
    world_tree.add_child(Point::new(12.0, 14.0));
    world_tree.add_child(Point::new(12.0, 14.0));
    world_tree.add_child(Point::new(12.0, 14.0));
    world_tree.add_child(Point::new(12.0, 14.0));

    info!("Children: {:?}", world_tree.get_childen().len());
    // add some points
    // commands.spawn(Point::new(10.0, 10.0));

    info!("{:?}", world_tree);
    info!("Is Subdivided: {}", world_tree.subdivided);
}

fn print_positions(mut _commands: Commands, q: Query<&Point>) {
    for point in q.iter() {
        info!("Hello Position: {:?}", point);
    }
}
