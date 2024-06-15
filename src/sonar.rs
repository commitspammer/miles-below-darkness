use bevy::prelude::*;
use bevy::window::*;
use bevy::sprite::MaterialMesh2dBundle;

pub struct SonarPlugin;
impl Plugin for SonarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_sonar)
            .add_systems(Startup, spawn_line)
            .add_systems(Update, line_spin_system);
    }
}

#[derive(Component)]
pub struct Sonar {
    radius: f32,
}

#[derive(Component)]
pub struct Line {
    rotation_speed: f32,
}

pub fn spawn_sonar(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let window = windows.single_mut();
    let sonar = meshes.add(Circle { radius: window.resolution.height() / 2.0 }).into();
    commands.spawn((MaterialMesh2dBundle {
        mesh: sonar,
        material: materials.add(Color::BLACK),
        transform: Transform::from_xyz(0.0, 0.0, -1.0),
        ..default()
    },
    Sonar {
        radius: window.resolution.height() / 2.,
    }));
}

pub fn spawn_line(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let window = windows.single_mut();
    let radius = window.resolution.height() / 2.0;
    let line_mesh = Mesh::from(Rectangle::new(2.0, radius));
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

pub fn line_spin_system(
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
