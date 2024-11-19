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

// TODO: build a progress bar bundle, 
pub fn spawn_character_status(commands: &mut Commands, parent_node: Entity) {

}
