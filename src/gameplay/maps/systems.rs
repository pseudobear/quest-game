use crate::gameplay::maps::Obstacle;
use crate::gameplay::maps::Ground;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::prelude::*;

/// Spawns rapier collidables for the obstacles of a level, on adding obstacles
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
    world: &mut World,
    obstacle_query: &mut QueryState<&GridCoords, Added<Obstacle>>,
) {

    // ToDo: merge adjacent rectangles and just spawn a long collider
    // ToDo: builder for rect, store tile size somewhere globally
    struct Rect {
        left: i32,
        right: i32,
        top: i32,
        bottom: i32,
    }

    let mut obstacle_rects: Vec<Rect> = Vec::new();
    for grid_coords in obstacle_query.iter(world) {
        println!("grid coords: ({}, {})", grid_coords.x, grid_coords.y);
        let obstacle_rect = Rect {
            left: grid_coords.x * 16 + 8,
            right: grid_coords.x * 16,
            top: grid_coords.y * 16,
            bottom: grid_coords.y * 16 + 8
        };
        obstacle_rects.push(obstacle_rect);
    }
    for rect in obstacle_rects {
        world.spawn((
            Collider::round_cuboid(8.0, 8.0, 0.1),
            TransformBundle::from(Transform::from_xyz(
                rect.left as f32,
                rect.bottom as f32,
                2.5
            )),
            ActiveEvents::COLLISION_EVENTS,
        ))
        .insert(Ground);
    }
}
