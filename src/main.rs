use bevy::prelude::*;
use bevy::window::*;
mod sonar;
mod torpedo;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1920., 1080.).with_scale_factor_override(1.0),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_systems(Startup, setup_cam)
        .add_plugins(sonar::SonarPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(torpedo::TorpedoPlugin)
        .add_systems(Update, pause_system.run_if(in_state(GameState::Game)))
        .add_systems(Update, unpause_system.run_if(in_state(GameState::Pause)))
        .add_systems(Update, button_system.run_if(in_state(GameState::Pause)))
        .add_systems(OnEnter(GameState::Pause), spawn_pause_menu)
        .add_systems(OnExit(GameState::Pause), despawn_system::<PauseComponent>)
        .run()
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Game,
    Pause,
}

#[derive(Component)]
struct PauseComponent;

fn setup_cam(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
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
        PauseComponent,
    )).with_children(|parent| {
        parent.spawn(
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
            }
        ).with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Button",
                    TextStyle {
                        //font: asset_server.load("fonts/FiraSans-Bold.ttf"),
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
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
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

fn despawn_system<T: Component>(
    mut commands: Commands,
    query: Query<Entity, With<T>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
