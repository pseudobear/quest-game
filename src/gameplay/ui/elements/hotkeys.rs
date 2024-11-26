use crate::ui::buttons::{
    ButtonColors,
    ui_button_with_margin,
};
use bevy::prelude::*;


const HOTKEY_SIZE_PX: f32 = 40.0;
const HOTKEY_MARGIN_PX: f32 = 5.0;
pub const HOTKEYS_PER_ROW: u8 = 6;

const HOTKEY_TEXT_SIZE: f32 = 12.0;


#[derive(Component)]
pub struct HotkeyGroupUi;

#[derive(Component)]
pub struct TopHotkeyRow;

#[derive(Component)]
pub struct BotHotkeyRow;

#[derive(Component)]
pub struct HotKeyUi;

/// Spawns a hotkey group under parent and returns the top hotkey row and bottom hotkey row entities
pub fn spawn_hotkey_group(commands: &mut Commands, parent: Entity) -> (Entity, Entity) {
    let hotkey_group = commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px((HOTKEY_SIZE_PX + HOTKEY_MARGIN_PX) * HOTKEYS_PER_ROW as f32),
                height: Val::Px((HOTKEY_SIZE_PX + HOTKEY_MARGIN_PX * 2.0) * 2.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                ..default()
            },
            background_color: Color::linear_rgba(0.8, 0.8, 0.8, 0.4).into(),
            border_radius: BorderRadius::all(Val::Px(5.)),
            ..default()
        },
        HotkeyGroupUi,
    )).id();

    let top_hotkey_row = commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px((HOTKEY_SIZE_PX + HOTKEY_MARGIN_PX) * HOTKEYS_PER_ROW as f32),
                height: Val::Px((HOTKEY_SIZE_PX + HOTKEY_MARGIN_PX) * 2.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                ..default()
            },
            ..default()
        },
        TopHotkeyRow,
    )).id();

    let bot_hotkey_row = commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px((HOTKEY_SIZE_PX + HOTKEY_MARGIN_PX) * HOTKEYS_PER_ROW as f32),
                height: Val::Px((HOTKEY_SIZE_PX + HOTKEY_MARGIN_PX) * 2.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                ..default()
            },
            ..default()
        },
        BotHotkeyRow,
    )).id();

    commands.entity(parent).add_child(hotkey_group);
    commands.entity(hotkey_group).push_children(&[top_hotkey_row, bot_hotkey_row]);

    return (top_hotkey_row, bot_hotkey_row);
}

pub fn spawn_hotkey(commands: &mut Commands, parent: Entity) {
    let hotkey = commands.spawn((
        ui_button_with_margin(HOTKEY_SIZE_PX, HOTKEY_SIZE_PX, HOTKEY_MARGIN_PX),
        ButtonColors::default(),
    )).id();

    commands.entity(parent).add_child(hotkey);
}