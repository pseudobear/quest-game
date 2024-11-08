use crate::gameplay::actions::Actions;
use crate::loading::{SwordsMasterSpriteAssets, TextureAssets};
use crate::GameState;
use crate::animations::trigger_animation;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, move_player.run_if(in_state(GameState::Playing)))
            .add_systems(Update, (
                trigger_animation::<Player>.run_if(input_just_pressed(KeyCode::ArrowRight)),
                trigger_animation::<Player>.run_if(input_just_pressed(KeyCode::ArrowLeft)),
            ));
    }
}

fn spawn_player(mut commands: Commands, player_sprite: Res<SwordsMasterSpriteAssets>) {
    commands
        .spawn(SpriteBundle {
            texture: player_sprite.sheet.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 2.5)),
            ..Default::default()
        })
        .insert(Player);
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }
    let speed = 150.;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_seconds(),
        actions.player_movement.unwrap().y * speed * time.delta_seconds(),
        0.,
    );
    for mut player_transform in &mut player_query {
        player_transform.translation += movement;
    }
}
