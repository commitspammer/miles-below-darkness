use bevy::prelude::*;
use crate::gamestate::GameState;

pub struct HudPlugin;
impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), spawn_controls_sheet);
    }
}

#[derive(Component)]
struct HudComponent;

fn spawn_controls_sheet(
    mut commands: Commands,
) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px(300.0), // Largura da caixa
                height: Val::Px(150.0), // Altura da caixa
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                padding: UiRect {
                    left: Val::Percent(1.0),
                    top: Val::Percent(1.0),
                    ..default()
                },
                ..default()
            },
            background_color: Color::rgb(0.0, 0.0, 0.0).into(), // Define o fundo como preto
            ..default()
        },
        HudComponent,
    )).with_children(|parent| {
        // Cada TextBundle agora é um filho direto do NodeBundle com fundo preto, centralizado dentro da caixa
        parent.spawn(TextBundle::from_section(
            "INSTRUCTIONS:\nRotate sub: A/D/<-/->\nFire regular: SPACE\nFire guided: SHIFT\nFire counter: CTRL",
            TextStyle {
                font_size: 20.0, // Ajuste o tamanho da fonte conforme necessário
                color: Color::rgb(0.8, 0.0, 0.5),
                ..default()
            },
        ));
    });
}
