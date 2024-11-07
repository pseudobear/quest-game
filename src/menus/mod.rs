pub mod start_menu;
mod button;
use crate::GameState;
use crate::menus::start_menu::StartMenuPlugin;
use bevy::prelude::*;

#[derive(SubStates, Default, Clone, Eq, PartialEq, Debug, Hash)]
#[source(GameState = GameState::Menu)]
enum MenuState {
    #[default]
    StartMenu,
    // no menu plugin engages with this, so no menu will be displayed
    off,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<MenuState>()
           .add_plugins((
                StartMenuPlugin,
           ))
           .add_systems(OnEnter(GameState::Menu), setup_menu)
           .add_systems(OnExit(GameState::Menu), cleanup_menu);
    }
}

fn setup_menu(mut next_state: ResMut<NextState<MenuState>>) {
    next_state.set(MenuState::StartMenu);
}

fn cleanup_menu(mut next_state: ResMut<NextState<MenuState>>) {
    next_state.set(MenuState::off);
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