use bevy::log::info;
use noise::core::perlin;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use rand::distributions::{Distribution, Standard};

use crate::consts::*;

#[derive(Debug, EnumIter, PartialEq, Eq)]
pub enum TileType {
    GRASS,
    WATER,
}

impl TileType {
    pub fn to_usize(&self) -> usize {
        for (index, value) in TileType::iter().enumerate() {
            if self == &value {
                return index;
            }
        }
        panic!("unable to get TileType as usize - no elements found in enum");
    }
}

impl From<&TileType> for usize {
    fn from(value: &TileType) -> Self {
        match value {
            TileType::GRASS => 0,
            TileType::WATER => 1,
        }
    }
}

impl From<TileType> for usize {
    fn from(value: TileType) -> Self {
        match value {
            TileType::GRASS => 0,
            TileType::WATER => 1,
        }
    }
}



impl Distribution<TileType> for Standard {
    fn sample<R: rand::prelude::Rng + ?Sized>(&self, rng: &mut R) -> TileType {
        let element_count = TileType::iter().len();

        match rng.gen_range(0..element_count) {
            0 => TileType::GRASS,
            1 => TileType::WATER,
            _ => panic!("unknow tile type -> please implent!!"),
        }
    }
}
