use bevy::prelude::*;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

#[derive(Resource)]
pub struct SeededRng{
    pub rng: ChaCha8Rng,
}

impl Default for SeededRng {
    fn default() -> Self {
        // let seed: <ChaCha8Rng as SeedableRng>::Seed = Default::default();
        let rng = ChaCha8Rng::seed_from_u64(1283674); //from_seed(seed);

        Self { rng }
    }
}