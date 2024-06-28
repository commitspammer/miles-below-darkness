use bevy::prelude::*;
use bevy::window::*;
use bevy::math::bounding::RayCast2d;
use bevy::math::bounding::BoundingCircle;
use crate::gamestate::GameState;
use crate::gamestate::despawn_system;
use crate::gamestate::GameDespawnable;
use crate::player::Player;
use crate::sonar::Sonar;
use crate::sonar::Pingable;
use crate::hitbox::Hitbox;
use crate::torpedo::FireRegularTorpedo;
use crate::torpedo::EnemyDamageEvent;
use rand::Rng;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), spawn_enemy)
           .add_systems(Update, enemy_movement_system.run_if(in_state(GameState::Game)))
           .add_systems(Update, enemy_rotation_system.run_if(in_state(GameState::Game)))
           .add_systems(Update, enemy_destination_system.run_if(in_state(GameState::Game)))
           .add_systems(Update, enemy_fire_system.run_if(in_state(GameState::Game)))
           .add_systems(Update, enemy_damage_system.run_if(in_state(GameState::Game)))
           .add_systems(OnEnter(GameState::Menu), despawn_system::<GameDespawnable>)
           .insert_resource(EnemyPositions::default());
        }
}

#[derive(Component)]
pub struct Enemy {
    rotation_speed: f32,
    movement_speed: f32,
    destination: Vec3,
    state: EnemyState,
    life: i32,
}

#[derive(Default, Resource)]
pub struct EnemyPositions {
    pub positions: Vec<Vec3>,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum EnemyState {
    Roaming,
    Attacking,
}

pub fn spawn_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let window = windows.single_mut();
    let radius = (window.resolution.height() / 4.0) + 20.0;
    let mut rng = rand::thread_rng();

    for _ in 0..5 {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let distance = rng.gen_range(radius..radius + 200.0);
        let position = Vec3::new(distance * angle.cos(), distance * angle.sin(), 0.0);
        let direction_to_player = Vec3::new(0.0, 0.0, 0.0) - position;
        let angle_to_player = direction_to_player.y.atan2(direction_to_player.x);

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("../assets/enemy.png"),
                transform: Transform {
                    translation: position,
                    rotation: Quat::from_rotation_z(angle_to_player - std::f32::consts::FRAC_PI_2),
                    scale: Vec3::splat(0.1),
                }, 
                ..default()
            },
            Enemy {
                rotation_speed: 0.4,//rng.gen_range(0.5..2.0),
                movement_speed: 25.0,
                destination: Vec3::ZERO, //this will be set by enemy_rotation_system()
                state: EnemyState::Roaming,
                life: 1,
            },
            Hitbox::new(30.0, 90.0),
            Pingable::default(),
            GameDespawnable,
        ));
    }
}

fn enemy_destination_system(
    mut enemies_query: Query<(&mut Enemy, &mut Transform)>,
    mut sonar_query: Query<(&Sonar, &Transform), Without<Enemy>>,
) {
    let (sonar, sonar_transform) = sonar_query.single_mut();
    let max_distance = sonar.radius;
    let min_distance = 185.0;
    let center = sonar_transform.translation;

    let mut rng = rand::thread_rng();
    for (mut enemy, transform) in enemies_query.iter_mut() {
        if enemy.state != EnemyState::Roaming {
            continue;
        }
        if enemy.destination == Vec3::ZERO
            || transform.translation.distance(enemy.destination) <= 1.0
        {
            let attack = enemy.destination != Vec3::ZERO;
            enemy.destination = loop {
                let radian = rng.gen_range(0.0..std::f32::consts::TAU);
                let distance = rng.gen_range(min_distance..=max_distance);
                let position = Vec3::new(
                    distance * radian.sin() + center.x,
                    distance * radian.cos() + center.y,
                    transform.translation.z
                );
                let ray = RayCast2d::new(
                    transform.translation.xy(),
                    Direction2d::new((position - transform.translation).xy()).expect("BRUH"),
                    sonar.radius * 2.0
                );
                let circle = BoundingCircle::new(center.xy(), min_distance);
                if ray.circle_intersection_at(&circle).is_none() {
                    break position;
                }
            };
            if attack {
                enemy.state = EnemyState::Attacking;
            }
        }
    }
}

fn enemy_rotation_system(
    time: Res<Time>,
    mut query: Query<(&mut Enemy, &mut Transform)>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    for (enemy, mut transform) in query.iter_mut() {
        let to_target = match enemy.state {
            EnemyState::Roaming => (enemy.destination.xy() - transform.translation.xy()).normalize(),
            EnemyState::Attacking => {
                let player_transform = player_query.single();
                (player_transform.translation.xy() - transform.translation.xy()).normalize()
            },
        };
        let up = transform.up().xy();
        let up_dot = up.dot(to_target);
        if (up_dot - 1.0).abs() < f32::EPSILON {
            continue;
        }
        let right = transform.right().xy();
        let right_dot = right.dot(to_target);
        let rotation_factor = -f32::copysign(1.0, right_dot);
        let max_angle = up_dot.clamp(-1.0, 1.0).acos();
        let rotation_angle = (enemy.rotation_speed * time.delta_seconds()).min(max_angle);
        transform.rotate_z(rotation_factor * rotation_angle);
    }
}

fn enemy_movement_system(
    mut enemy_positions: ResMut<EnemyPositions>,
    time: Res<Time>,
    mut query: Query<(&mut Enemy, &mut Transform)>,
) {
    enemy_positions.positions.clear();
    for (enemy, mut transform) in query.iter_mut() {
        let up = transform.up();
        transform.translation += match enemy.state {
            EnemyState::Roaming => up * enemy.movement_speed * time.delta_seconds(),
            EnemyState::Attacking => up * enemy.movement_speed / 2.0 * time.delta_seconds(),
        };
        enemy_positions.positions.push(transform.translation);
    }
}

fn enemy_fire_system(
    mut event_writer: EventWriter<FireRegularTorpedo>,
    mut query: Query<(&mut Enemy, &Transform)>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    for (mut enemy, transform) in query.iter_mut() {
        if enemy.state != EnemyState::Attacking {
            continue;
        }
        let player_transform = player_query.single();
        let to_target = (player_transform.translation.xy() - transform.translation.xy()).normalize();
        let up = transform.up().xy();
        let up_dot = up.dot(to_target);
        if (up_dot - 1.0).abs() < f32::EPSILON {
            event_writer.send(FireRegularTorpedo { from: transform.translation.xy(), towards: to_target });
            enemy.state = EnemyState::Roaming;
        }
    }
}


fn enemy_damage_system(
    mut commands: Commands,
    mut enemy_query: Query<(&mut Enemy, Entity)>,
    mut damage_events: EventReader<EnemyDamageEvent>,
    //mut game_state: ResMut<NextState<GameState>>,
) {
    for damage_event in damage_events.read() {
        if let Ok((mut enemy, enemy_entity)) = enemy_query.get_mut(damage_event.entity) {
            enemy.life -= damage_event.damage;
            if enemy.life <= 0 {
                // Opcional: Adicione lógica para quando o jogador morre, como despawn da entidade
                //game_state.set(GameState::GameOver); 
                commands.entity(enemy_entity).despawn();
            }
        }
    }
}
