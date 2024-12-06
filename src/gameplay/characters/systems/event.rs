use crate::gameplay::skills::events::ActivateSkillEvent;
use crate::gameplay::items::CharacterEquips;
use crate::gameplay::items::weapons::WeaponType;
use bevy::prelude::*;


pub fn emit_ds_skill_activation<SourcePhysics: Component, SourceSprite: Component>(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut ev_activate_skill: EventWriter<ActivateSkillEvent>,
    physics_entity_query: Query<Entity, With<SourcePhysics>>,
    sprite_entity_query: Query<(Entity, &CharacterEquips), With<SourceSprite>>,
) {
    // iterate in case the player controls multiple
    for physics_entity in physics_entity_query.iter() {
        for (sprite_entity, equips) in sprite_entity_query.iter() {
            if keyboard_input.just_pressed(KeyCode::KeyZ) && equips.weapon.weapon_type == WeaponType::DualSwords {
                ev_activate_skill.send(ActivateSkillEvent {
                    skill: "ds_basic_attack".to_string(),
                    physics_entity: physics_entity,
                    sprite_entity: sprite_entity,
                    animation_index: 7,
                    animation_lock: true,
                    hitbox_index: 0,
                    hitbox_lock: true,
                });
            }
        }
    }
}