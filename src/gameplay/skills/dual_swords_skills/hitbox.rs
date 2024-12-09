use crate::gameplay::hitbox::{
    HitboxThrower,
    HitboxConfig,
};
use crate::gameplay::hitbox::hitbox_frame::{
    HitboxFrame,
    CuboidColliderSpec,
};

/* Hitboxes List:
0: slash_1 (basic attack)
*/
pub fn create_dual_swords_hitbox_thrower() -> HitboxThrower {

    let basic_attack_0_config = HitboxConfig::new("ds_basic_attack_0".to_string(), 0, 6, 10, false, ds_basic_attack_0_frames());
    let basic_attack_1_config = HitboxConfig::new("ds_basic_attack_1".to_string(), 0, 4, 10, false, ds_basic_attack_1_frames());
    let basic_attack_2_config = HitboxConfig::new("ds_basic_attack_2".to_string(), 0, 4, 10, false, ds_basic_attack_2_frames());
    let basic_attack_3_config = HitboxConfig::new("ds_basic_attack_3".to_string(), 0, 5, 10, false, ds_basic_attack_3_frames());

    HitboxThrower::new(Vec::from([
        basic_attack_0_config,
        basic_attack_1_config,
        basic_attack_2_config,
        basic_attack_3_config,
    ]))
}


fn ds_basic_attack_0_frames() -> Vec<HitboxFrame> {
    Vec::from([
        HitboxFrame::new(1, 1, Vec::from([CuboidColliderSpec::new(
            24.0, 8.0,
            14.0, -7.0
        )])),
        HitboxFrame::new(2, 2, Vec::from([CuboidColliderSpec::new(
            24.0, 4.0,
            14.0, -7.0
        )])),
    ])
}

fn ds_basic_attack_1_frames() -> Vec<HitboxFrame> {
    Vec::from([
        HitboxFrame::new(1, 1, Vec::from([CuboidColliderSpec::new(
            24.0, 8.0,
            14.0, -7.0
        )])),
        HitboxFrame::new(2, 2, Vec::from([CuboidColliderSpec::new(
            24.0, 4.0,
            14.0, -7.0
        )])),
    ])
}

fn ds_basic_attack_2_frames() -> Vec<HitboxFrame> {
    Vec::from([
        HitboxFrame::new(1, 1, Vec::from([CuboidColliderSpec::new(
            24.0, 8.0,
            14.0, -7.0
        )])),
        HitboxFrame::new(2, 2, Vec::from([CuboidColliderSpec::new(
            24.0, 4.0,
            14.0, -7.0
        )])),
    ])
}

fn ds_basic_attack_3_frames() -> Vec<HitboxFrame> {
    Vec::from([
        HitboxFrame::new(1, 1, Vec::from([CuboidColliderSpec::new(
            24.0, 8.0,
            14.0, -7.0
        )])),
        HitboxFrame::new(2, 2, Vec::from([CuboidColliderSpec::new(
            24.0, 4.0,
            14.0, -7.0
        )])),
    ])
}