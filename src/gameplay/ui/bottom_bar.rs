use bevy::prelude::*;
use crate::gameplay::GameState;
use crate::gameplay::ui::setup_game_ui;
use crate::gameplay::ui::button::{
    ui_button,
    ButtonColors,
    ui_button_text,
};


#[derive(Component)]
pub struct GameUiBottomBar;

#[derive(Component)]
pub struct GameUiBottomBarLeft;

#[derive(Component)]
pub struct GameUiBottomBarCenter;

#[derive(Component)]
pub struct GameUiBottomBarRight;


pub struct BottomBarPlugin;

// This plugin is responsible to control the game audio
impl Plugin for BottomBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_bottom_bar.after(setup_game_ui));
        app.add_systems(OnExit(GameState::Playing), cleanup_bottom_bar);
    }
}

fn setup_bottom_bar(mut commands: Commands, bottom_bar: Query<Entity, With<GameUiBottomBar>>) {
    let left_group = commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(25.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::End,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        },
        GameUiBottomBarLeft,
    )).id();
    let center_group = commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(25.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::End,
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        },
        GameUiBottomBarCenter,
    )).id();
    let right_group = commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(25.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::End,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        },
        GameUiBottomBarRight,
    )).id();
    commands.entity(bottom_bar.single()).push_children(&[left_group, center_group, right_group]);

    populate_left_group(&mut commands, left_group);
    populate_center_group(&mut commands, center_group);
    populate_right_group(&mut commands, right_group);
}

fn populate_left_group(commands: &mut Commands, left_group: Entity) {
    let test_button = commands.spawn((
        ui_button(140.0, 50.0),
        ButtonColors::default(),
    )).id();
    let test_button_left = commands.spawn((
        ui_button(140.0, 50.0),
        ButtonColors::default(),
    )).id();
    commands.entity(left_group).push_children(&[test_button, test_button_left]);
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

fn cleanup_bottom_bar(mut commands: Commands, bottom_bar: Query<Entity, With<GameUiBottomBar>>) {
    for entity in bottom_bar.iter() {
        commands.entity(entity).despawn_recursive();
    }
}