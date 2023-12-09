use bevy::prelude::*;

pub const CHUNK_SIZE: usize = 16;

#[derive(Resource)]
pub struct World {}

impl World {
    pub fn new() -> Self {
        Self {}
    }
}
