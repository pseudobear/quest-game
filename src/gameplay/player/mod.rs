mod movement_systems;
mod animation_systems;

use crate::loading::swordsmaster::SwordsMasterSpriteAssets;
use crate::gameplay::player::movement_systems::{
    grounded_movement,
    grounded_turn_player,
    air_movement,
    air_turn_player,
    detect_grounded,
};
use crate::gameplay::player::animation_systems::{
    idle_animation,
    walk_animation,
    run_animation,
};
use crate::gameplay::resources::ScreenBottomLeft;
use crate::GameState;
use crate::animations::{ Animatable, AnimationConfig };
use bevy_rapier2d::prelude::*;
use bevy::prelude::*;
use movement_systems::limit_velocity;


#[derive(SubStates, Default, Clone, Eq, PartialEq, Debug, Hash)]
#[source(GameState = GameState::Playing)]
enum PlayerGroundState {
    #[default]
    Grounded,
    Air,
}

#[derive(SubStates, Default, Clone, Eq, PartialEq, Debug, Hash)]
#[source(GameState = GameState::Playing)]
enum PlayerMovementState {
    // let all movement continue normally
    #[default]
    Free,
    // player control does not work in this state, but momentum continues
    Momentum,
    // player control does not work and player position is frozen
    Freeze,
}

#[derive(Default, Clone, Eq, PartialEq, Debug, Hash)]
enum Facing {
    #[default]
    Right,
    Left,
}

#[derive(Component, Default, Debug)]
pub struct PlayerFacing {
    face: Facing
}

pub struct PlayerPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<PlayerGroundState>().add_sub_state::<PlayerMovementState>()
           .add_systems(OnEnter(GameState::Playing), spawn_player)
           .add_systems(Update, (
                detect_grounded,
                limit_velocity,

                (   // movement systems
                    (grounded_movement, grounded_turn_player)
                        .run_if(in_state(PlayerGroundState::Grounded)),
                    (air_movement, air_turn_player)
                        .run_if(in_state(PlayerGroundState::Air)),
                ).run_if(in_state(PlayerMovementState::Free)).after(detect_grounded),

                (   // animation systems
                    idle_animation,
                    walk_animation,
                    run_animation,
                ).run_if(in_state(PlayerGroundState::Grounded)).after(grounded_movement),

           ).run_if(in_state(GameState::Playing))
        );
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerSprite;

fn spawn_player(
    mut commands: Commands, 
    player_sprite: Res<SwordsMasterSpriteAssets>,
    screen_bottom_left: Res<ScreenBottomLeft>,
) {

    let mut player_animatable = create_player_animatable(&player_sprite);
    player_animatable.trigger_animation(0);

    commands
        .spawn((
            RigidBody::Dynamic,
            TransformBundle::from(Transform::from_xyz(
                10.0 + screen_bottom_left.x as f32 + 20.0,
                32.0 + screen_bottom_left.y as f32 + 12.0,
                2.5
            )),
            Collider::capsule_y(6.0, 6.0),
            GravityScale(2.0),
            LockedAxes::ROTATION_LOCKED,
            ActiveEvents::COLLISION_EVENTS,
            // markers to access rigidbody attributes
            ExternalForce { ..Default::default() },
            ExternalImpulse { ..Default::default() },
            Damping { ..Default::default() },
            Velocity { ..Default::default() },
            CollidingEntities::default(),
            Player
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
                    layout: player_animatable.animations[0].atlas_layout.clone(),
                    index: player_animatable.animations[0].first_sprite_index,
                },
                player_animatable,
                PlayerFacing { ..Default::default() },
                PlayerSprite
            ));
        });
}

/* Animations List:
0: Idle
1: Walk
2: Run
3: Run fast
4: Jump
5: Jump fall transition
6: Fall
*/
fn create_player_animatable(player_sprite: &Res<SwordsMasterSpriteAssets>) -> Animatable {
    let idle_config = AnimationConfig::new(0, 8, 10, true, player_sprite.idle.clone());
    let walk_config = AnimationConfig::new(0, 7, 10, true, player_sprite.walk.clone());
    let run_config = AnimationConfig::new(0, 7, 10, true, player_sprite.run.clone());
    let run_fast_config = AnimationConfig::new(0, 7, 10, true, player_sprite.run_fast.clone());
    let jump_config = AnimationConfig::new(0, 2, 10, true, player_sprite.jump.clone());
    let jump_fall_transition_config = AnimationConfig::new(0, 3, 10, false, player_sprite.jump_fall_transition.clone());
    let fall_config = AnimationConfig::new(0, 2, 10, true, player_sprite.fall.clone());

    let player_animatable = Animatable::new(Vec::from([
        idle_config,
        walk_config,
        run_config,
        run_fast_config,
        jump_config,
        jump_fall_transition_config,
        fall_config,
    ]));

    return player_animatable;
}