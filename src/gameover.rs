use bevy::prelude::*;
use crate::gamestate::GameState;
use crate::gamestate::despawn_system;

pub struct GameOverPlugin;
impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), setup_game_over)
           .add_systems(OnExit(GameState::GameOver), despawn_system::<GameOverComponent>);
    }
}

#[derive(Component)]
struct GameOverComponent;

fn setup_game_over(
    mut commands: Commands,
) {
    // Configura o fundo azul escuro
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        background_color: Color::rgb(0.1, 0.2, 0.3).into(), // Cor de fundo desejada
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text::from_section(
                "GameOver",
                TextStyle {
                    //font: asset_server.load("caminho/para/sua/fonte.ttf"), // Certifique-se de carregar a fonte corretamente
                    font_size: 60.0,
                    color: Color::rgba(1.0, 0.0, 0.0, 1.0),
                    ..default()
                },
            ),
            // Ajuste a posição se necessário, mas como está dentro de um NodeBundle com alinhamento central, pode não ser necessário.
            ..Default::default()
        });
    });
}

// fn cleanup_game_over(mut commands: Commands, query: Query<Entity, With<GameOverScreen>>) {
//     // Remove todos os elementos da tela de GameOver
//     for entity in query.iter() {
//         commands.entity(entity).despawn();
//     }
// }
