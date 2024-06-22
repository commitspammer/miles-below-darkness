use bevy::prelude::*;
use bevy::math::Vec3;
use rand::Rng; 
use bevy::window::*;
use std::time::Duration;
use bevy::time::TimerMode;
use crate::gamestate::GameState;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), spawn_enemy)
           .add_systems(Update, enemy_movement_system.run_if(in_state(GameState::Game)))
           .add_systems(Update, collision_detection_system.run_if(in_state(GameState::Game)));

    }
}

#[derive(Component)]
pub struct Hitbox {
    pub size: Vec2, // Dimensões da hitbox
}

#[derive(Component)]
pub struct Enemy {
    rotation_speed: f32,
    move_speed: f32,
    move_timer: Timer,
}

fn create_enemy(rotation_speed: f32, move_speed: f32, move_timer: Timer) -> Enemy {
    Enemy {
        rotation_speed,
        move_speed,
        move_timer,
    }
}

fn position_is_free(position: Vec3, existing_enemies: &Vec<(Entity, Vec3, Vec2)>) -> bool {
    println!("antes do for, existing_enemies len: {}", existing_enemies.len());
    let buffer_distance = 1.5; // Ajuste para um valor razoável
    for (_, enemy_pos, enemy_size) in existing_enemies.iter() {
        let adjusted_size = enemy_size.x * buffer_distance;
        if position.distance(*enemy_pos) < adjusted_size {
            println!("dentro do for e dentro do if");
            return false;
        }
        println!("dentro do for e fora do if");
    }
    println!("Posição livre,{}", position);
    true
}

pub fn spawn_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    query: Query<(Entity, &Transform, &Hitbox)>, 
) {
    let window = windows.single_mut();
    let radius = (window.resolution.height() / 4.0) + 20.0;
    let mut rng = rand::thread_rng();

    let mut existing_enemies: Vec<(Entity, Vec3, Vec2)> = query.iter().map(|(e, t, h)| (e, t.translation, h.size)).collect();

    for _ in 0..5 {
        let mut position = Vec3::ZERO;
        let mut position_found = false;

        while !position_found {
            let angle = rng.gen_range(0.0..std::f32::consts::TAU);
            let distance = rng.gen_range(radius..radius + 200.0);
            position = Vec3::new(distance * angle.cos(), distance * angle.sin(), 0.0);

            if position_is_free(position, &existing_enemies) {
                position_found = true;
            }
        }

        let direction_to_player = Vec3::new(0.0, 0.0, 0.0) - position;
        let angle_to_player = direction_to_player.y.atan2(direction_to_player.x);

        
        let enemy_entity = commands.spawn(SpriteBundle {
            texture: asset_server.load("../assets/enemy.png"),
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
        })
        .insert(Hitbox {
            size: Vec2::new(50.0, 50.0),
        })
        .id();

       
        existing_enemies.push((enemy_entity, position, Vec2::new(50.0, 50.0)));
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
                //println!("Inimigo dentro da distância limite ou muito próximo ao ponto de origem");
            }

            enemy.move_timer.reset();
        }
    }
}

fn collision_detection_system(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Hitbox)>,
) {
    let entities: Vec<(Entity, &Transform, &Hitbox)> = query.iter().collect();
    for (i, (entity_a, transform_a, hitbox_a)) in entities.iter().enumerate() {
        for (entity_b, transform_b, hitbox_b) in entities.iter().skip(i + 1) {
            let distance = transform_a.translation.truncate() - transform_b.translation.truncate();
            if distance.length() < (hitbox_a.size + hitbox_b.size).length() / 2.0 {
                println!("Colisão detectada")
            }
        }
    }
}
