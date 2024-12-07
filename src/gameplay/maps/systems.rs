use std::collections::HashMap;

use crate::gameplay::maps::Obstacle;
use crate::gameplay::maps::Ground;
use crate::gameplay::maps::MAP_TILE_SIZE;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::prelude::*;


pub fn spawn_oob_colliders(
    mut commands: Commands,
    layer_query: Query<&LayerMetadata, Added<LayerMetadata>>,
) {
    // we just need one LayerMetadata
    if let Some(layer_data) = layer_query.iter().next() {
        println!("LAYER METADATA");
        println!("{:?}", layer_data);
        let grid_size = layer_data.grid_size as f32;
        let height = layer_data.c_hei as f32;
        let width = layer_data.c_wid as f32;

        // left wall
        commands.spawn((
            Collider::cuboid(
                grid_size / 2.0,
                height * grid_size / 2.0,
            ),
            TransformBundle::from(Transform::from_xyz(
                -(grid_size / 2.0),
                height * grid_size / 2.0,
                2.5,
            )),
            ActiveEvents::COLLISION_EVENTS,
        ));
        // right wall
        commands.spawn((
            Collider::cuboid(
                grid_size / 2.0,
                height * grid_size / 2.0,
            ),
            TransformBundle::from(Transform::from_xyz(
                grid_size * (width + 0.5),
                height * grid_size / 2.0,
                2.5,
            )),
            ActiveEvents::COLLISION_EVENTS,
        ));
        // ceiling
        commands.spawn((
            Collider::cuboid(
                width * grid_size / 2.0,
                grid_size / 2.0,
            ),
            TransformBundle::from(Transform::from_xyz(
                width * grid_size / 2.0,
                (height+0.5) * grid_size,
                2.5,
            )),
            ActiveEvents::COLLISION_EVENTS,
        ));
    }
}

/// Spawns rapier collidables for the obstacles of a level, on adding obstacles
///
/// Instead, by flagging the wall tiles and spawning the collisions later,
/// we can minimize the amount of colliding entities.
///
/// 1. Create hashmap of rectangles made from grid coords keyed on Y coord
/// 2. iterate through each key to generate sets of adjacent obstacles
/// 4. spawn colliders for each
pub fn spawn_wall_collision(
    world: &mut World,
    obstacle_query: &mut QueryState<&GridCoords, Added<Obstacle>>,
) {

    #[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
    struct Rect {
        left: i32,
        right: i32,
        top: i32,
        bottom: i32,
    }

    let mut obstacles: HashMap<i32, Vec<Rect>> = HashMap::new();
    for grid_coords in obstacle_query.iter(world) {
        println!("grid coords: ({}, {})", grid_coords.x, grid_coords.y);

        if !obstacles.contains_key(&grid_coords.y) {
            obstacles.insert(grid_coords.y, Vec::new());
        }

        let obstacle_rect = Rect {
            left: grid_coords.x * MAP_TILE_SIZE,
            right: grid_coords.x * MAP_TILE_SIZE + MAP_TILE_SIZE,
            top: grid_coords.y * MAP_TILE_SIZE + MAP_TILE_SIZE,
            bottom: grid_coords.y * MAP_TILE_SIZE
        };

        obstacles.get_mut(&grid_coords.y).unwrap().push(obstacle_rect);
    }

    for (_, row) in obstacles.iter_mut() {
        row.sort_by(|a, b| (a.left).cmp(&b.left));
    }

    let mut final_colliders: Vec<Rect> = Vec::new();
    for y in obstacles.keys() {
        let obstacles_in_row = &obstacles[y];

        let mut curr_rect = obstacles_in_row[0].clone();
        for i in 1..obstacles[y].len() {
            if curr_rect.right == obstacles_in_row[i].left {
                curr_rect.right = obstacles_in_row[i].right;
            } else {
                final_colliders.push(curr_rect);
                curr_rect = obstacles_in_row[i].clone();
            }
        }
        final_colliders.push(curr_rect);
        println!("{:?}", final_colliders);
    }
    // ToDo: merge vertical colliders

    for rect in final_colliders {
        world.spawn((
            Collider::cuboid(
                ((rect.right - rect.left) / 2) as f32,
                ((rect.top - rect.bottom) / 2) as f32
            ),
            TransformBundle::from(Transform::from_xyz(
                ((rect.left + rect.right) / 2) as f32,
                ((rect.top + rect.bottom) / 2) as f32,
                2.5
            )),
            ActiveEvents::COLLISION_EVENTS,
        ))
        .insert(Ground);
    }
}
