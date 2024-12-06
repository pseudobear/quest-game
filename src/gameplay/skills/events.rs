use bevy::prelude::*;


#[derive(Event)]
pub struct ActivateSkillEvent {
    pub physics_entity: Entity,
    pub sprite_entity: Entity, 
    pub skill: String,
    pub animation_index: usize,
    pub animation_lock: bool,
    pub hitbox_index: usize,
    pub hitbox_lock: bool,
}

#[derive(Event)]
pub struct EndSkillEvent {
    pub physics_entity: Entity,
    pub sprite_entity: Entity,
    pub skill: String,
}