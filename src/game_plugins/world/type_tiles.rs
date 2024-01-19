use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use rand::distributions::{Distribution, Standard};


#[derive(Debug, EnumIter, PartialEq, Eq)]
pub enum TileType {
    Grass,
    Water,
    DeepWater,
    Sand,
    Dirt,
}

impl TileType {
    pub fn to_usize(&self) -> usize {
        for (index, value) in TileType::iter().enumerate() {
            if self == &value {
                return index;
            }
        }
        panic!("unable to get TileType as usize - no elements found in enum - this code should nevery be able to execute");
    }
}

impl From<&TileType> for usize {
    fn from(value: &TileType) -> Self {
        value.to_usize()

    }
}

impl From<TileType> for usize {
    fn from(value: TileType) -> Self {
        value.to_usize()
    }
}



impl Distribution<TileType> for Standard {
    fn sample<R: rand::prelude::Rng + ?Sized>(&self, rng: &mut R) -> TileType {
        let element_count = TileType::iter().len();

        match rng.gen_range(0..element_count) {
            0 => TileType::Grass,
            1 => TileType::Water,
            3 => TileType::DeepWater,
            4 => TileType::Sand,
            2 => TileType::Dirt,
            _ => panic!("unknow tile type -> please implent!!"),
        }
    }
}
