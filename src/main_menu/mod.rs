mod start_menu;
mod options;
mod buttons;

use crate::GameState;
use crate::main_menu::start_menu::StartMenuPlugin;
use crate::main_menu::options::OptionsMenuPlugin;
use bevy::prelude::*;

#[derive(SubStates, Default, Clone, Eq, PartialEq, Debug, Hash)]
#[source(GameState = GameState::Menu)]
enum MainMenuState {
    #[default]
    StartMenu,
    OptionsMenu,
}

#[derive(Component)]
struct MainMenu;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<MainMenuState>()
           .add_systems(OnEnter(GameState::Menu), setup_menu)
           .add_plugins((
                StartMenuPlugin,
                OptionsMenuPlugin,
           ))
           .add_systems(OnExit(GameState::Menu), cleanup_menu);
    }
}

fn setup_menu(mut commands: Commands) {
    info!("main menu");
    commands.spawn((Camera2dBundle::default(), MainMenu));
}

fn cleanup_menu(mut commands: Commands, menu: Query<Entity, With<MainMenu>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
}