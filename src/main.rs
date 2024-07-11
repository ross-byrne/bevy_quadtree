use bevy::window::PrimaryWindow;
use bevy::{
    color::palettes::css::{GREEN, RED},
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle, Wireframe2dPlugin},
};
use quadtree::{QuadTree, TreeNode};
use rand::prelude::*;

mod quadtree;

const WORLD_HEIGHT: f32 = 600.0;
const WORLD_WIDTH: f32 = 1000.0;
const QTREE_CAPACITY: usize = 6;
const CAPTURE_RECT_HEIGHT: f32 = 120.0;
const CAPTURE_RECT_WIDTH: f32 = 160.0;

#[derive(Component)]
struct MainCamera;

#[derive(Resource, Debug, Default, Deref, DerefMut)]
struct WorldTree(pub QuadTree);

#[derive(Component)]
struct CaptureRect;

fn main() {
    App::new()
        .init_resource::<WorldTree>()
        .add_plugins((DefaultPlugins, Wireframe2dPlugin))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                draw_qtree_gizmos,
                mouse_button_input,
                colour_selected_points,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut world_tree: ResMut<WorldTree>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((Camera2dBundle::default(), MainCamera));

    // initialise world tree. Centered to 0.0, 0.0
    let origin: Vec2 = Vec2::new(0.0, 0.0);
    let half_size: Vec2 = Vec2::new(WORLD_WIDTH / 2.0, WORLD_HEIGHT / 2.0);
    *world_tree = WorldTree(QuadTree::new(origin, half_size, QTREE_CAPACITY));

    // create some objects
    let mut rng = rand::thread_rng();
    let spawn_padding: f32 = 10.0;

    for _ in 0..100 {
        let x: f32 = rng
            .gen_range(-(WORLD_WIDTH / 2.0) + spawn_padding..(WORLD_WIDTH / 2.0) - spawn_padding);
        let y: f32 = rng
            .gen_range(-(WORLD_HEIGHT / 2.0) + spawn_padding..WORLD_HEIGHT / 2.0 - spawn_padding);

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
        world_tree.insert(&TreeNode::new(Some(entity), x, y));
    }

    info!("Children: {:?}", world_tree.get_childen().len());
    info!(
        "Number of Subdivisions: {}",
        world_tree.get_tree_rects().len()
    );
}

fn draw_qtree_gizmos(
    mut gizmos: Gizmos,
    world_tree: ResMut<WorldTree>,
    capture_rect: Query<&Transform, With<CaptureRect>>,
) {
    // draw quad tree segments
    for rect in world_tree.get_tree_rects() {
        gizmos.rect_2d(rect.center(), 0.0, rect.size(), GREEN)
    }

    // draw capture rect
    if let Ok(tansform) = capture_rect.get_single() {
        gizmos.rect_2d(
            tansform.translation.xy(),
            0.0,
            Vec2::new(CAPTURE_RECT_WIDTH, CAPTURE_RECT_HEIGHT),
            Color::from(RED),
        );
    };
}

fn mouse_button_input(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    capture_rect: Query<Entity, With<CaptureRect>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let window = q_window.single();
        let (camera, camera_transform) = q_camera.single();

        // check if the cursor is inside the window and get its position
        // then, ask bevy to convert into world coordinates, and truncate to discard Z
        let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        else {
            return;
        };

        info!("World coords: {}/{}", world_position.x, world_position.y);

        if let Ok(entity) = capture_rect.get_single() {
            // remove exiting capture rect
            commands.entity(entity).despawn_recursive();
        }

        // spawn new one
        commands.spawn((
            CaptureRect,
            Transform::from_xyz(world_position.x, world_position.y, 1.0),
        ));

        // change colour of points within capture rect
    }
}

fn colour_selected_points(
    _commands: Commands,
    capture_rect: Query<&Transform, With<CaptureRect>>,
    world_tree: ResMut<WorldTree>,
) {
    // get capture rect or return
    let Ok(transform) = capture_rect.get_single() else {
        return;
    };

    // find all points inside capture rect by asking the qtree
    let children = world_tree.get_childen();
    info!("All Children: {}", children.len());

    // build catpure rect to test qtree
    let range: Rect = Rect::from_center_size(
        Vec2::from(transform.translation.xy()),
        Vec2::new(CAPTURE_RECT_WIDTH, CAPTURE_RECT_HEIGHT),
    );

    let contained: Vec<&TreeNode> = world_tree.query(&range);
    info!("Children in range: {}", contained.len());
}
