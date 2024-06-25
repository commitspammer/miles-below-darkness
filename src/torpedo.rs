use bevy::prelude::*;
use crate::gamestate::GameState;
use crate::player::Player;
use std::time::Duration;
use std::f32::consts::PI;

pub struct TorpedoPlugin;
impl Plugin for TorpedoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, shoot_torpedo_system.run_if(in_state(GameState::Game)))
            .add_systems(Update, move_torpedo_system.run_if(in_state(GameState::Game)))
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
                movement_speed: 50.0,
            },
            GuidedTorpedo,
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
                    movement_speed: 25.0,
                },
                CounterTorpedo,
            ));
        }
        cooldown_timer.reset();
    }
}

pub fn move_torpedo_system(
    time: Res<Time>,
    mut torpedos_query: Query<(&Torpedo, &mut Transform)>
) {
    for (torpedo, mut torpedo_transform) in torpedos_query.iter_mut() {
        let up = torpedo_transform.up();
        torpedo_transform.translation += up * torpedo.movement_speed * time.delta_seconds();
    }
}

