use std::collections::{HashMap, HashSet};
use crate::gameplay::maps::Obstacle;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::prelude::*;

/// Spawns rapier collidables for the obstacles of a level
///
/// You could just insert a ColliderBundle in to the WallBundle,
/// but this spawns a different collider for EVERY wall tile.
/// This approach leads to bad performance.
///
/// Instead, by flagging the wall tiles and spawning the collisions later,
/// we can minimize the amount of colliding entities.
///
/// The algorithm used here is a nice compromise between simplicity, speed,
/// and a small number of rectangle colliders.
/// In basic terms, it will:
/// 1. consider where the walls are
/// 2. combine wall tiles into flat "plates" in each individual row
/// 3. combine the plates into rectangles across multiple rows wherever possible
/// 4. spawn colliders for each rectangle
pub fn spawn_wall_collision(
    mut commands: Commands,
    obtacle_query: Query<(&GridCoords, &Parent), Added<Obstacle>>,
) {
    /*
    /// Represents a wide wall that is 1 tile tall
    /// Used to spawn wall collisions
    #[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
    struct Plate {
        left: i32,
        right: i32,
    }

    /// A simple rectangle type representing a wall of any size
    struct Rect {
        left: i32,
        right: i32,
        top: i32,
        bottom: i32,
    }
    commands.entity(level_entity).with_children(|level| {
        // Spawn colliders for every rectangle..
        // Making the collider a child of the level serves two purposes:
        // 1. Adjusts the transforms to be relative to the level for free
        // 2. the colliders will be despawned automatically when levels unload
        for wall_rect in wall_rects {
            level
                .spawn_empty()
                .insert(Collider::cuboid(
                    (wall_rect.right as f32 - wall_rect.left as f32 + 1.)
                        * grid_size as f32
                        / 2.,
                    (wall_rect.top as f32 - wall_rect.bottom as f32 + 1.)
                        * grid_size as f32
                        / 2.,
                ))
                .insert(RigidBody::Fixed)
                .insert(Friction::new(1.0))
                .insert(Transform::from_xyz(
                    (wall_rect.left + wall_rect.right + 1) as f32 * grid_size as f32
                        / 2.,
                    (wall_rect.bottom + wall_rect.top + 1) as f32 * grid_size as f32
                        / 2.,
                    0.,
                ))
                .insert(GlobalTransform::default());
        }
    });
    */
}
