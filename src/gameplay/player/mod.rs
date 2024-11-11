mod movement_systems;

use crate::loading::SwordsMasterSpriteAssets;
use crate::gameplay::player::movement_systems::{
    grounded_movement,
    air_movement,
    detect_grounded,
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
enum PlayerAnimationState {
    // let all movement continue normally
    #[default]
    Free,
    // player control does not work in this state, but momentum continues
    Momentum,
    // player control does not work and player position is frozen
    Freeze,
}

pub struct PlayerPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<PlayerGroundState>().add_sub_state::<PlayerAnimationState>()
           .add_systems(OnEnter(GameState::Playing), spawn_player)
           .add_systems(Update, (
                detect_grounded
                    .run_if(in_state(GameState::Playing)),
                grounded_movement
                    .run_if(in_state(GameState::Playing))
                    .run_if(in_state(PlayerGroundState::Grounded))
                    .run_if(in_state(PlayerAnimationState::Free))
                    .after(detect_grounded),
                air_movement
                    .run_if(in_state(GameState::Playing))
                    .run_if(in_state(PlayerGroundState::Air))
                    .run_if(in_state(PlayerAnimationState::Free))
                    .after(detect_grounded),
                limit_velocity
                    .run_if(in_state(GameState::Playing)),
           )
        );
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