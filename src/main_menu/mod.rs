mod start_menu;
mod options;

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
           .add_systems(Update, change_state_on_button_press.run_if(in_state(GameState::Menu)))
           .add_plugins((
                StartMenuPlugin,
                OptionsMenuPlugin,
           ))
           .add_systems(OnExit(GameState::Menu), cleanup_menu);
    }
}

#[derive(Component)]
struct ChangeGameState(GameState);

#[derive(Component)]
struct ChangeMenuState(MainMenuState);

fn change_state_on_button_press(
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_menu_state: ResMut<NextState<MainMenuState>>,
    mut interaction_query: Query<
        (
            &Interaction,
            Option<&ChangeGameState>,
            Option<&ChangeMenuState>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, change_game_state, change_menu_state) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(state) = change_game_state {
                    next_game_state.set(state.0.clone());
                } 
                if let Some(state) = change_menu_state {
                    next_menu_state.set(state.0.clone());
                } 
            }
            _ => {}
        }
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