use crate::{
    consts::{CHUNK_SIZE, NOISE_SCALE, RENDER_DISTANCE, SPITE_SHEET_COLUMNS, TILE_SIZE},
    game_plugins::world::{chunks::ChunkConfig, type_bioms::BiomType, type_tiles::TileType},
};
use bevy::{log::info, prelude::IVec2};
use bevy_ecs_tilemap::tiles::TileBundle;
use noise::{NoiseFn, Perlin};

pub trait ToPerlinPoint<F64, const DIM: usize> {
    fn to_perlin_point(&self) -> [f64; DIM];
}

impl ToPerlinPoint<f64, 2> for IVec2 {
    fn to_perlin_point(&self) -> [f64; 2] {
        return [self.x as f64 + 0.1, self.y as f64 + 0.1];
    }
}

fn _get_biom_type_for_pos(tile_pos: &IVec2, config: &ChunkConfig) -> (BiomType, f64) {
    let perlin = Perlin::new(config.seeds.biom);

    let point = [
        ((tile_pos.x as f64) + 0.1 / (NOISE_SCALE * 2.0 * config.chunk_size as f64)), // / NOISE_SCALE * 8.0,
        ((tile_pos.y as f64) + 0.1 / (NOISE_SCALE * 2.0 * config.chunk_size as f64)), // / NOISE_SCALE * 8.0,
    ];
    let perlin_value = perlin.get(point);

    let biom = if perlin_value >= 0.2 {
        BiomType::GrassLand
    // } else if perlin_value >= -0.2 {
    //     BiomType::Desert
    } else {
        BiomType::Ocean
    };

    return (biom, perlin_value);
}

pub fn get_tile_type_for_pos(tile_pos: &IVec2, config: &ChunkConfig) -> TileType {
    let perlin1: Perlin = Perlin::new(config.seeds.tiles);
    let perlin2: Perlin = Perlin::new(config.seeds.tiles);

    let biom_noise = NOISE_SCALE * config.chunk_size as f64;
    let biom_point = [
        tile_pos.x as f64 / biom_noise,
        tile_pos.y as f64 / biom_noise,
    ];

    let tile_noise = NOISE_SCALE * config.chunk_size as f64 / 8.0 + 0.000432;
    let tile_point = [
        tile_pos.x as f64 / tile_noise,
        tile_pos.y as f64 / tile_noise,
    ];

    let point = perlin1.get(tile_point);
    let point2 = perlin2.get(biom_point);

    let point = 1.0 - ((point -0.45) * 2.0).abs();

    let threshhold = 0.23897;
    let point = point + 2.5*point2 - threshhold;

    if point > 2.0 {
        TileType::Dirt
    } else if point > 0.0 {
        TileType::Grass
    } else if point > -0.6 {
        TileType::Sand
    } else if point > -1.2 {
        TileType::Water
    } else {
        TileType::DeepWater
    }

    // abs((h - 0.5) * 2)
}

/*
pub fn get_tile_type_for_pos(tile_pos: &IVec2, config: &ChunkConfig) -> TileType {
    let biom_perlin = Perlin::new(config.seeds.biom);
    let perlin1 = Perlin::new(config.seeds.tiles);

    let biom_noise = NOISE_SCALE * config.chunk_size as f64 * 1.0;
    let biom_point = [
        (tile_pos.x as f64 + 0.1) / biom_noise,
        (tile_pos.y as f64 + 0.1) / biom_noise,
    ];

    let tile_point = [
        tile_pos.x as f64 / (NOISE_SCALE * 2 as f64),
        tile_pos.y as f64 / (NOISE_SCALE * 2 as f64),
    ];

    let biom_perlin_value = biom_perlin.get(biom_point);
    let tile_perlin_value = perlin1.get(tile_point);

    let biom_amplifier = 1.5;
    let tile_amplifier = 1.0;

    let tile_perlin_value = if biom_perlin_value.abs() > 0.3 {
        tile_perlin_value * tile_amplifier
    } else if biom_perlin_value.abs() > 0.25 {
        tile_perlin_value / 1.125 * tile_amplifier
    } else if biom_perlin_value.abs() > 0.2 {
        tile_perlin_value / 1.25 * tile_amplifier
    } else if biom_perlin_value.abs() > 0.15 {
        tile_perlin_value / 1.375 * tile_amplifier
    } else if biom_perlin_value.abs() > 0.1 {
        tile_perlin_value / 1.5 * tile_amplifier
    } else {
        tile_perlin_value / 1.625 * tile_amplifier
    };

    let biom_perlin_value = if biom_perlin_value.abs() > 0.8 {
        biom_perlin_value * 2.0 * biom_amplifier
    } else if biom_perlin_value.abs() > 0.75 {
        biom_perlin_value * 1.875 * biom_amplifier
    } else if biom_perlin_value.abs() > 0.7 {
        biom_perlin_value * 1.5 * biom_amplifier
    } else if biom_perlin_value.abs() > 0.65 {
        biom_perlin_value * 1.375 * biom_amplifier
    } else if biom_perlin_value.abs() > 0.6 {
        biom_perlin_value * 1.25 * biom_amplifier
    } else if biom_perlin_value.abs() > 0.55 {
        biom_perlin_value * 1.125 * biom_amplifier
    } else {
        biom_perlin_value * biom_amplifier
    };

    let cmp_value = tile_perlin_value + biom_perlin_value;

    // if cmp_value > 0.3 {
    //     TileType::Grass
    // } else {
    //     TileType::Water
    // }

    // info!("biom: {biom_perlin_value} tile: {tile_perlin_value} x: {} y: {}", tile_pos.x, tile_pos.y);

    if biom_perlin_value <= -2.95 {
        TileType::DeepWater
    } else if cmp_value > -0.45 {
        TileType::Grass
    } else if cmp_value > -1.9 {
        TileType::Sand
    } else if cmp_value > -2.5 {
        TileType::Water
    } else {
        TileType::DeepWater
    }
}
*/
