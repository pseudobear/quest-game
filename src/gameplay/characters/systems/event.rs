use crate::gameplay::skills::events::ActivateSkillEvent;
use crate::gameplay::characters::components::{
    CharacterPhysics,
    CharacterSprite,
};
use bevy::prelude::*;


pub fn emit_ds_skill_activation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut ev_activate_skill: EventWriter<ActivateSkillEvent>,
    player_entity_query: Query<Entity, With<CharacterPhysics>>,
    player_sprite_query: Query<Entity, With<CharacterSprite>>,
) {
    for player_entity in player_entity_query.iter() {
        for player_sprite in player_sprite_query.iter() {
            if keyboard_input.just_pressed(KeyCode::KeyZ) {
                ev_activate_skill.send(ActivateSkillEvent {
                    skill: "ds_basic_attack".to_string(),
                    entity: player_entity,
                    sprite_entity: player_sprite,
                    animation_index: 7,
                    animation_lock: true,
                    hitbox_index: 0,
                    hitbox_lock: true,
                });
            }
        }
    }
}