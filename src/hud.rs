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
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                padding: UiRect {
                    left: Val::Percent(1.0),
                    top: Val::Percent(1.0),
                    ..default()
                },
                ..default()
            },
            ..default()
        },
        HudComponent,
    )).with_children(|parent| {
        parent.spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
        )).with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Rotate sub: A/D/<-/->",
                    TextStyle {
                        font_size: 30.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ),
            ));
            parent.spawn((
                TextBundle::from_section(
                    "Fire regular: SPACE",
                    TextStyle {
                        font_size: 30.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ),
            ));
            parent.spawn((
                TextBundle::from_section(
                    "Fire guided: SHIFT",
                    TextStyle {
                        font_size: 30.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ),
            ));
            parent.spawn((
                TextBundle::from_section(
                    "Fire counter: CTRL",
                    TextStyle {
                        font_size: 30.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ),
            ));
        });
    });
}
