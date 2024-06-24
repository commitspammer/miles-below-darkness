use bevy::prelude::*;
use bevy::window::*;
use crate::gamestate::GameState;
use crate::sonar::Sonar;
use std::time::Duration;
use rand::Rng;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), spawn_enemy)
           .add_systems(Update, enemy_movement_system.run_if(in_state(GameState::Game)))
           .add_systems(Update, enemy_rotation_system.run_if(in_state(GameState::Game)))
           .add_systems(Update, enemy_destination_system.run_if(in_state(GameState::Game)))
           .add_systems(Update, collision_detection_system.run_if(in_state(GameState::Game)));
    }
}

#[derive(Component)]
pub struct Hitbox {
    size: Vec2,
}

#[derive(Component)]
pub struct Enemy {
    rotation_speed: f32,
    movement_speed: f32,
    destination: Vec3,
    move_timer: Timer,
    patrol_points: Vec<Vec3>,
    current_patrol_index: usize,
}

fn generate_patrol_points(center: Vec3, radius: f32, count: usize) -> Vec<Vec3> {
    let mut points = Vec::new();
    let angle_increment = std::f32::consts::TAU / count as f32;

    for i in 0..count {
        let angle = i as f32 * angle_increment;
        let x = center.x + radius * angle.cos();
        let y = center.y + radius * angle.sin();
        points.push(Vec3::new(x, y, 0.0));
    }

    points
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
    let patrol_radius = radius + 100.0;
    let patrol_points = generate_patrol_points(Vec3::ZERO, patrol_radius, 6); // 6 pontos de patrulha
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
            rotation_speed: 0.4,//rng.gen_range(0.5..2.0),
            movement_speed: 40.0,
            destination: Vec3::ZERO, //this will be set by enemy_rotation_system()
            move_timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
            patrol_points: patrol_points.clone(),
            current_patrol_index: 0,
        })
        .insert(Hitbox {
            size: Vec2::new(50.0, 50.0),
        })
        .id();

        existing_enemies.push((enemy_entity, position, Vec2::new(50.0, 50.0)));
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
        if enemy.destination == Vec3::ZERO || transform.translation.distance(enemy.destination) <= 1.0 {
            let radian = rng.gen_range(0.0..std::f32::consts::TAU);
            let distance = rng.gen_range(min_distance..=max_distance);
            let position = Vec3::new(
                distance * radian.sin() + center.x,
                distance * radian.cos() + center.y,
                transform.translation.z
            );
            enemy.destination = position;
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
    time: Res<Time>,
    mut query: Query<(&mut Enemy, &mut Transform)>,
) {
    for (enemy, mut transform) in query.iter_mut() {
        let up = transform.up();
        transform.translation += up * enemy.movement_speed * time.delta_seconds();

        //enemy.move_timer.tick(time.delta());

        //if enemy.move_timer.finished() {
        //    let target = enemy.patrol_points[enemy.current_patrol_index];
        //    let direction = (target - transform.translation).normalize();
        //    let desired_angle = direction.y.atan2(direction.x);
        //    let current_angle = transform.rotation.to_euler(EulerRot::XYZ).2; 

        //    let mut angle_diff = (desired_angle - current_angle).abs();
        //    if angle_diff > PI {
        //        angle_diff = 2.0 * PI - angle_diff; 
        //    }

        //    let rotation_limit = FRAC_PI_4; 
        //    if angle_diff > rotation_limit {
        //        angle_diff = angle_diff.signum() * rotation_limit;
        //    }

        //    println!("Enemy position1: {:?}", transform.translation);
        //   
        //    transform.rotation *= Quat::from_rotation_z(angle_diff);

        //    println!("Enemy position2: {:?}", transform.translation);

        //    let forward = transform.forward();
        //    let forward_xy = Vec3::new(forward.x, forward.y, 0.0);
        //    println!("forward_xy: {:?}", forward_xy);
        //    transform.translation += forward_xy * enemy.movement_speed * time.delta_seconds();

        //    println!("Enemy position3: {:?}", transform.translation);

        //    if transform.translation.distance(target) < 5.0 {
        //        enemy.current_patrol_index = (enemy.current_patrol_index + 1) % enemy.patrol_points.len();
        //    }

        //    enemy.move_timer.reset();
        //}
    }
}

fn collision_detection_system(
    _commands: Commands,
    query: Query<(Entity, &Transform, &Hitbox)>,
) {
    let entities: Vec<(Entity, &Transform, &Hitbox)> = query.iter().collect();
    for (i, (_entity_a, transform_a, hitbox_a)) in entities.iter().enumerate() {
        for (_entity_b, transform_b, hitbox_b) in entities.iter().skip(i + 1) {
            let distance = transform_a.translation.truncate() - transform_b.translation.truncate();
            if distance.length() < (hitbox_a.size + hitbox_b.size).length() / 2.0 {
                //println!("Colisão detectada")
            }
        }
    }
}

//fn sex(
//    mut commands: Commands,
//    mut sonar_query: Query<(&Sonar, &Transform), Without<Enemy>>,
//    mut meshes: ResMut<Assets<Mesh>>,
//    mut materials: ResMut<Assets<ColorMaterial>>,
//) {
//    let (mut sonar, sonar_transform) = sonar_query.single_mut();
//    let max_distance = sonar.radius;
//    let min_distance = 185.0;
//    let center = sonar_transform.translation;
//
//    let mut rng = rand::thread_rng();
//    let radian = rng.gen_range(0.0..std::f32::consts::TAU);
//    let distance = rng.gen_range(min_distance..=max_distance);
//    let position = Vec3::new(
//        distance * radian.sin() + center.x,
//        distance * radian.cos() + center.y,
//        3.0
//    );
//    commands.spawn((
//        bevy::sprite::MaterialMesh2dBundle {
//            mesh: meshes.add(Rectangle::new(3.0, 3.0)).into(),
//            material: materials.add(Color::RED),
//            transform: Transform::from_translation(position),
//            ..default()
//        }
//    ));
//}
