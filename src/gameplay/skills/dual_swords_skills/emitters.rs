use std::collections::HashMap;

use crate::gameplay::skills::events::ActivateSkillEvent;
use crate::gameplay::skills::SkillCooldowns;
use crate::gameplay::items::CharacterEquips;
use crate::gameplay::items::weapons::WeaponType;
use bevy::prelude::*;


pub fn emit_ds_skill_activation<SourcePhysics: Component, SourceSprite: Component>(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut ev_activate_skill: EventWriter<ActivateSkillEvent>,
    physics_entity_query: Query<Entity, With<SourcePhysics>>,
    mut sprite_entity_query: Query<(Entity, &CharacterEquips, &mut SkillCooldowns), With<SourceSprite>>,
) {
    // iterate in case the player controls multiple
    for physics_entity in physics_entity_query.iter() {
        for (sprite_entity, equips, mut cooldowns) in &mut sprite_entity_query {
            if equips.weapon.weapon_type != WeaponType::DualSwords {
                continue;
            }

            if keyboard_input.just_pressed(KeyCode::KeyZ) {
                basic_attack_chain(&mut cooldowns, &mut ev_activate_skill, physics_entity, sprite_entity);
            }
        }
    }
}

/// use this to activate basic attacks. This function handles the chain logic for determining which 
/// version of the basic attack triggers and clears the cooldowns accordingly
fn basic_attack_chain(
    cooldowns: &mut SkillCooldowns,
    ev_activate_skill: &mut EventWriter<ActivateSkillEvent>,
    physics_entity: Entity,
    sprite_entity: Entity
) {
    if cooldowns.contains_key("ds_basic_attack_3") {
        return;
    } else if cooldowns.contains_key("ds_basic_attack_2") {
        ev_activate_skill.send(ActivateSkillEvent {
            skill: "ds_basic_attack_3".to_string(),
            physics_entity: physics_entity,
            sprite_entity: sprite_entity,
            animation_index: 7,
            animation_lock: true,
            hitbox_index: 0,
            hitbox_lock: true,
        });
        cooldowns.remove("ds_basic_attack_2");

    } else if cooldowns.contains_key("ds_basic_attack_1") {
        ev_activate_skill.send(ActivateSkillEvent {
            skill: "ds_basic_attack_2".to_string(),
            physics_entity: physics_entity,
            sprite_entity: sprite_entity,
            animation_index: 7,
            animation_lock: true,
            hitbox_index: 0,
            hitbox_lock: true,
        });
        cooldowns.remove("ds_basic_attack_1");

    } else {
        ev_activate_skill.send(ActivateSkillEvent {
            skill: "ds_basic_attack_1".to_string(),
            physics_entity: physics_entity,
            sprite_entity: sprite_entity,
            animation_index: 7,
            animation_lock: true,
            hitbox_index: 0,
            hitbox_lock: true,
        });
    }
}