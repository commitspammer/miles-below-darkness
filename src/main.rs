use bevy::prelude::*;
use bevy::window::*;
mod gamestate;
mod sonar;
mod torpedo;
mod player;
mod pause;
mod enemy;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1920., 1080.).with_scale_factor_override(1.0),
                ..default()
            }),
            ..default()
        }))
        .init_state::<gamestate::GameState>()
        .add_systems(Startup, setup_cam)
        .add_plugins(sonar::SonarPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(torpedo::TorpedoPlugin)
        .add_plugins(enemy::EnemyPlugin)
        .add_plugins(pause::PausePlugin)
        .run()
}

fn setup_cam(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}
