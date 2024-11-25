use crate::animations::Animatable;
use crate::gameplay::items::CharacterEquips;
use crate::gameplay::items::weapons::BARE_FISTS;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;


#[derive(Component, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GroundStatus {
    #[default]
    Grounded,
    Air,
}

#[derive(Component, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Facing {
    #[default]
    Right,
    Left,
}

#[derive(Component)]
pub struct CharacterAttributes;

#[derive(Component)]
pub struct CharacterPhysics;

#[derive(Component)]
pub struct CharacterSprite {
    pub centering_transform: Vec3
}

#[derive(Bundle)]
pub struct CharacterPhysicsBundle {
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub gravity_scale: GravityScale,
    pub locked_axes: LockedAxes,
    pub active_events: ActiveEvents,
    pub solver_group: SolverGroups,
    // markers to access rigidbody attributes
    pub external_force: ExternalForce,
    pub external_impulse: ExternalImpulse,
    pub damping: Damping,
    pub velocity: Velocity,
    pub colliding_entities: CollidingEntities,
    pub ground_status: GroundStatus,
    pub character_physics: CharacterPhysics,
}

impl Default for CharacterPhysicsBundle {
    fn default() -> Self {
        Self {
            rigid_body: RigidBody::Dynamic,
            collider: Collider::capsule_y(6.0, 6.0),
            gravity_scale: GravityScale(2.0),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            active_events: ActiveEvents::COLLISION_EVENTS,
            solver_group: SolverGroups::new(Group::GROUP_1, Group::GROUP_1.complement()),
            // markers to access rigidbody attributes
            external_force: ExternalForce { ..Default::default() },
            external_impulse: ExternalImpulse { ..Default::default() },
            damping: Damping { ..Default::default() },
            velocity: Velocity { ..Default::default() },
            colliding_entities: CollidingEntities::default(),
            ground_status: GroundStatus::default(),
            character_physics: CharacterPhysics,
        }
    }
}

#[derive(Bundle)]
pub struct CharacterSpriteBundle {
    pub sprite_bundle: SpriteBundle,
    pub texture_atlas: TextureAtlas,
    pub animatable: Animatable,
    pub facing: Facing,
    pub character_sprite: CharacterSprite,
}

impl CharacterSpriteBundle {
    pub fn new(
        transform: Transform,
        texture: Handle<Image>,
        animatable: Animatable,
        centering_transform: Vec3,
    ) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                texture: texture,
                transform: transform,
                ..Default::default()
            },
            texture_atlas: TextureAtlas { ..Default::default() },
            animatable: animatable,
            facing: Facing::default(),
            character_sprite: CharacterSprite { centering_transform: centering_transform },
        }
    }
}

#[derive(Bundle)]
pub struct CharacterAttributesBundle {
    pub character_equips: CharacterEquips,
    pub character_attributes: CharacterAttributes,
}

impl Default for CharacterAttributesBundle {
    fn default() -> Self {
        Self {
            character_equips: CharacterEquips {
                weapon: BARE_FISTS
            },
            character_attributes: CharacterAttributes,
        }
    }
}