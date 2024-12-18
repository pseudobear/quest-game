use crate::loading::test_enemy::TestEnemySpriteAssets;
use crate::gameplay::characters::components::{
    CharacterPhysicsBundle,
    CharacterSpriteBundle,
};
use crate::gameplay::characters::enemies::BossPhysics;
use crate::gameplay::resources::ScreenBottomLeft;
use crate::animations::{ Animatable, AnimationConfig };
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;


pub fn spawn_test_enemy(
    mut commands: Commands, 
    enemy_sprite: Res<TestEnemySpriteAssets>,
    screen_bottom_left: Res<ScreenBottomLeft>,
) {

    let mut enemy_animatable = create_test_enemy_animatable(&enemy_sprite);
    enemy_animatable.trigger_animation(0, false);

    commands
        .spawn((
            TransformBundle::from(Transform::from_xyz(
                10.0 + screen_bottom_left.x as f32 + 20.0 + 200.0,
                32.0 + screen_bottom_left.y as f32 + 12.0,
                2.5
            )),
            CharacterPhysicsBundle { 
                collider: Collider::capsule_y(17., 12.),
                ..Default::default()
            },
            BossPhysics,
        ))
        .with_children(|children| {
            children.spawn(   // Animations, appearance, hitbox and attributes
                CharacterSpriteBundle::new(
                    Transform::from_translation(Vec3::new(
                        4.0, 8.0, 0.
                    )),
                    enemy_sprite.sheet.clone(),
                    enemy_animatable,
                    Vec3::new(4.0, 8.0, 0.0)
                ),
            );
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
fn create_test_enemy_animatable(enemy_sprite: &Res<TestEnemySpriteAssets>) -> Animatable {
    let idle_config = AnimationConfig::new(0, 7, 10, true, enemy_sprite.float.clone());
    let walk_config = AnimationConfig::new(0, 7, 10, true, enemy_sprite.float.clone());
    let run_config = AnimationConfig::new(0, 7, 10, true, enemy_sprite.float.clone());
    let run_fast_config = AnimationConfig::new(0, 7, 10, true, enemy_sprite.float.clone());
    let jump_config = AnimationConfig::new(0, 7, 10, true, enemy_sprite.float.clone());
    let jump_fall_transition_config = AnimationConfig::new(0, 7, 10, false, enemy_sprite.float.clone());
    let fall_config = AnimationConfig::new(0, 7, 10, true, enemy_sprite.float.clone());
    let slash_1_config = AnimationConfig::new(0, 5, 10, false, enemy_sprite.slash_1.clone());

    let enemy_animatable = Animatable::new(Vec::from([
        idle_config,
        walk_config,
        run_config,
        run_fast_config,
        jump_config,
        jump_fall_transition_config,
        fall_config,
        slash_1_config,
    ]));

    return enemy_animatable;
}