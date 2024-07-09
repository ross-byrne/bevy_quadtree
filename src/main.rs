use crate::quadtree::Point;
use bevy::prelude::*;
use quadtree::QuadTree;

mod quadtree;

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
    // initialise world tree
    *world_tree = WorldTree(QuadTree::new(4));

    world_tree.add_child(Point::new(12.0, 14.0));
    world_tree.add_child(Point::new(12.0, 14.0));
    world_tree.add_child(Point::new(12.0, 14.0));
    world_tree.add_child(Point::new(12.0, 14.0));
    world_tree.add_child(Point::new(12.0, 14.0));

    info!("Children: {:?}", world_tree.get_childen());
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
