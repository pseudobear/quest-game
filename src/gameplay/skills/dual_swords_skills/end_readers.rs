use crate::gameplay::skills::events::EndSkillEvent;
use crate::gameplay::skills::SkillCooldowns;
use bevy::prelude::*;

pub fn ds_end_basic_attack (
    mut ev_end_skill: EventReader<EndSkillEvent>,
    mut skill_cooldowns_query: Query<&mut SkillCooldowns>
) {
    for ev in ev_end_skill.read() {
        if ev.skill != "ds_basic_attack" {
            continue;
        }
        skill_cooldowns_query.get_mut(ev.sprite_entity).unwrap().add_cooldown("ds_basic_attack", 1.0);
    }
}