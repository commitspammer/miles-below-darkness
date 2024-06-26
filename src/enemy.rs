use bevy::prelude::*;
use bevy::window::*;
use bevy::math::bounding::RayCast2d;
use bevy::math::bounding::BoundingCircle;
use crate::gamestate::GameState;
use crate::sonar::Sonar;
use crate::hitbox::Hitbox;
use rand::Rng;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), spawn_enemy)
           .add_systems(Update, enemy_movement_system.run_if(in_state(GameState::Game)))
           .add_systems(Update, enemy_rotation_system.run_if(in_state(GameState::Game)))
           .add_systems(Update, enemy_destination_system.run_if(in_state(GameState::Game)))
           .insert_resource(EnemyPositions::default());
        }
}

#[derive(Component)]
pub struct Enemy {
    rotation_speed: f32,
    movement_speed: f32,
    destination: Vec3,
}

#[derive(Default, Resource)]
pub struct EnemyPositions {
    pub positions: Vec<Vec3>,
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
                movement_speed: 40.0,
                destination: Vec3::ZERO, //this will be set by enemy_rotation_system()
            },
            Hitbox::new(30.0, 90.0),
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
        if enemy.destination == Vec3::ZERO
            || transform.translation.distance(enemy.destination) <= 1.0
        {
            enemy.destination = loop {
                let radian = rng.gen_range(0.0..std::f32::consts::TAU);
                let distance = rng.gen_range(min_distance..=max_distance);
                let position = Vec3::new(
                    distance * radian.sin() + center.x,
                    distance * radian.cos() + center.y,
                    transform.translation.z
                );
                //println!("{:?}", position);
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
            //println!("");
        }
    }
}

fn enemy_rotation_system(
    time: Res<Time>,
    mut query: Query<(&mut Enemy, &mut Transform)>,
) {
    for (enemy, mut transform) in query.iter_mut() {
        let to_destination = (enemy.destination.xy() - transform.translation.xy()).normalize();
        let up = transform.up().xy();
        let up_dot = up.dot(to_destination);
        if (up_dot - 1.0).abs() < f32::EPSILON {
            continue;
        }
        let right = transform.right().xy();
        let right_dot = right.dot(to_destination);
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
        transform.translation += up * enemy.movement_speed * time.delta_seconds();
        enemy_positions.positions.push(transform.translation);
    }
}
