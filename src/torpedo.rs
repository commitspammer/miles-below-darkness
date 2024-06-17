use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use crate::player::Player;

pub struct TorpedoPlugin;
impl Plugin for TorpedoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_torpedo_system)
            .add_systems(Update, move_torpedo_system);
    }
}

#[derive(Component)]
pub struct Torpedo {
    movement_speed: f32,
    //...
}

pub fn spawn_torpedo_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut player_query: Query<(&Player, &mut Transform)>,
    //query player...
) {
    //logic...
    let (player, mut player_transform) = player_query.single_mut();
    if keyboard_input.pressed(KeyCode::Space) {
        let shape = meshes.add(Capsule2d::new(3.0, 25.0)).into();
        commands.spawn((
            MaterialMesh2dBundle {
                mesh:shape,
                material: materials.add(Color::GREEN),
                transform: Transform {
                    translation: player_transform.translation,
                    rotation: player_transform.rotation,
                    scale: Vec3::splat(0.5),
                    //..default()
                },
                ..default()
            },
            Torpedo {
                movement_speed: 100.0,
            }
        ));
    }
}

pub fn move_torpedo_system(
    time: Res<Time>,
    mut player_query: Query<(&Player, &mut Transform)>,
    mut torpedos_query: Query<(&Torpedo, &mut Transform), Without<Player>>
) {
    let (player, mut player_transform) = player_query.single_mut();
    for (torpedo, mut torpedo_transform) in torpedos_query.iter_mut() {
        let up = torpedo_transform.up();
        torpedo_transform.translation += up * torpedo.movement_speed * time.delta_seconds();
    }
}

