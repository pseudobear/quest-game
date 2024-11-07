use crate::loading::TextureAssets;
use crate::GameState;
use crate::menus::ButtonColors;
use crate::menus::button::*;
use bevy::prelude::*;

pub struct StartMenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for StartMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu)
           .add_systems(Update, click_play_button.run_if(in_state(GameState::Menu)))
           .add_systems(OnExit(GameState::Menu), cleanup_menu);
    }
}

#[derive(Component)]
struct StartMenu;

fn setup_menu(mut commands: Commands, textures: Res<TextureAssets>) {
    info!("start menu");
    commands.spawn(Camera2dBundle::default());
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
                    ChangeState(GameState::Playing),  // attach component to specify interaction with button
                ))
                .with_children(|parent| {
                    parent.spawn(menu_button_text("play"));
                });
        });
}

#[derive(Component)]
struct ChangeState(GameState);

fn click_play_button(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &ButtonColors,
            Option<&ChangeState>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, button_colors, change_state) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(state) = change_state {
                    next_state.set(state.0.clone());
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

fn cleanup_menu(mut commands: Commands, start_menus: Query<Entity, With<StartMenu>>) {
    for entity in start_menus.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
