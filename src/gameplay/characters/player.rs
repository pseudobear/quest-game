use crate::loading::swordsmaster::SwordsMasterSpriteAssets;
use crate::gameplay::characters::components::{
    CharacterPhysicsBundle,
    CharacterSpriteBundle,
};
use crate::gameplay::characters::systems::movement::{
    player_grounded_movement,
    player_air_movement,
    detect_grounded,
};
use crate::gameplay::characters::systems::event::{
    emit_ds_skill_activation,
};
use crate::gameplay::resources::ScreenBottomLeft;
use crate::GameState;
use crate::animations::{ Animatable, AnimationConfig };
use crate::gameplay::items::CharacterEquips;
use crate::gameplay::items::weapons::TESTING_SWORDS;
use bevy::prelude::*;

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

#[derive(Component)]
pub struct PlayerPhysics;

#[derive(Component)]
pub struct PlayerSprite;

pub struct PlayerPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<PlayerMovementState>()
           .add_systems(OnEnter(GameState::Playing), spawn_player)
           .add_systems(Update, (
                emit_ds_skill_activation::<PlayerPhysics, PlayerSprite>,
                (   // movement systems
                    player_grounded_movement::<PlayerPhysics>,
                    player_air_movement::<PlayerPhysics>,
                ).run_if(in_state(PlayerMovementState::Free)).after(detect_grounded),
           ).run_if(in_state(GameState::Playing))
        );
    }
}

fn spawn_player(
    mut commands: Commands, 
    player_sprite: Res<SwordsMasterSpriteAssets>,
    screen_bottom_left: Res<ScreenBottomLeft>,
) {

    let mut player_animatable = create_player_animatable(&player_sprite);
    player_animatable.trigger_animation(0, false);

    commands
        .spawn((
            TransformBundle::from(Transform::from_xyz(
                10.0 + screen_bottom_left.x as f32 + 20.0,
                32.0 + screen_bottom_left.y as f32 + 12.0,
                2.5
            )),
            CharacterPhysicsBundle { ..Default::default() },
            PlayerPhysics,
        ))
        .with_children(|children| {
            children.spawn((   // Animations, appearance, hitbox and attributes
                CharacterSpriteBundle::new(
                    Transform::from_translation(Vec3::new(
                        17.0, 3.0, 0.
                    )),
                    player_sprite.sheet.clone(),
                    player_animatable,
                    Vec3::new(17.0, 3.0, 0.0)
                ),
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
7: Slash_1
*/
fn create_player_animatable(player_sprite: &Res<SwordsMasterSpriteAssets>) -> Animatable {
    let idle_config = AnimationConfig::new(0, 8, 10, true, player_sprite.idle.clone());
    let walk_config = AnimationConfig::new(0, 7, 10, true, player_sprite.walk.clone());
    let run_config = AnimationConfig::new(0, 7, 10, true, player_sprite.run.clone());
    let run_fast_config = AnimationConfig::new(0, 7, 10, true, player_sprite.run_fast.clone());
    let jump_config = AnimationConfig::new(0, 2, 10, true, player_sprite.jump.clone());
    let jump_fall_transition_config = AnimationConfig::new(0, 3, 16, false, player_sprite.jump_fall_transition.clone());
    let fall_config = AnimationConfig::new(0, 2, 10, true, player_sprite.fall.clone());
    let slash_1_config = AnimationConfig::new(0, 6, 10, false, player_sprite.slash_1.clone());

    let player_animatable = Animatable::new(Vec::from([
        idle_config,
        walk_config,
        run_config,
        run_fast_config,
        jump_config,
        jump_fall_transition_config,
        fall_config,
        slash_1_config,
    ]));

    return player_animatable;
}