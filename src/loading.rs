use bevy::prelude::*;
use iyes_progress::{prelude::*, dummy_system_wait_millis};
use crate::gamestate::GameState;
use crate::gamestate::despawn_system;
use crate::gamestate::LoadingDespawnable;

pub struct LoadingPlugin;
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ProgressPlugin::new(GameState::Loading).continue_to(GameState::Game))
            .add_systems(OnEnter(GameState::Loading), spawn_loading_screen)
            .add_systems(Update, (
                    dummy_system_wait_millis::<500>.track_progress(),
                    progress_percent_system.after(TrackedProgressSet),
                ).run_if(in_state(GameState::Loading))
            )
            .add_systems(OnExit(GameState::Loading), despawn_system::<LoadingDespawnable>);
    }
}

#[derive(Component)]
struct ProgressPercent;

const BACKGROUND: Color = Color::rgba(0.15, 0.15, 0.15, 0.30);

fn spawn_loading_screen(
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
        LoadingDespawnable,
    )).with_children(|parent| {
        parent.spawn((
            TextBundle::from_section(
                "Loading",
                TextStyle {
                    font_size: 60.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ),
            ProgressPercent,
        ));
    });
}

fn progress_percent_system(
    counter: Res<ProgressCounter>,
    mut text_query: Query<&mut Text, With<ProgressPercent>>,
) {
    let progress = counter.progress();
    let mut percent: f32 = progress.into();
    if percent.is_nan() {
        percent = 0.0;
    }
    let mut text = text_query.single_mut();
    text.sections[0].value = format!("Loading {:.0}%", percent);
}
