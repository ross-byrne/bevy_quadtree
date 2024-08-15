use bevy::window::PrimaryWindow;
use bevy::{
    color::palettes::css::{GREEN, RED, ROYAL_BLUE},
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle, Wireframe2dPlugin},
};
use quadtree::{QuadTree, TreeNode};
use rand::prelude::*;
use std::time::Instant;

mod quadtree;

const WORLD_HEIGHT: f32 = 600.0;
const WORLD_WIDTH: f32 = 1000.0;
const QTREE_CAPACITY: usize = 6;
const NUMBER_OF_POINTS: usize = 200;

const CAPTURE_RECT_HEIGHT: f32 = 120.0;
const CAPTURE_RECT_WIDTH: f32 = 160.0;

const POINT_RADIUS: f32 = 5.0;

#[derive(Component)]
struct MainCamera;

#[derive(Resource, Debug, Default, Deref, DerefMut)]
struct WorldTree(pub QuadTree);

#[derive(Component)]
struct Points;

#[derive(Component)]
struct CaptureRect;

#[derive(Component)]
struct Captured;

#[derive(Event)]
struct UntagPoints;

#[derive(Event)]
struct TagPoints {
    origin: Vec2,
}

fn main() {
    App::new()
        .init_resource::<WorldTree>()
        .add_plugins((DefaultPlugins, Wireframe2dPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (draw_qtree_gizmos, mouse_button_input))
        .observe(tag_points)
        .observe(untag_points)
        .run();
}

fn setup(
    mut commands: Commands,
    mut world_tree: ResMut<WorldTree>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((Camera2dBundle::default(), MainCamera));

    // benchmark
    let before = Instant::now();

    // initialise world tree. Centered to 0.0, 0.0
    let origin: Vec2 = Vec2::new(0.0, 0.0);
    let size: Vec2 = Vec2::new(WORLD_WIDTH, WORLD_HEIGHT);
    *world_tree = WorldTree(QuadTree::new(origin, size, QTREE_CAPACITY));

    // create some objects
    let mut rng = rand::thread_rng();
    let spawn_padding: f32 = 10.0;

    for _ in 0..NUMBER_OF_POINTS {
        let x: f32 = rng
            .gen_range(-(WORLD_WIDTH / 2.0) + spawn_padding..(WORLD_WIDTH / 2.0) - spawn_padding);
        let y: f32 = rng
            .gen_range(-(WORLD_HEIGHT / 2.0) + spawn_padding..WORLD_HEIGHT / 2.0 - spawn_padding);

        // spawn entity
        let entity: Entity = commands
            .spawn((
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Circle {
                        radius: POINT_RADIUS,
                    })),
                    material: materials.add(Color::WHITE),
                    transform: Transform::from_xyz(x, y, 0.0),
                    ..default()
                },
                Points,
            ))
            .id();

        // add entity to quadtree
        world_tree.insert(TreeNode::new(Some(entity), x, y));
    }

    info!("Children: {:?}", world_tree.get_childen().len());
    info!(
        "Number of Subdivisions: {}",
        world_tree.get_tree_rects().len()
    );

    info!("Time taken to setup: {:?}", before.elapsed());
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

        // remove existing tagged points
        commands.trigger(UntagPoints);

        // tag points in capture range
        commands.trigger(TagPoints {
            origin: Vec2::from(world_position),
        });
    }
}

fn untag_points(
    _: Trigger<UntagPoints>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    capture_rect: Query<Entity, With<CaptureRect>>,
    captured_points: Query<Entity, (With<Points>, With<Captured>)>,
) {
    // remove existing capture rect
    if let Ok(capture_rect_entity) = capture_rect.get_single() {
        commands.entity(capture_rect_entity).despawn_recursive();
    }

    // reset all captured points
    for entity in captured_points.iter() {
        commands
            .entity(entity)
            .remove::<Captured>()
            .insert(materials.add(Color::WHITE));
    }
}

fn tag_points(
    trigger: Trigger<TagPoints>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    world_tree: ResMut<WorldTree>,
    points: Query<Entity, With<Points>>,
) {
    // get clicked origin position
    let capture_origin = trigger.event().origin;

    // spawn a new capture rect
    commands.spawn((
        CaptureRect,
        Transform::from_xyz(capture_origin.x, capture_origin.y, 1.0),
    ));

    // build catpure rect to test qtree
    let range: Rect = Rect::from_center_size(
        Vec2::from(capture_origin),
        Vec2::new(CAPTURE_RECT_WIDTH, CAPTURE_RECT_HEIGHT),
    );

    let before = Instant::now();
    let mut count: i32 = 0;
    // get children from tree
    let contained: Vec<&TreeNode> = world_tree.query(&range, &mut count);
    info!(
        "Children in range: {}. Comparisons: {}. Time taken: {:?}",
        contained.len(),
        count,
        before.elapsed()
    );

    // tag new points to highlight them
    for child in contained.iter() {
        let Some(saved_entity) = child.entity else {
            continue;
        };

        if let Ok(entity) = points.get(saved_entity) {
            commands
                .entity(entity)
                .insert(Captured)
                .insert(materials.add(Color::from(ROYAL_BLUE)));
        }
    }
}
