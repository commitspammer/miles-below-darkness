use bevy::prelude::*;
use bevy::window::*;
use bevy::sprite::MaterialMesh2dBundle;
use crate::gamestate::GameState;
use crate::hitbox::Hitbox;
use crate::hitbox::Collision;
use std::time::Duration;

pub struct SonarPlugin;
impl Plugin for SonarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), setup_sonar)
            .add_systems(Update, line_spin_system.run_if(in_state(GameState::Game)))
            .add_systems(Update, ping_system.run_if(in_state(GameState::Game)))
            .add_systems(Update, fade_away_system.run_if(in_state(GameState::Game)));
    }
}

#[derive(Component)]
pub struct Sonar {
    pub radius: f32,
}

#[derive(Component)]
pub struct Line {
    rotation_speed: f32,
}

#[derive(Component)]
pub struct Pingable {
    timer: Timer,
    keep: Duration,
    fade_away: Duration,
}

impl Pingable {
    pub fn default() -> Pingable {
        Pingable {
            timer: Timer::new(Duration::from_millis(2500), TimerMode::Once),
            keep: Duration::from_millis(500),
            fade_away: Duration::from_millis(2000)
        }
    }
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
    let texture_handle2 = asset_server.load("../assets/painel.png");
    let texture_height = 857.0; //yes, this is the hardcoded sprite's height in px's (dont @ me)
    let background_height = 1024.0; 
    let background_width = 1792.0;
    let (x, y) = (0.0, 0.0);
    let painel_center_x = (window.resolution.width() / 2.0) - 1000.0;
    let painel_center_y = (window.resolution.height() / 2.0) - 750.0;
    let scale_factor_x = (window.resolution.width() / background_width) * 1.7; 
    let scale_factor_y = (window.resolution.height() / background_height) * 1.7; 
    
    commands.spawn((
        SpriteBundle {
            texture: texture_handle2,
            transform: Transform {
                translation: Vec3::new(painel_center_x, painel_center_y, -3.0),
                scale: Vec3::new(scale_factor_x, scale_factor_y, 1.0), 
                ..default()
            },
            ..default()
        },
    ));
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
        },
        Hitbox::new(2.0, radius),
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

pub fn ping_system(
    mut event_reader: EventReader<Collision>,
    mut line_query: Query<&Line>,
    mut pingable_query: Query<(&mut Pingable, &mut Sprite)>,
) {
    for event in event_reader.read() {
        let (entity_a, entity_b) = (event.entity_a, event.entity_b);
        let items = if line_query.get(entity_a).is_ok() {
            Some((entity_a, entity_b))
        } else if line_query.get(entity_b).is_ok() {
            Some((entity_b, entity_a))
        } else {
            None
        };
        if let Some((l, p)) = items {
            let Ok(line) = line_query.get(l) else { return; };
            let Ok((mut pingable, mut sprite)) = pingable_query.get_mut(p) else { return; };

            let keep = pingable.keep;
            pingable.timer.set_duration(keep);
            pingable.timer.reset();
        }
    }
}

pub fn fade_away_system(
    time: Res<Time>,
    mut query: Query<(&mut Pingable, &mut Sprite)>,
) {
    for (mut pingable, mut sprite) in query.iter_mut() {
        let color = if pingable.timer.finished() {
            if pingable.timer.duration() == pingable.keep {
                let fade_away = pingable.fade_away;
                pingable.timer.set_duration(fade_away);
                pingable.timer.reset();
            }
            sprite.color.with_a(1.0)
        } else {
            if pingable.timer.duration() == pingable.keep {
                sprite.color.with_a(1.0)
            } else if pingable.timer.duration() == pingable.fade_away {
                sprite.color.with_a(pingable.timer.fraction_remaining())
            } else {
                sprite.color.with_a(0.5) //error
            }
        };
        pingable.timer.tick(Duration::from_secs_f32(time.delta_seconds()));
        sprite.color = color;
    }
}
