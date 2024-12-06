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

    let basic_attack_config = HitboxConfig::new("ds_basic_attack".to_string(), 0, 6, 10, false, ds_basic_attack_frames());

    HitboxThrower::new(Vec::from([
        basic_attack_config
    ]))
}


fn ds_basic_attack_frames() -> Vec<HitboxFrame> {
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
