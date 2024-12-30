use crate::gameplay::ui::elements::party_status::PlayerHealthBar;
use crate::ui::bars::Bar;
use bevy::prelude::*;

pub struct StatsPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component, Default)]
pub struct CharacterStats {
    pub health: u32,
    pub mana: u32,
    pub max_health: u32,
}

fn sync_health_bar(
    mut bar_query: Query<&Bar, With<PlayerHealthBar>>,
    mut player_stat_query: Query<&CharacterStats>,
) {
    let mut updated_health = 0.0;
    for stats in player_stat_query.iter_mut() {
        updated_health = (stats.health as f32 / stats.max_health as f32);
    }

    for mut bar in bar_query.iter_mut() {
        bar.set_progress(updated_health);
    }
}

