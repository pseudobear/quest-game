mod start_menu;
mod options;
mod button;

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
           .add_plugins((
                StartMenuPlugin,
                OptionsMenuPlugin,
           ))
           .add_systems(OnEnter(GameState::Menu), setup_menu)
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


#[derive(Component)]
struct ButtonColors {
    normal: Color,
    hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::linear_rgb(-1.15, 0.15, 0.15),
            hovered: Color::linear_rgb(-1.25, 0.25, 0.25),
        }
    }
}