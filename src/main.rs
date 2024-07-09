use bevy::{
    color::palettes::css::GREEN,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle, Wireframe2dConfig, Wireframe2dPlugin},
};
use quadtree::{QuadTree, TreeNode};
use rand::prelude::*;

mod quadtree;

const WORLD_HEIGHT: f32 = 1000.0;
const WORLD_WIDTH: f32 = 1000.0;

#[derive(Resource, Debug, Default, Deref, DerefMut)]
struct WorldTree(pub QuadTree);

// #[derive(Component, Debug)]
// pub struct Circle;

fn main() {
    App::new()
        .init_resource::<WorldTree>()
        .add_plugins((DefaultPlugins, Wireframe2dPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, draw_qtree_gizmos)
        .run();
}

fn setup(
    mut commands: Commands,
    mut world_tree: ResMut<WorldTree>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // initialise world tree. Centered to 0.0, 0.0
    let origin: Vec2 = Vec2::new(0.0, 0.0);
    let half_size: Vec2 = Vec2::new(WORLD_WIDTH / 2.0, WORLD_HEIGHT / 2.0);
    *world_tree = WorldTree(QuadTree::new(origin, half_size, 4));

    // create some objects
    let mut rng = rand::thread_rng();
    for _ in 1..20 {
        let x: f32 = rng.gen_range(-(WORLD_WIDTH / 2.0)..WORLD_WIDTH / 2.0);
        let y: f32 = rng.gen_range(-(WORLD_HEIGHT / 2.0)..WORLD_HEIGHT / 2.0);

        // spawn entity
        let entity: Entity = commands
            .spawn(MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle { radius: 5.0 })),
                material: materials.add(Color::WHITE),
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            })
            .id();

        // add entity to quadtree
        world_tree.add_child(TreeNode::new(Some(entity), x, y));
    }

    info!("Children: {:?}", world_tree.get_childen().len());
    info!("{:?}", world_tree);
    info!("Is Subdivided: {}", world_tree.subdivided);
}

fn draw_qtree_gizmos(mut gizmos: Gizmos, world_tree: Res<WorldTree>, _time: Res<Time>) {
    // draw tree root
    gizmos.rect_2d(
        world_tree.dimensions.center(),
        0.0,
        world_tree.dimensions.size(),
        GREEN,
    )

    // TODO: draw child segments
}
