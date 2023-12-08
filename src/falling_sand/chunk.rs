use super::particle::Particle;
use super::world::CHUNK_SIZE;
use bevy::prelude::*;

pub struct Chunk {
    position: IVec2,
    pub particles: [Option<Entity>; CHUNK_SIZE * CHUNK_SIZE],
    count: usize,
}

impl Chunk {
    pub fn new(position: IVec2) -> Self {
        Self {
            position,
            particles: [None; CHUNK_SIZE * CHUNK_SIZE],
            count: 0,
        }
    }

    pub fn empty(&self) -> bool {
        self.count == 0
    }

    pub fn insert(&mut self, commands: &mut Commands, particle: Particle) {
        let index = particle.in_chunk_index();
        if self.particles[index].is_none() {
            self.particles[index] = Some(particle.spawn(commands));
            self.count += 1;
        }
    }

    pub fn remove(&mut self, commands: &mut Commands, particle: &Particle) {
        let index = particle.in_chunk_index();
        if let Some(entity) = self.particles[index] {
            commands.entity(entity).despawn();
            self.particles[index] = None;
            self.count -= 1;
        }
    }
}
