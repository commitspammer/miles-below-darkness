use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Menu,
    Loading,
    Game,
    Pause,
    GameOver
}

#[derive(Component)]
pub struct GameDespawnable;

#[derive(Component)]
pub struct GameOverDespawnable;

#[derive(Component)]
pub struct MenuDespawnable;

#[derive(Component)]
pub struct PauseDespawnable;

#[derive(Component)]
pub struct LoadingDespawnable;

pub fn despawn_system<T: Component>(
    mut commands: Commands,
    query: Query<Entity, With<T>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
