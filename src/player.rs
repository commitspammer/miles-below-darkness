use bevy::prelude::*;
//use bevy::window::*;
use bevy::sprite::MaterialMesh2dBundle;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(FixedUpdate, player_rotation_system);
    }
}

#[derive(Component)]
pub struct Player {
    rotation_speed: f32,
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Carregar a textura

// Substituir o MaterialMesh2dBundle por um SpriteBundle
commands.spawn(SpriteBundle {
    texture: asset_server.load("../assets/submarino.png"),
    transform: Transform::from_xyz(0.0, 0.0, 0.0),
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

    //let up = transform.up(); //moves the player forward
    //transform.translation += up * player.rotation_speed * 200. * time.delta_seconds();
    transform.rotate_z(rotation_factor * player.rotation_speed * time.delta_seconds());
}
