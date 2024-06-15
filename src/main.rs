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
        .add_systems(Update, line_movement_system)
        .add_systems(Update, line_rotation_system)
        .run()
}

#[derive(Component)]
struct Player {
    rotation_speed: f32,
}

#[derive(Component)]
struct Line {
    movement_speed: f32,
    rotation_speed: f32,
    //radian: &mut f32,
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
        visibility: bevy::render::view::Visibility::Hidden,
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

    //let fw = transform.right();
    //transform.translation += fw * 64.0 * time.delta_seconds();
    transform.rotate_z(rotation_factor * player.rotation_speed * time.delta_seconds());
}

fn spawn_line(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let window = windows.single_mut();
    let radius = window.resolution.height() / 2.0; // Raio do mapa
    // A linha tem comprimento igual ao raio para que uma ponta toque o centro e a outra a borda do círculo
    let line_mesh = Mesh::from(shape::Quad::new(Vec2::new(2.0, radius)));
    let line_handle = meshes.add(line_mesh);

    commands.spawn((MaterialMesh2dBundle {
        mesh: line_handle.into(),
        material: materials.add(Color::GREEN),
        transform: Transform {
            translation: Vec3::new(0.0, radius / 2.0, 0.0), // Posiciona a linha para que uma extremidade toque o centro
            rotation: Quat::from_rotation_z(0.0), // Inicia apontando para cima
            scale: Vec3::ONE,
        },
        ..default()
    }, Line {
        movement_speed: 12.,
        rotation_speed: 2.,
        //radian: 0.,
    }));
}

fn line_movement_system(
    time: Res<Time>,
    mut query: Query<(&Line, &mut Transform)>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    //let (line, mut transform) = query.single_mut();
    //let right = transform.right();
    //transform.translation += right * line.movement_speed * time.delta_seconds();

    let (line, mut transform) = query.single_mut();

    transform.rotate_z(-1. * line.rotation_speed * time.delta_seconds());
    //if transform.rotation.z > 2. * PI * radius { transform.rotation %= 2. * PI * radius }

    let window = windows.single_mut();
    let radius = window.resolution.height() / 4.0;
    let (_, radian) = transform.rotation.to_axis_angle();
    let x = radian.sin() * radius;
    let y = radian.cos() * radius;
    *transform = transform.with_translation(Vec3::new(x, y, transform.translation.z));

    //println!("{} {}", transform.rotation.z, transform.rotation.w);
    println!("{}", radian);
}

fn line_rotation_system(
    time: Res<Time>,
    mut query: Query<(&Line, &mut Transform)>,
) {
    //let (line, mut transform) = query.single_mut();
    //transform.rotate_z(-1.0 * line.rotation_speed * time.delta_seconds());

    //let (line, mut transform) = query.single_mut();
    //let center = Vec3::ZERO;
    //let look_at_center = transform.looking_at(center, *transform.local_y());
    //let incremental_turn_weight = line.rotation_speed * time.delta_seconds();
    //let old_rotation = transform.rotation;
    //transform.rotation = old_rotation.lerp(look_at_center.rotation, incremental_turn_weight);

    //let (line, mut transform) = query.single_mut();
    //let translation = transform.translation.xy();
    //let center = Transform::from_xyz(0., 0., 0.);
    //let to_center = (translation - center.translation.xy()).normalize();
    //let rotation_to_center = Quat::from_rotation_arc(Vec3::Y, to_center.extend(0.));
    //transform.rotation = rotation_to_center;
    ////let old_rotation = transform.rotation;
    ////transform.rotation = old_rotation.lerp(rotation_to_center, line.rotation_speed * time.delta_seconds());
    

}
