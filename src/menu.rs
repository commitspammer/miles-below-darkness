use bevy::prelude::*;
use bevy::app::AppExit;
use crate::gamestate::GameState;
use crate::gamestate::despawn_system;
use bevy::window::*;

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, button_system.run_if(in_state(GameState::Menu)))
            .add_systems(Update, menu_action.run_if(in_state(GameState::Menu)))
            .add_systems(OnEnter(GameState::Menu), spawn_menu)
            .add_systems(OnExit(GameState::Menu), despawn_system::<MenuComponent>);
    }
}

#[derive(Component)]
struct MenuComponent;

#[derive(Component)]
enum MenuButtonAction {
    Play,
    Quit,
}

const NORMAL_BUTTON: Color = Color::rgba(0.15, 0.15, 0.15,0.9);

const BACKGROUND: Color = Color::rgba(0.15, 0.15, 0.15, 0.30);
//const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn spawn_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {

    let window = windows.single_mut();
    let radius = window.resolution.height() / 2.0;
    let background_height = 1024.0; 
    let background_width = 1792.0;

    let painel_center_x = (window.resolution.width() / 2.0) - 960.0;
    let painel_center_y = (window.resolution.height() / 2.0) - 560.0;
    
    
    let scale_factor_x = (window.resolution.width() / background_width) * 1.0; 
    let scale_factor_y = (window.resolution.height() / background_height) * 1.0; 

    
    commands.spawn(SpriteBundle {
        texture: asset_server.load("../assets/menu-inicial.png"),
        transform: Transform {
            translation: Vec3::new(painel_center_x, painel_center_y, -3.0),
            scale: Vec3::new(scale_factor_x, scale_factor_y, 1.0), 
            ..default()
        },
        ..default()
    });


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
        MenuComponent,
    )).with_children(|parent| {
        parent.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(150.0),
                    height: Val::Px(65.0),
                    margin: UiRect {
                        top: Val::Px(-320.0),
                        left: Val::Px(50.0), // Adiciona margem à esquerda
                        ..default()
                    },
                    border: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                
                ..default()
            },
            MenuButtonAction::Play
        )).with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Play",
                    TextStyle {
                        //font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgba(0.5, 0.0, 0.5, 0.5),
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
                    margin: UiRect {
                        top: Val::Px(-320.0),
                        left: Val::Px(0.0), // Adiciona margem à esquerda
                        ..default()
                    },
                    border: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: NORMAL_BUTTON.into(),
                ..default()
            },
            MenuButtonAction::Quit
        )).with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Quit",
                    TextStyle {
                        //font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgba(0.5, 0.0, 0.5, 0.5),
                        ..default()
                    },
                )
            );
        });
    });
}

fn menu_action(
    interaction_query: Query<(&Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>)>,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match action {
                MenuButtonAction::Quit => {
                    app_exit_events.send(AppExit);
                }
                MenuButtonAction::Play => {
                    game_state.set(GameState::Loading);
                }
            }
        }
    }
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
