use bevy::prelude::*;
use bevy::window::*;
use crate::gamestate::GameState;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), spawn_player)
            .add_systems(Update, player_rotation_system.run_if(in_state(GameState::Game)));
    }
}

#[derive(Component)]
pub struct Player {
    rotation_speed: f32,
    rotation_acceleration: f32,
    terminal_rotation_speed: f32,
    turbine_power: f32,
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let window = windows.single_mut();
    let radius = window.resolution.height() / 2.0;
    let diameter = radius * 2.0; 
    let scale = diameter / 1024.0; 
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("../assets/submarino.png"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(0.1*scale)),
            ..default()
        },
        Player {
            rotation_speed: 0.0,
            rotation_acceleration: 0.7,
            terminal_rotation_speed: 0.7,
            turbine_power: 1.5,
        }
    ));
}

pub fn player_rotation_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Player, &mut Transform)>,
    //mut camera: Query<(&Camera, &mut Transform), Without<Player>>,
) {
    let (mut player, mut transform) = query.single_mut();

    let mut rotation_factor = 0.0;
    if player.rotation_speed != 0.0 {
        rotation_factor = -player.rotation_speed.signum();
    }

    if keyboard_input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        rotation_factor += player.turbine_power;
    } else if keyboard_input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        rotation_factor -= player.turbine_power;
    } else if player.rotation_speed.abs() < 0.1 {
        player.rotation_speed = 0.0;
    }

    if player.rotation_speed.abs() > player.terminal_rotation_speed {
        player.rotation_speed = player.terminal_rotation_speed * player.rotation_speed.signum();
    }
    transform.rotate_z(player.rotation_speed * time.delta_seconds());
    player.rotation_speed += rotation_factor * player.rotation_acceleration * time.delta_seconds();
}
