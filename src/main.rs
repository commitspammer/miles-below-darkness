use bevy::prelude::*;
use bevy::window::*;
use bevy::sprite::MaterialMesh2dBundle;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1920., 1080.).with_scale_factor_override(1.0),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_cam)
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_map)
        .add_systems(Startup, spawn_line)
        .add_systems(FixedUpdate, player_rotation_system)
        .add_systems(Update, line_spin_system)
        .run()
}

#[derive(Component)]
struct Player {
    rotation_speed: f32,
}

#[derive(Component)]
struct Line {
    rotation_speed: f32,
}

fn setup_cam(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = meshes.add(Capsule2d::new(15.0, 70.0)).into();
    commands.spawn((MaterialMesh2dBundle {
        mesh: shape,
        material: materials.add(Color::GREEN),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        //visibility: bevy::render::view::Visibility::Hidden,
        ..default()
    },
    Player {
        rotation_speed: 1.0,
    }));
}

fn spawn_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let window = windows.single_mut();
    let map = meshes.add(Circle { radius: window.resolution.height() / 2.0 }).into();
    commands.spawn(MaterialMesh2dBundle {
        mesh: map,
        material: materials.add(Color::BLACK),
        transform: Transform::from_xyz(0.0, 0.0, -1.0),
        ..default()
    });
}

fn player_rotation_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    let (player, mut transform) = query.single_mut();

    let mut rotation_factor = 0.0;
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        rotation_factor += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        rotation_factor -= 1.0;
    }

    transform.rotate_z(rotation_factor * player.rotation_speed * time.delta_seconds());
}

fn spawn_line(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let window = windows.single_mut();
    let radius = window.resolution.height() / 2.0;
    let line_mesh = Mesh::from(shape::Quad::new(Vec2::new(2.0, radius)));
    let line_handle = meshes.add(line_mesh);

    commands.spawn((MaterialMesh2dBundle {
        mesh: line_handle.into(),
        material: materials.add(Color::GREEN),
        transform: Transform {
            translation: Vec3::new(0.0, radius / 2.0, 0.0),
            rotation: Quat::from_rotation_z(0.0),
            scale: Vec3::ONE,
        },
        ..default()
    }, Line {
        rotation_speed: 2.,
    }));
}

fn line_spin_system(
    time: Res<Time>,
    mut query: Query<(&Line, &mut Transform)>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let (line, mut transform) = query.single_mut();

    let old_w = transform.rotation.w;
    transform.rotate_z(-1. * line.rotation_speed * time.delta_seconds());
    if transform.rotation.w > old_w {
        transform.rotation.w *= -1.;
        transform.rotation.z *= -1.;
    }

    let window = windows.single_mut();
    let radius = window.resolution.height() / 4.0;
    let (_, radian) = transform.rotation.to_axis_angle();
    let x = radian.sin() * radius;
    let y = radian.cos() * radius;
    *transform = transform.with_translation(Vec3::new(x, y, transform.translation.z));
}
