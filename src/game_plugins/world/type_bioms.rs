use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use rand::distributions::{Distribution, Standard};


#[derive(Debug, EnumIter, PartialEq, Eq)]
pub enum BiomType {
    GrassLand,
    Ocean,
    // Desert,
}

impl BiomType {
    pub fn to_usize(&self) -> usize {
        for (index, value) in BiomType::iter().enumerate() {
            if self == &value {
                return index;
            }
        }
        panic!("unable to get BiomType as usize - no elements found in enum - this code should nevery be able to execute");
    }
}

impl From<&BiomType> for usize {
    fn from(value: &BiomType) -> Self {
        value.to_usize()

    }
}

impl From<BiomType> for usize {
    fn from(value: BiomType) -> Self {
        value.to_usize()
    }
}



impl Distribution<BiomType> for Standard {
    fn sample<R: rand::prelude::Rng + ?Sized>(&self, rng: &mut R) -> BiomType {
        let element_count = BiomType::iter().len();

        match rng.gen_range(0..element_count) {
            0 => BiomType::GrassLand,
            1 => BiomType::Ocean,
            // 2 => BiomType::Desert,
            _ => panic!("unknow tile type -> please implent!!"),
        }
    }
}
