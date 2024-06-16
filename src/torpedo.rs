use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

pub struct TorpedoPlugin;
impl Plugin for TorpedoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_torpedo)
            .add_systems(Update, move_torpedo_system);
    }
}

#[derive(Component)]
pub struct Torpedo {
    movement_speed: f32,
    //...
}

pub fn spawn_torpedo(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    //query player...
) {
    //logic...
}

pub fn move_torpedo_system(
    time: Res<Time>,
    //query torpedos...
) {
    //logic...
}
