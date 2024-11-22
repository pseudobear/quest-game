use crate::ui::bars::{
    Bar,
    spawn_bar,
};
use bevy::prelude::*;


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
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        },
        PartyStatusUi,
    )).id();

    commands.entity(parent).add_child(party_status_group);

    return party_status_group;
}

pub fn spawn_character_status(commands: &mut Commands, parent_node: Entity) {
    let mut health_bar = Bar::new_single(
        Vec2::new(300.0, 50.0),
        Color::linear_rgb(255., 0., 0.),
        Color::linear_rgb(10., 10., 200.),
    );

    health_bar.add_section(5, Color::linear_rgb(0.,0.,255.));
    health_bar.set_progress(0.95);
    

    spawn_bar(commands, health_bar, parent_node);
}
