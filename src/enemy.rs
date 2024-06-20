use bevy::prelude::*;
use bevy::math::Vec3;
use rand::Rng; 
use bevy::window::*;
use std::time::Duration;
use bevy::time::TimerMode;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,spawn_enemy)
           .add_systems(Update,enemy_movement_system);
    }
}

#[derive(Component)]
pub struct Enemy {
    rotation_speed: f32,
    move_speed: f32,
    move_timer: Timer,
}

pub fn spawn_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let window = windows.single_mut();
    let radius = (window.resolution.height() / 4.0) + 20.0; // Distância mínima do centro
    let mut rng = rand::thread_rng();

    for _ in 0..5 { // Exemplo: spawnar 5 inimigos
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let distance = rng.gen_range(radius..radius + 200.0);
        let position = Vec3::new(distance * angle.cos(), distance * angle.sin(), 0.0);

        let direction_to_player = Vec3::new(0.0, 0.0, 0.0) - position;
        let angle_to_player = direction_to_player.y.atan2(direction_to_player.x);

        commands.spawn(SpriteBundle {
            texture: asset_server.load("../assets/submarino.png"),
            transform: Transform {
                translation: position,
                rotation: Quat::from_rotation_z(angle_to_player - std::f32::consts::FRAC_PI_2),
                scale: Vec3::splat(0.1),
            },
            ..default()
        })
        .insert(Enemy {
            rotation_speed: rng.gen_range(0.5..2.0),
            move_speed: 10.0,
            move_timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
        });
    }
}

fn enemy_movement_system(
    time: Res<Time>,
    mut query: Query<(&mut Enemy, &mut Transform)>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let window = windows.single_mut();
    let distance_limit = (window.resolution.height() / 4.0) + 20.0; // Define a distância limite baseada na altura da janela
    let origin = Vec3::ZERO; 

    for (mut enemy, mut transform) in query.iter_mut() {
        enemy.move_timer.tick(time.delta());

        if enemy.move_timer.finished() {
            let current_distance = transform.translation.distance(origin);
            if current_distance > distance_limit {
                let move_distance = enemy.move_speed;
                let direction_to_origin = (origin - transform.translation).normalize();
                transform.translation += direction_to_origin * move_distance;
            } else {
                println!("Inimigo dentro da distância limite ou muito próximo ao ponto de origem");
            }

            enemy.move_timer.reset();
        }
    }
}