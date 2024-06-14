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
        .add_systems(Startup, spawn_line) // Adicione esta linha
        .add_systems(FixedUpdate, player_movement_system)
        .add_systems(FixedUpdate, line_movement_system) // Adicione esta linha
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

#[derive(Component)]
struct Line;

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

    commands.spawn(MaterialMesh2dBundle {
        mesh: line_handle.into(),
        material: materials.add(Color::GREEN),
        transform: Transform {
            translation: Vec3::new(0.0, radius / 2.0, 0.0), // Posiciona a linha para que uma extremidade toque o centro
            rotation: Quat::from_rotation_z(0.0), // Inicia apontando para cima
            scale: Vec3::ONE,
        },
        ..default()
    }).insert(Line);
}

fn line_movement_system(
    time: Res<Time>,
    mut query: Query<(&Line, &mut Transform)>,
) {
    for (_, mut transform) in query.iter_mut() {
        let rotation_speed = 1.0; // Ajuste a velocidade de rotação conforme necessário
        let angle = rotation_speed * time.delta_seconds();
    
        // Obtém a rotação atual em Euler, adiciona o ângulo, e converte de volta para Quat
        //let current_rotation_z = transform.rotation.to_euler(EulerRot::XYZ).2;
        let new_rotation_x = transform.rotation.to_euler(EulerRot::XYZ).2 + angle;
        transform.rotation = Quat::from_rotation_z(new_rotation_x);
    
        // Calcula a nova posição usando cos e sin no ângulo de rotação
        let radius = transform.translation.length();
        transform.translation = Vec3::new(
            radius * new_rotation_x.cos(), 
            radius * new_rotation_x.sin(), 
            0.0,
        );
    }
}
