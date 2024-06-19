use bevy::prelude::*;
use bevy::window::*;
//use bevy::sprite::MaterialMesh2dBundle;
use crate::gamestate::GameState;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), spawn_player)
            .add_systems(Update, player_rotation_system.run_if(in_state(GameState::Game)));
    }
}

#[derive(Component)]
pub struct Player {
    rotation_speed: f32,
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    //mut meshes: ResMut<Assets<Mesh>>,
    //mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let window = windows.single_mut();
    let radius = window.resolution.height() / 2.0;
    let diameter = radius * 2.0; 
    let scale = diameter / 1024.0; 

    commands.spawn(SpriteBundle {
        texture: asset_server.load("../assets/submarino.png"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(0.1*scale)),
        ..default()
    })
    .insert(Player {
        rotation_speed: 1.0,
    });
}

pub fn player_rotation_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
    //mut camera: Query<(&Camera, &mut Transform), Without<Player>>,
) {
    let (player, mut transform) = query.single_mut();

    let mut rotation_factor = 0.0;
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        rotation_factor += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        rotation_factor -= 1.0;
    }

    transform.rotate_z(rotation_factor * player.rotation_speed * time.delta_seconds());
}
