use crate::GameState;
use crate::main_menu::ButtonColors;
use crate::main_menu::MainMenuState;
use crate::main_menu::button::*;
use bevy::prelude::*;

pub struct StartMenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainMenuState::StartMenu), setup_menu)
           .add_systems(Update, click_play_button.run_if(in_state(MainMenuState::StartMenu)))
           .add_systems(OnExit(MainMenuState::StartMenu), cleanup_menu);
    }
}

#[derive(Component)]
struct StartMenu;

fn setup_menu(mut commands: Commands) {
    info!("start menu");
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            StartMenu,
        ))
        .with_children(|children| {
            let button_colors = ButtonColors::default();
            children
                .spawn((
                    menu_button(140.0, 50.0),
                    button_colors,
                    ChangeGameState(GameState::Playing),  // attach component to specify interaction with button
                ))
                .with_children(|parent| {
                    parent.spawn(menu_button_text("play"));
                });
        })
        .with_children(|children| {
            let button_colors = ButtonColors::default();
            children
                .spawn((
                    menu_button(140.0, 50.0),
                    button_colors,
                    ChangeMenuState(MainMenuState::OptionsMenu),
                ))
                .with_children(|parent| {
                    parent.spawn(menu_button_text("options"));
                });
        });
}

#[derive(Component)]
struct ChangeGameState(GameState);

#[derive(Component)]
struct ChangeMenuState(MainMenuState);

fn click_play_button(
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_menu_state: ResMut<NextState<MainMenuState>>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &ButtonColors,
            Option<&ChangeGameState>,
            Option<&ChangeMenuState>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, button_colors, change_game_state, change_menu_state) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(state) = change_game_state {
                    next_game_state.set(state.0.clone());
                } 
                if let Some(state) = change_menu_state {
                    next_menu_state.set(state.0.clone());
                } 
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, start_menu: Query<Entity, With<StartMenu>>) {
    for entity in start_menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
}