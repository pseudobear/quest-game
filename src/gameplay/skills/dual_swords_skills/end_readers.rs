use crate::gameplay::skills::events::EndSkillEvent;
use crate::gameplay::skills::SkillCooldown;
use bevy::prelude::*;

pub fn ds_end_basic_attack (
    mut commands: Commands,
    mut ev_end_skill: EventReader<EndSkillEvent>,
) {
    for ev in ev_end_skill.read() {
        if ev.skill != "ds_basic_attack" {
            continue;
        }
        commands.entity(ev.sprite_entity).insert(
            SkillCooldown::new("ds_basic_attack", 1.0)
        );
    }
}