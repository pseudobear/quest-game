use crate::ui::bars::Bar;
use crate::gameplay::ui::elements::party_status::PlayerHealthBar;
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
    pub max_health: u32,
}

fn sync_health_bar(
    mut query: Query<&Bar, With<PlayerHealthBar>>
) {
    for bar in query.iter_mut() {
    }
}