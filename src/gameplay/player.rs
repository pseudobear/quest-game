use crate::gameplay::actions::Actions;
use crate::loading::SwordsMasterSpriteAssets;
use crate::gameplay::resources::ScreenBottomLeft;
use crate::GameState;
use crate::animations::{ trigger_animation, AnimationConfig };
use bevy::input::common_conditions::input_just_pressed;
use bevy::{animation, prelude::*};

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

fn spawn_player(
    mut commands: Commands, 
    player_sprite: Res<SwordsMasterSpriteAssets>,
    screen_bottom_left: Res<ScreenBottomLeft>,
) {

    let idle_config = AnimationConfig::new(0, 8, 10, true, player_sprite.idle.clone());

    commands
        .spawn((
            SpriteBundle {
                texture: player_sprite.sheet.clone(),
                transform: Transform::from_translation(Vec3::new(
                    0. + screen_bottom_left.x as f32, 0. + screen_bottom_left.y as f32, 2.5
                )),
                ..Default::default()
            },
            TextureAtlas {
                layout: idle_config.layout.clone(),
                index: idle_config.first_sprite_index,
            },
            idle_config,
        ))
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
