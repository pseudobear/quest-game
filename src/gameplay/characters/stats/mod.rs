use crate::gameplay::ui::elements::party_status::PlayerHealthBar;
use crate::ui::bars::Bar;
use bevy::prelude::*;
use blake2::digest::Update;

pub struct StatsPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sync_health_bar);
    }
}

#[derive(Component, Default)]
pub struct CharacterStats {
    pub health: u32,
    pub mana: u32,
    pub max_health: u32,
}

fn sync_health_bar(
    mut bar_query: Query<&mut Bar, With<PlayerHealthBar>>,
    player_stat_query: Query<&CharacterStats>,
) {
    let player_stat = player_stat_query.single();
    let mut bar = bar_query.single_mut();
    bar.set_progress(player_stat.health as f32 / player_stat.max_health as f32);
}
