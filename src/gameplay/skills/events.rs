use bevy::prelude::*;


#[derive(Event)]
pub struct ActivateSkillEvent {
    pub entity: Entity,
    pub sprite_entity: Entity, 
    pub skill: String,
    pub animation_index: usize,
    pub animation_lock: bool
}

#[derive(Event)]
pub struct EndSkillEvent {
    pub entity: Entity,
    pub skill: String,
}