use super::chunk::Chunk;
use super::particle::Particle;
use bevy::prelude::*;
use bevy::utils::HashMap;

pub const CHUNK_SIZE: usize = 16;

#[derive(Resource)]
pub struct World {
    chunks: HashMap<IVec2, Chunk>,
}

impl World {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
        }
    }

    pub fn add_particle(&mut self, commands: &mut Commands, particle: Particle) {
        let chunk_pos = particle.position / CHUNK_SIZE as i32;
        let chunk = self
            .chunks
            .entry(chunk_pos)
            .or_insert(Chunk::new(chunk_pos));
        chunk.insert(commands, particle);
    }
}
