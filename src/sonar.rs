use bevy::prelude::*;
use bevy::window::*;
use bevy::sprite::MaterialMesh2dBundle;
use crate::gamestate::GameState;

pub struct SonarPlugin;
impl Plugin for SonarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), setup_sonar)
            .add_systems(Update, line_spin_system.run_if(in_state(GameState::Game)));
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

pub fn setup_sonar(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let window = windows.single_mut();
    let radius = window.resolution.height() / 2.0;
    let texture_handle = asset_server.load("../assets/radar.png");
    let texture_height = 857.0; //yes, this is the hardcoded sprite's height in px's (dont @ me)
    let (x, y) = (0.0, 0.0);
    commands.spawn((
        SpriteBundle {
            texture: texture_handle,
            transform: Transform {
                translation: Vec3::new(x, y, -2.0),
                scale: Vec3::splat(radius * 2.0 / texture_height),
                ..default()
            },
            ..default()
        },
        Sonar {
            radius: radius,
        }
    ));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(2.0, radius)).into(),
            material: materials.add(Color::GREEN),
            transform: Transform::from_translation(Vec3::Z * -1.0), //line_spin_system() will set x & y for us
            ..default()
        },
        Line {
            rotation_speed: 2.0,
        }
    ));
}

pub fn line_spin_system(
    time: Res<Time>,
    mut line: Query<(&Line, &mut Transform)>,
    mut sonar: Query<(&Sonar, &Transform), Without<Line>>,
) {
    let (line, mut transform) = line.single_mut();
    {
        let old_w = transform.rotation.w;
        transform.rotate_z(-1. * line.rotation_speed * time.delta_seconds());
        if transform.rotation.w > old_w {
            transform.rotation.w *= -1.0;
            transform.rotation.z *= -1.0;
        }
    }
    {
        let (sonar, sonar_transform) = sonar.single_mut();
        let radius = sonar.radius / 2.0;
        let (_, radian) = transform.rotation.to_axis_angle();
        let x = radian.sin() * radius + sonar_transform.translation.x;
        let y = radian.cos() * radius + sonar_transform.translation.y;
        *transform = transform.with_translation(Vec3::new(x, y, transform.translation.z));
    }
}
