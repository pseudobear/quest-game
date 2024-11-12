use crate::gameplay::skills::events::ActivateSkillEvent;
use crate::gameplay::GameState;
use bevy::prelude::*;

pub struct DualSwordSkillsPlugin;

impl Plugin for DualSwordSkillsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, 
            (
                ds_activate_basic_attack
            ).run_if(in_state(GameState::Playing))
        );
    }
}
// ToDo: make these systems run after whatever is writing the event to avoid frame gap

fn ds_activate_basic_attack (
    mut ev_activate_skill: EventReader<ActivateSkillEvent>,
) {
    for ev in ev_activate_skill.read() {
        if ev.skill != "basic_attack" {
            continue;
        }
        // get the animatable

        // play the animation
        
        // throw the sensor colliders
        
        // lock the movement state to momentum 

        // emit a EndSkill event when finished
    }
}