use bevy::prelude::*;
use bevy::app::AppExit;
use crate::gamestate::GameState;
use crate::gamestate::despawn_system;
use crate::gamestate::PauseDespawnable;

pub struct PausePlugin;
impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, pause_system.run_if(in_state(GameState::Game)))
            .add_systems(Update, unpause_system.run_if(in_state(GameState::Pause)))
            .add_systems(Update, pause_menu_action.run_if(in_state(GameState::Pause)))
            .add_systems(Update, button_system.run_if(in_state(GameState::Pause)))
            .add_systems(OnEnter(GameState::Pause), spawn_pause_menu)
            .add_systems(OnExit(GameState::Pause), despawn_system::<PauseDespawnable>);
    }
}

#[derive(Component)]
enum PauseButtonAction {
    Unpause,
    Menu,
    Quit,
}

const BACKGROUND: Color = Color::rgba(0.15, 0.15, 0.15, 0.30);
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn spawn_pause_menu(
    mut commands: Commands,
) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: BACKGROUND.into(),
            ..default()
        },
        PauseDespawnable,
    )).with_children(|parent| {
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(150.0),
                    height: Val::Px(65.0),
                    border: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                background_color: NORMAL_BUTTON.into(),
                ..default()
            },
            PauseButtonAction::Unpause,
        )).with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Back",
                    TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                )
            );
        });
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(150.0),
                    height: Val::Px(65.0),
                    border: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                background_color: NORMAL_BUTTON.into(),
                ..default()
            },
            PauseButtonAction::Menu,
        )).with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Menu",
                    TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                )
            );
        });
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(150.0),
                    height: Val::Px(65.0),
                    border: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                background_color: NORMAL_BUTTON.into(),
                ..default()
            },
            PauseButtonAction::Quit,
        )).with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Quit",
                    TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                )
            );
        });
    });
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn pause_menu_action(
    interaction_query: Query<(&Interaction, &PauseButtonAction), (Changed<Interaction>, With<Button>)>,
    mut app_exit_events: EventWriter<AppExit>,
    mut time: ResMut<Time<Virtual>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match action {
                PauseButtonAction::Unpause => {
                    game_state.set(GameState::Game);
                    time.unpause();
                }
                PauseButtonAction::Quit => {
                    app_exit_events.send(AppExit);
                }
                PauseButtonAction::Menu => {
                    game_state.set(GameState::Menu);
                }
            }
        }
    }
}

fn pause_system(
    mut time: ResMut<Time<Virtual>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        game_state.set(GameState::Pause);
        time.pause();
    }
}

fn unpause_system(
    mut time: ResMut<Time<Virtual>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        game_state.set(GameState::Game);
        time.unpause();
    }
}
