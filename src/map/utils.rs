use bevy::math::IVec2;
use bevy_ecs_tilemap::tiles::TilePos;

use crate::consts::{TILE_SIZE, CHUNK_SIZE};



pub fn world_to_chunks((x, y): (f32, f32)) -> (i32, i32) {
    // x and y are the cords in pixels
    // we want to get the chunk position
    // calculation: cord / pixels_per_tile * tiles_per_chunk

    let current_chunk_x = x / (TILE_SIZE.x * CHUNK_SIZE.x as f32);
    let current_chunk_y = y / (TILE_SIZE.y * CHUNK_SIZE.y as f32);

    return (
        current_chunk_x.floor() as i32,
        current_chunk_y.floor() as i32,
    );
}

pub fn world_to_chunks_tile((x, y): (f32, f32)) -> (IVec2, TilePos) {
    let x_tile_count = (x / TILE_SIZE.x).floor() as i32;
    let y_tile_count = (y / TILE_SIZE.y).floor() as i32;

    let x_chunk_count = x_tile_count / CHUNK_SIZE.x as i32;
    let y_chunk_count = y_tile_count / CHUNK_SIZE.y as i32;

    let tile_x = x_tile_count % CHUNK_SIZE.x as i32;
    let tile_y = y_tile_count % CHUNK_SIZE.y as i32;

    return (
        IVec2::new(x_chunk_count, y_chunk_count),
        TilePos::new(tile_x.abs() as u32, tile_y.abs() as u32)
    );
}

pub fn chunks_to_world(chunk_position: IVec2, tile_position: TilePos) -> (f32, f32) {
    (
        (CHUNK_SIZE.x as i32 * chunk_position.x * TILE_SIZE.x as i32) as f32 + (tile_position.x as i32 * TILE_SIZE.x as i32) as f32,
        (CHUNK_SIZE.y as i32 * chunk_position.y * TILE_SIZE.y as i32) as f32 + (tile_position.y as i32 * TILE_SIZE.y as i32) as f32,
    )
}

pub fn world_to_tiles((x, y): (f32, f32)) -> (i32, i32) {
    let x_tile_count = (x / TILE_SIZE.x).floor() as i32;
    let y_tile_count = (y / TILE_SIZE.y).floor() as i32;

    return (x_tile_count, y_tile_count);
}

