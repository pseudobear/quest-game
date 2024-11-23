use bevy::prelude::*;
use crate::gameplay::characters::player;
use crate::gameplay::GameState;
use crate::gameplay::ui::setup_game_ui;
use crate::gameplay::ui::elements::party_status::{
    setup_player_status_group
};
use crate::ui::buttons::{
    ui_button,
    ButtonColors,
};

use super::elements::party_status::spawn_character_status;


#[derive(Component)]
pub struct GameUiTopBar;

#[derive(Component)]
pub struct GameUiTopBarLeft;

#[derive(Component)]
pub struct GameUiTopBarCenter;

#[derive(Component)]
pub struct GameUiTopBarRight;


pub struct TopBarPlugin;

// This plugin is responsible to control the game audio
impl Plugin for TopBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_top_bar.after(setup_game_ui));
        app.add_systems(OnExit(GameState::Playing), cleanup_top_bar);
    }
}

fn setup_top_bar(mut commands: Commands, top_bar: Query<Entity, With<GameUiTopBar>>) {
    let left_group = commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(25.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                ..default()
            },
            ..default()
        },
        GameUiTopBarLeft,
    )).id();
    let center_group = commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(25.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Start,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
            ..default()
        },
        GameUiTopBarCenter,
    )).id();
    let right_group = commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(25.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Start,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        },
        GameUiTopBarRight,
    )).id();
    commands.entity(top_bar.single()).push_children(&[left_group, center_group, right_group]);

    populate_left_group(&mut commands, left_group);
    populate_center_group(&mut commands, center_group);
    populate_right_group(&mut commands, right_group);
}

fn populate_left_group(mut commands: &mut Commands, left_group: Entity) {
    let player_status_group = setup_player_status_group(&mut commands, left_group);
    spawn_character_status(&mut commands, player_status_group);
}

fn populate_center_group(commands: &mut Commands, center_group: Entity) {
    let test_button = commands.spawn((
        ui_button(140.0, 50.0),
        ButtonColors::default(),
    )).id();
    commands.entity(center_group).push_children(&[test_button]);
}

fn populate_right_group(commands: &mut Commands, right_group: Entity) {
    let test_button = commands.spawn((
        ui_button(140.0, 50.0),
        ButtonColors::default(),
    )).id();
    let test_button_left = commands.spawn((
        ui_button(140.0, 50.0),
        ButtonColors::default(),
    )).id();
    commands.entity(right_group).push_children(&[test_button, test_button_left]);
}

fn cleanup_top_bar(mut commands: Commands, top_bar: Query<Entity, With<GameUiTopBar>>) {
    for entity in top_bar.iter() {
        commands.entity(entity).despawn_recursive();
    }
}