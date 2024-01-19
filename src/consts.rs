use bevy::{math::{UVec2, Vec2}, log::info};


// const Mult: f32 = 2.;

pub const TILE_SIZE: Vec2 = Vec2::splat(32.0);


pub const CHUNK_SIZE: u32 = 8;
pub const RENDER_DISTANCE: u32 = 2;

pub const NOISE_SCALE: f64 = 53.123;

pub const MAX_ZOOM: f32 = 16. * 4.;


pub const SPITE_SHEET_COLUMNS: usize = 4;
pub const SPITE_SHEET_ROWS: usize = 5;
pub const SPITE_SHEET_PADDING: Vec2 = Vec2::splat(5.0);
pub const SPITE_SHEET_OFFSET: Vec2 = Vec2::splat(4.0);


