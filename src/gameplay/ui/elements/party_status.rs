use crate::ui::bars::{
    Bar,
    spawn_bar,
};
use bevy::prelude::*;


const STATUS_BAR_LENGTH: f32 = 300.0;
const STATUS_BAR_HEIGHT: f32 = 16.0;
const HEALTH_BAR_COLOR: Color = Color::linear_rgb(1., 0., 0.);
const MANA_BAR_COLOR: Color = Color::linear_rgb(0.2, 0.4, 1.);
const EMPTY_BAR_COLOR: Color = Color::linear_rgba(70., 70., 70., 0.1);

const STATUS_TEXT_SIZE: f32 = 18.0;

#[derive(Component)]
pub struct PartyStatusUi;

#[derive(Component)]
pub struct CharacterStatusUi;

#[derive(Component)]
pub struct HealthBar;

pub fn setup_player_status_group(commands: &mut Commands, parent: Entity) -> Entity {
    let party_status_group = commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                ..default()
            },
            ..default()
        },
        PartyStatusUi,
    )).id();

    commands.entity(parent).add_child(party_status_group);

    return party_status_group;
}

/// TODO: spawn for given player and implement update to sync this to HP once it exists
/// Also need to attach identifier components to each of the entities to allow it to be updated
/// with the correct values and despawned when character leaves party
pub fn spawn_character_status(commands: &mut Commands, parent_node: Entity) {
    let mut health_bar = Bar::new_single(
        Vec2::new(STATUS_BAR_LENGTH, STATUS_BAR_HEIGHT),
        HEALTH_BAR_COLOR,
        EMPTY_BAR_COLOR,
    );
    let mut mana_bar = Bar::new_single(
        Vec2::new(STATUS_BAR_LENGTH, STATUS_BAR_HEIGHT),
        MANA_BAR_COLOR,
        EMPTY_BAR_COLOR,
    );

    health_bar.set_progress(0.7);
    mana_bar.set_progress(0.3);

    let character_name = commands.spawn(
        character_status_text("Character Name", Color::linear_rgb(255., 255., 255.))
    ).id();
    commands.entity(parent_node).add_child(character_name);
    
    spawn_bar(
        commands,
        health_bar,
        parent_node,
        Val::Px(5.0),
        Val::Px(0.0),
    );

    spawn_bar(
        commands,
        mana_bar,
        parent_node,
        Val::Px(5.0),
        Val::Px(0.0),
    );
}

fn character_status_text(text: &str, color: Color) -> TextBundle {
    TextBundle::from_section(
        text,
        TextStyle {
            font_size: STATUS_TEXT_SIZE,
            color: color,
            ..default()
        }
    )
}