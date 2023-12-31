use std::{io::{BufWriter, BufReader, Write}, fs::File};

use bevy::{math::{UVec2, Vec3}, utils::{HashMap, Instant}, log::info};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use super::tile::Tile;




pub struct ChunkConfig {
    chunk_dimentions: UVec2,

    
}

#[derive(Serialize, Deserialize)]
pub struct Chunk(pub HashMap<(i32, i32), Tile>);

impl Chunk {
    pub fn empty() -> Self {
        return Self{0: HashMap::new()}
    }

    pub fn insert_tile(&mut self, tile: Tile) {
        self.0.insert((tile.grid_position.x as i32, tile.grid_position.y as i32), tile);
    }

    pub fn generate(size: Option<u16>, spite_count: Option<usize>) -> Self {
        let size = size.unwrap_or_else(|| 4) as i32;
        let spite_count = spite_count.unwrap_or_else(|| 4);

        let mut chunk = Chunk::empty();
        let mut rng = thread_rng();

        let start = Instant::now();
        for x in -size ..=size {
            for y in -size..=size {
                let tile = Tile {
                    grid_position: Vec3 {
                        x: x as f32,
                        y: y as f32,
                        z: 0 as f32,
                    },
                    scale: 1.0,
                    spite_index: rng.gen_range(0..spite_count)
                };

                chunk.insert_tile(tile);
            }
        }
        let end = start.elapsed();

        info!("generating chunk of size {}, took: {} seconds", size, end.as_secs_f64());

        return chunk;
    }

    pub fn save_to_file(&self) {
        let file_path = "chunk.bin";


        let start = std::time::Instant::now();
        let mut writer = BufWriter::new(File::create(file_path).expect("Uh Oh"));
        bincode::serialize_into(&mut writer, self).unwrap();
        writer.flush().unwrap();
    
        let end1 = start.elapsed().as_secs_f64();
        info!("writing to file took {} seconds", end1);
    }

    pub fn load_from_file(file_path: &str) -> Option<Chunk> {
        let start = std::time::Instant::now();
        let file_path = File::open(file_path);
        
        if file_path.is_err() {
            return None;
        }
    
    
        let reader = BufReader::new(file_path.unwrap());
        let loaded: Chunk = bincode::deserialize_from(reader).unwrap();
        let end2 = start.elapsed().as_secs_f64();
    
        info!("loading file took {} seconds", end2);
    
        return Some(loaded);
    }
}
