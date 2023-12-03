use crate::{Money, Player};
use bevy::{audio::PlaybackMode, prelude::*};

pub struct WorldGrid;

impl Plugin for WorldGrid {
    fn build(&self, app: &mut App) {
        app.init_resource::<PigCount>()
            .add_systems(Startup, setup_grid)
            .register_type::<Pig>();
    }
}