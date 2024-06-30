use bevy::prelude::*;
use crate::gamestate::GameState;
use crate::gamestate::despawn_system;
use crate::gamestate::GameDespawnable;
use crate::player::Player;
use std::time::Duration;
use std::f32::consts::PI;
use crate::sonar::Pingable;
use crate::hitbox::Hitbox;
use crate::hitbox::InvulnerableAfterSpawn;
use crate::hitbox::Collision;
use crate::enemy::Enemy;
use crate::enemy::EnemyPositions;
// use bevy::ecs::query::QueryEntityError;

pub struct TorpedoPlugin;
impl Plugin for TorpedoPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FireRegularTorpedo>()
            .add_event::<PlayerDamageEvent>()
            .add_event::<EnemyDamageEvent>()
            .add_systems(Update, player_shoot_torpedo_system.run_if(in_state(GameState::Game)))
            .add_systems(Update, shoot_torpedo_event_system.run_if(in_state(GameState::Game)))
            .add_systems(Update, collide_system.run_if(in_state(GameState::Game)))
            .add_systems(Update, move_torpedo_system.run_if(in_state(GameState::Game)))
            .add_systems(OnEnter(GameState::Menu), despawn_system::<GameDespawnable>)
            .insert_resource(TorpedoCooldown(Timer::new(Duration::from_secs(2), TimerMode::Once)));
    }
}

#[derive(Component)]
pub struct Torpedo {
    movement_speed: f32,
    damage: i32,
}

#[derive(Component)]
pub struct RegularTorpedo;

#[derive(Component)]
pub struct GuidedTorpedo;

#[derive(Component)]
pub struct CounterTorpedo;

#[derive(Component)]
pub struct PlayerTorpedo;

#[derive(Component)]
pub struct EnemyTorpedo;

#[derive(Resource, Deref, DerefMut)]
pub struct TorpedoCooldown(Timer);

#[derive(Event)]
pub struct FireRegularTorpedo {
    pub from: Vec2,
    pub towards: Vec2,
}

#[derive(Event)]
pub struct PlayerDamageEvent {
    pub entity: Entity,
    pub damage: i32,
}

#[derive(Event)]
pub struct EnemyDamageEvent {
    pub entity: Entity,
    pub damage: i32,
}

pub fn player_shoot_torpedo_system(
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut player_query: Query<(&Player, &mut Transform)>,
    mut cooldown_timer: ResMut<TorpedoCooldown>,
) {
    cooldown_timer.tick(Duration::from_secs_f32(time.delta_seconds()));
    if !cooldown_timer.finished()  {
        return
    }
    let (_, player_transform) = player_query.single_mut();
    if keyboard_input.pressed(KeyCode::Space) {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("../assets/torpedo-comum.png"),
                transform: Transform {
                    translation: player_transform.translation,
                    rotation: player_transform.rotation,
                    scale: Vec3::splat(0.2),
                },
                ..default()
            },
            Torpedo {
                movement_speed: (35.0/2.0),
                damage: 1,
            },
            RegularTorpedo,
            PlayerTorpedo,
            Hitbox::new(10.0, 50.0),
            InvulnerableAfterSpawn,
            Pingable::default().pinged(),
            GameDespawnable,
        ));
        cooldown_timer.reset();
    }
    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("../assets/torpedo-teleguiado.png"),
                transform: Transform {
                    translation: player_transform.translation,
                    rotation: player_transform.rotation,
                    scale: Vec3::splat(0.2),
                },
                ..default()
            },
            Torpedo {
                movement_speed: (35.0/2.0),
                damage: 1,
            },
            GuidedTorpedo,
            PlayerTorpedo,
            Hitbox::new(15.0, 60.0),
            InvulnerableAfterSpawn,
            Pingable::default().pinged(),
            GameDespawnable,
        ));
        cooldown_timer.reset();
    }
    if keyboard_input.pressed(KeyCode::ControlLeft) {
        for i in vec![0.5, 1.5] {
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("../assets/torpedo-contramedida.png"),
                    transform: Transform {
                        translation: player_transform.translation,
                        rotation: player_transform.rotation * Quat::from_rotation_z(PI * i),
                        scale: Vec3::splat(0.2),
                    },
                    ..default()
                },
                Torpedo {
                    movement_speed: (15.0/2.0),
                    damage: 1,
                },
                CounterTorpedo,
                PlayerTorpedo,
                Hitbox::new(5.0, 25.0),
                InvulnerableAfterSpawn,
                Pingable::default().pinged(),
                GameDespawnable,
            ));
        }
        cooldown_timer.reset();
    }
}

fn shoot_torpedo_event_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut regular_ev_reader: EventReader<FireRegularTorpedo>,
) {
    for (event, _) in regular_ev_reader.read_with_id() {
        let angle = if event.towards.x < 0.0 {
            event.towards.extend(0.0).angle_between(Vec3::Y)
        } else {
            (event.towards * -1.0).extend(0.0).angle_between(Vec3::Y) + PI
        };
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("../assets/torpedo-comum.png"),
                transform: Transform {
                    translation: event.from.extend(0.0),
                    rotation: Quat::from_rotation_z(angle),
                    scale: Vec3::splat(0.2),
                },
                ..default()
            },
            Torpedo {
                movement_speed: (35.0/2.0),
                damage: 1,
            },
            RegularTorpedo,
            EnemyTorpedo,
            Hitbox::new(10.0, 50.0),
            InvulnerableAfterSpawn,
            Pingable::default(),
            GameDespawnable,
        ));
    }
}

fn collide_system(
    mut commands: Commands,
    mut event_reader: EventReader<Collision>,
    mut damage_event_writer: EventWriter<PlayerDamageEvent>,
    mut damage_event_writer2: EventWriter<EnemyDamageEvent>,
    torpedo_query: Query<(Entity, &Hitbox, &Torpedo), With<Torpedo>>,
    enemy_query: Query<Entity, With<Enemy>>,
    player_query: Query<Entity, With<Player>>,
    player_torpedo_query: Query<Entity, With<PlayerTorpedo>>, // Query para identificar torpedos do jogador
) {
    for event in event_reader.read() {
        let entity_a = event.entity_a;
        let entity_b = event.entity_b;

        // Verifica se o torpedo colidiu com o inimigo e se é um torpedo do jogador
        let torpedo_enemy_collision = if torpedo_query.get_component::<Hitbox>(entity_a).is_ok() && enemy_query.get(entity_b).is_ok() {
            player_torpedo_query.get(entity_a).ok().map(|_| entity_a)
        } else if torpedo_query.get_component::<Hitbox>(entity_b).is_ok() && enemy_query.get(entity_a).is_ok() {
            player_torpedo_query.get(entity_b).ok().map(|_| entity_b)
        } else {
            None
        };

        // Verifica se o torpedo colidiu com o jogador (mantém a lógica anterior)
        let torpedo_player_collision = if torpedo_query.get_component::<Hitbox>(entity_a).is_ok() && player_query.get(entity_b).is_ok() {
            Some(entity_a)
        } else if torpedo_query.get_component::<Hitbox>(entity_b).is_ok() && player_query.get(entity_a).is_ok() {
            Some(entity_b)
        } else {
            None
        };

        // Supondo que você já tenha definido PlayerDamageEvent e que torpedo_query possa fornecer o dano
        if let Some(torpedo) = torpedo_enemy_collision {
            // Se o torpedo colidiu com um inimigo, apenas despawne o torpedo
            let enemy_hit = if enemy_query.get(entity_a).is_ok() { entity_a } else { entity_b };

            if let Ok((_, _,  torpedo_component)) = torpedo_query.get(torpedo) {
                println!("Torpedo colidiu com inimigo, despawnando torpedo");
                damage_event_writer2.send(EnemyDamageEvent {
                    entity: enemy_hit,
                    damage: torpedo_component.damage,
                });
                commands.entity(torpedo).despawn();
            }
        } else if let Some(torpedo) = torpedo_player_collision {

            let player_hit = if player_query.get(entity_a).is_ok() { entity_a } else { entity_b };
            // Se o torpedo colidiu com o jogador, emita o evento de dano antes de despawnar o torpedo
            if let Ok((_, _,  torpedo_component)) = torpedo_query.get(torpedo) {
                damage_event_writer.send(PlayerDamageEvent {
                    entity: player_hit,
                    damage: torpedo_component.damage,
                });
            }
            println!("Torpedo colidiu com o jogador, despawnando torpedo");
            commands.entity(torpedo).despawn();
        }
    }
}



fn move_torpedo_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Torpedo, Option<&RegularTorpedo>, Option<&GuidedTorpedo>, Option<&CounterTorpedo>)>,
    //enemy_torpedo_query: Query<(&Transform, &Torpedo), Without<CounterTorpedo>>, // Query para identificar torpedos inimigos
    enemy_positions: Res<EnemyPositions>,
) {
    for (mut torpedo_transform, torpedo, regular, guided, _counter) in query.iter_mut() {
        // if let Some(_) = counter {
        //     // Lógica específica para CounterTorpedo
        //     let in_range_enemy_torpedos: Vec<&Transform> = enemy_torpedo_query.iter().filter_map(|(enemy_transform, _)| {
        //         let distance = enemy_transform.translation.distance(torpedo_transform.translation);
        //         if distance <= 200.0 { // Distância máxima para considerar um torpedo inimigo como alvo
        //             Some(enemy_transform)
        //         } else {
        //             None
        //         }
        //     }).collect();

        //     if let Some(closest_enemy_torpedo_transform) = in_range_enemy_torpedos.iter().min_by(|a, b| {
        //         let distance_a = a.translation.distance(torpedo_transform.translation);
        //         let distance_b = b.translation.distance(torpedo_transform.translation);
        //         distance_a.partial_cmp(&distance_b).unwrap_or(std::cmp::Ordering::Equal)
        //     }) {
        //         let direction_to_enemy_torpedo = (closest_enemy_torpedo_transform.translation - torpedo_transform.translation).normalize();
        //         torpedo_transform.translation += direction_to_enemy_torpedo * torpedo.movement_speed * time.delta_seconds();
        //     } else {
        //         // Movimentação padrão se não houver torpedos inimigos próximos
        //         let up = torpedo_transform.up();
        //         torpedo_transform.translation += up * torpedo.movement_speed * time.delta_seconds();
        //     }
        // } else if regular.is_some() {
            if regular.is_some() {
            // Lógica para RegularTorpedo
            let up = torpedo_transform.up();
            torpedo_transform.translation += up * torpedo.movement_speed * time.delta_seconds();
        } else if guided.is_some() {
            let in_range_enemies: Vec<&Vec3> = enemy_positions.positions.iter().filter(|&enemy_pos| {
                let distance = enemy_pos.distance(torpedo_transform.translation);
                distance <= 200.0 // Distância máxima
            }).collect();

            if let Some(closest_enemy_position) = in_range_enemies.iter().min_by(|a, b| {
                let distance_a = a.distance(torpedo_transform.translation);
                let distance_b = b.distance(torpedo_transform.translation);
                distance_a.partial_cmp(&distance_b).unwrap_or(std::cmp::Ordering::Equal)
            }) {
                let direction_to_enemy = (**closest_enemy_position - torpedo_transform.translation).normalize();
                torpedo_transform.translation += direction_to_enemy * torpedo.movement_speed * time.delta_seconds();
                
                let angle_to_enemy = Vec3::Y.angle_between(direction_to_enemy);
                let axis_of_rotation = Vec3::Y.cross(direction_to_enemy).normalize_or_zero();
                if axis_of_rotation.length_squared() > 0.0 {
                    let rotation = Quat::from_axis_angle(axis_of_rotation, angle_to_enemy);
                    torpedo_transform.rotation = rotation;
                }
            } else {
                let up = torpedo_transform.up();
                torpedo_transform.translation += up * torpedo.movement_speed * time.delta_seconds();
            }
        }
    }
}

