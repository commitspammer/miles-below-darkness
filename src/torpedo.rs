use bevy::prelude::*;
use crate::gamestate::GameState;
use crate::player::Player;
use std::time::Duration;
use std::f32::consts::PI;
use crate::hitbox::Hitbox;
use crate::hitbox::Collision;
use crate::enemy::Enemy;
use bevy::ecs::system::{Commands, Query};
use bevy::math::Vec3;
use bevy::ecs::query::With;


pub struct TorpedoPlugin;
impl Plugin for TorpedoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, shoot_torpedo_system.run_if(in_state(GameState::Game)))
            .add_systems(Update, move_torpedo_system.run_if(in_state(GameState::Game)))
            .add_systems(Update, collide_system.run_if(in_state(GameState::Game)))
            .insert_resource(TorpedoCooldown(Timer::new(Duration::from_secs(2), TimerMode::Once)));
    }
}

#[derive(Component)]
pub struct Torpedo {
    movement_speed: f32,
}

#[derive(Component)]
pub struct RegularTorpedo;

#[derive(Component)]
pub struct GuidedTorpedo;

#[derive(Component)]
pub struct CounterTorpedo;

#[derive(Resource, Deref, DerefMut)]
pub struct TorpedoCooldown(Timer);

pub fn shoot_torpedo_system(
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
                movement_speed: 50.0,
            },
            RegularTorpedo,
        ))
        .insert(Hitbox::new(10.0, 50.0));
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
                movement_speed: 50.0,
            },
            GuidedTorpedo,
        ))
        .insert(Hitbox::new(15.0, 60.0));
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
                    movement_speed: 25.0,
                },
                CounterTorpedo,
            ))
            .insert(Hitbox::new(5.0, 25.0));
        }
        cooldown_timer.reset();
    }
}

 // Substitua `your_enemy_module` pelo caminho do módulo onde `Enemy` está definido

// Componentes Torpedo já definidos em torpedo.rs

fn collide_system(
    mut commands: Commands,
    mut event_reader: EventReader<Collision>,
    torpedo_query: Query<(Entity, &Hitbox),With<Torpedo>>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    for event in event_reader.read() {
        let entity_a = event.entity_a;
        let entity_b = event.entity_b;

        // Verifica se a entidade A é um torpedo e a entidade B é um inimigo, ou vice-versa
        let torpedo_entity = if torpedo_query.get_component::<Hitbox>(entity_a).is_ok() && enemy_query.get(entity_b).is_ok() {
            Some(entity_a)
        } else if torpedo_query.get_component::<Hitbox>(entity_b).is_ok() && enemy_query.get(entity_a).is_ok() {
            Some(entity_b)
        } else {
            None
        };

        if let Some(torpedo) = torpedo_entity {
            // Log para depuração
            println!("Torpedo colidiu com inimigo, despawnando torpedo");
            // Despawna o torpedo
            commands.entity(torpedo).despawn();
        }
    }
}


fn move_torpedo_system(
    time: Res<Time>,
    mut torpedos_query: Query<(&Torpedo, &mut Transform)>,
    //enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (torpedo, mut torpedo_transform) in torpedos_query.iter_mut() {
        // // Supondo que você queira ajustar o movimento do torpedo baseado na proximidade de um inimigo
        // // Encontra o inimigo mais próximo
        // if let Some((_, enemy_transform)) = enemy_query.iter().min_by(|a, b| {
        //     let distance_a = a.1.translation.distance(torpedo_transform.translation);
        //     let distance_b = b.1.translation.distance(torpedo_transform.translation);
        //     distance_a.partial_cmp(&distance_b).unwrap_or(std::cmp::Ordering::Equal)
        // }) {
        //     // Aqui você pode ajustar o movimento do torpedo baseado na posição do inimigo mais próximo
        //     // Exemplo: movendo o torpedo na direção do inimigo
        //     let direction_to_enemy = (enemy_transform.translation - torpedo_transform.translation).normalize();
        //     torpedo_transform.translation += direction_to_enemy * torpedo.movement_speed * time.delta_seconds();
        // } else {
            // Movimento padrão do torpedo se nenhum inimigo estiver próximo
            let up = torpedo_transform.up();
            torpedo_transform.translation += up * torpedo.movement_speed * time.delta_seconds();
        // }
    }
}

