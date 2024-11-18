mod button;
mod top_bar;
mod bottom_bar;

use crate::gameplay::ui::bottom_bar::{ GameUiBottomBar, BottomBarPlugin};
use crate::gameplay::ui::top_bar::{ GameUiTopBar, TopBarPlugin };
use crate::gameplay::GameState;
use bevy::prelude::*;


#[derive(Component)]
struct GameUi;

pub struct GameUiPlugin;

// This plugin is responsible to control the game audio
impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((TopBarPlugin, BottomBarPlugin))
           .add_systems(OnEnter(GameState::Playing), setup_game_ui)
           .add_systems(OnExit(GameState::Playing), cleanup_game_ui);
    }
}

fn setup_game_ui(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::End,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        },
        GameUi,
        GameUiTopBar,
    ));
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        },
        GameUi,
        GameUiBottomBar,
    ));
}

fn cleanup_game_ui(mut commands: Commands, game_ui: Query<Entity, With<GameUi>>) {
    for entity in game_ui.iter() {
        commands.entity(entity).despawn_recursive();
    }
}