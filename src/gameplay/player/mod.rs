mod movement_systems;

use crate::loading::SwordsMasterSpriteAssets;
use crate::gameplay::player::movement_systems::move_player;
use crate::gameplay::resources::ScreenBottomLeft;
use crate::GameState;
use crate::animations::{ Animatable, AnimationConfig };
use bevy_rapier2d::prelude::*;
use bevy::prelude::*;


#[derive(SubStates, Default, Clone, Eq, PartialEq, Debug, Hash)]
#[source(GameState = GameState::Playing)]
enum PlayerGroundState {
    #[default]
    Grounded,
    Air,
}

pub struct PlayerPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<PlayerGroundState>()
           .add_systems(OnEnter(GameState::Playing), spawn_player)
           .add_systems(Update, move_player.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(
    mut commands: Commands, 
    player_sprite: Res<SwordsMasterSpriteAssets>,
    screen_bottom_left: Res<ScreenBottomLeft>,
) {

    let idle_config = AnimationConfig::new(0, 8, 10, true, player_sprite.idle.clone());
    let walk_config = AnimationConfig::new(0, 7, 10, true, player_sprite.walk.clone());
    let run_config = AnimationConfig::new(0, 7, 10, true, player_sprite.run.clone());

    let mut player_animatable = Animatable::new(Vec::from([
        idle_config,
        walk_config,
        run_config,
    ]));

    player_animatable.trigger_animation(0);

    commands
        .spawn((
            RigidBody::Dynamic,
            TransformBundle::from(Transform::from_xyz(
                10.0 + screen_bottom_left.x as f32 + 20.0,
                32.0 + screen_bottom_left.y as f32 + 12.0,
                2.5
            )),
            Collider::cuboid(6.0, 12.0),
            LockedAxes::ROTATION_LOCKED
        ))
        .with_children(|children| {
            children.spawn((
                SpriteBundle {
                    texture: player_sprite.sheet.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        17.0, 3.0, 0.
                    )),
                    ..Default::default()
                },
                TextureAtlas {
                    layout: player_animatable.animations[0].layout.clone(),
                    index: player_animatable.animations[0].first_sprite_index,
                },
                player_animatable,
            ));
        })
        .insert(Player);
}