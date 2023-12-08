use super::chunk::Chunk;
use super::particle::Particle;
use bevy::prelude::*;
use bevy::utils::HashMap;
use std::sync::{Arc, RwLock};

pub const CHUNK_SIZE: usize = 16;

#[derive(Resource)]
pub struct World {
    chunks: HashMap<IVec2, Arc<RwLock<Chunk>>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
        }
    }

    pub fn chunk_positions(&self) -> Vec<&IVec2> {
        self.chunks.keys().collect()
    }

    pub fn add_particle(&mut self, commands: &mut Commands, particle: Particle) {
        let chunk_pos = particle.position / CHUNK_SIZE as i32;
        let chunk = self
            .chunks
            .entry(chunk_pos)
            .or_insert(Arc::new(RwLock::new(Chunk::new(chunk_pos))));
        chunk.write().unwrap().insert(commands, particle);
    }

    pub fn remove_particle(&mut self, commands: &mut Commands, position: IVec2) {
        let chunk_pos = position / CHUNK_SIZE as i32;
        let mut removed = false;
        if let Some(chunk) = self.chunks.get_mut(&chunk_pos) {
            let mut chunk = chunk.write().unwrap();
            chunk.remove(commands, position);
            if chunk.empty() {
                removed = true;
            }
        }

        if removed {
            self.chunks.remove(&chunk_pos);
        }
    }
}
