use bevy::prelude::*;


pub struct StatsPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
    }
}

#[derive(Component, Default)]
pub struct CharacterStats {
    pub health: u32,
    pub mana: u32,
}