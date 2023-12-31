use bevy::{math::Vec3, transform::components::Transform, log::{debug, info}, prelude::default, ecs::component::Component};
use serde::{Deserialize, Serialize};

use crate::consts::TILE_SIZE;

#[derive(Component, Clone, Copy, Serialize, Deserialize)]
pub struct Tile {
    pub grid_position: Vec3,
    pub scale: f32,
    pub spite_index: usize
}

impl From<Tile> for Transform {
    fn from(tile: Tile) -> Self {
        let tile_size: Vec3 = Vec3 {
            x: TILE_SIZE.x,
            y: TILE_SIZE.y,
            z: 1.0,
        };

        let position = tile.grid_position * tile_size * tile.scale;

        debug!("old: {}, new: {}", tile.grid_position, position);

        return Transform {
            translation: position,
            scale: Vec3::splat(tile.scale),
            ..default()
        };
    }
}

impl From<&Tile> for Transform {
    fn from(tile: &Tile) -> Self {
        // let tile_size: Vec3 = Vec3 {
        //     x: TILE_SIZE.x,
        //     y: TILE_SIZE.y,
        //     z: 1.0,
        // };

        // let position = tile.grid_position * tile_size * tile.scale;

        // debug!("old: {}, new: {}", tile.grid_position, position);

        return Transform {
            translation: tile.grid_position,
            scale: Vec3::splat(tile.scale),
            ..default()
        };
    }
}
