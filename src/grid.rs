use bevy::{audio::PlaybackMode, prelude::Color, prelude::*};

const CHUNK_DIMENSIONS: usize = 124;
const TILE_SIZE: usize = 124;
pub struct WorldGridBundel;




fn setup_grid(commands: Commands) {}


#[derive(Component, Clone, Copy, Debug)]
pub struct Tile {
    color: Color,
}
impl Default for Tile {
    fn default() -> Self {
        Self {
            color: Color::GREEN,
        }
    }
}


#[derive(Debug)]
pub struct Chunk {
    tiles: [[Tile; CHUNK_DIMENSIONS]; CHUNK_DIMENSIONS],
}

impl Default for Chunk {
    fn default() -> Self {
        let tiles = [[Tile::default(); CHUNK_DIMENSIONS]; CHUNK_DIMENSIONS];

        return Self { tiles }
    }
}

impl Chunk {
    pub fn get_tile(&self, x: usize, y: usize) -> Tile {
        return self.tiles[x][y]
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: Tile) {
        self.tiles[x][y] = tile
    }
}

