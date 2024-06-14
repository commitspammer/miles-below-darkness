use bevy::prelude::*;
use bevy::window::*;
use bevy::sprite::MaterialMesh2dBundle;

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
        .add_systems(FixedUpdate, player_movement_system)
        .run()
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

#[derive(Component)]
struct Player {
    rotation_speed: f32,
}

fn player_movement_system(
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
