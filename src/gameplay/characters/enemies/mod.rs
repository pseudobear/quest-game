pub mod test_enemy;

use crate::gameplay::GameState;
use crate::gameplay::characters::enemies::test_enemy::spawn_test_enemy;
use bevy::prelude::*;


pub struct EnemyPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_test_enemy);
        app.add_systems(Update, (
            move_bosses,
            move_mobs
           ).run_if(in_state(GameState::Playing))
        );
    }
}

#[derive(Component)]
pub struct BossPhysics;

#[derive(Component)]
pub struct MobPhysics;

fn move_bosses() {

}

fn move_mobs() {

}