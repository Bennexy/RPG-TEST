use bevy::math::UVec2;
use bevy_ecs_tilemap::map::TilemapTileSize;

// const Mult: f32 = 2.;

pub const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 32.0, y: 32.0 };
// the size of the provided images
pub const TILE_PIXEL_SIZE: TilemapTileSize = TilemapTileSize { x: 32.0, y: 32.0 }; 
// For this example, don't choose too large a chunk size.
pub const CHUNK_SIZE: UVec2 = UVec2 { x: 16, y: 16 };
// Render chunk sizes are set to 4 render chunks per user specified chunk.
pub const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: 6 * 2,
    y: 6 * 2
};

pub const NOISE_SCALE: f64 = 6.;

pub const MAX_ZOOM: f32 = 16. * 2.;