use super::particle::Particle;
use super::resources::CurrentParticleType;
use bevy::prelude::*;
use bevy::utils::HashMap;

// pub const CHUNK_SIZE: usize = 16;

#[derive(Resource)]
pub struct World {
    pub entities: Vec<Entity>,
    pub particles: HashMap<Entity, Particle>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            particles: HashMap::new(),
        }
    }

    pub fn get(&self, id: Entity) -> Option<&Particle> {
        self.particles.get(&id)
    }

    pub fn insert(
        &mut self,
        commands: &mut Commands,
        particle_type: &Res<CurrentParticleType>,
        position: IVec2,
    ) {
        let particle = particle_type.0.create(commands, position);
        self.entities.push(*particle.id());
        self.particles.insert(*particle.id(), particle);
    }

    pub fn remove(&mut self, commands: &mut Commands, position: IVec2) {
        if let Some(id) = self
            .particles
            .iter()
            .find(|(_, particle)| particle.position == position)
            .map(|(id, _)| *id)
        {
            self.particles.remove(&id);
            self.entities.retain(|entity| *entity != id);
            commands.entity(id).despawn();
        }
    }

    pub fn update_single(&mut self, commands: &mut Commands, id: Entity, position: IVec2) {
        if let Some(particle) = self.particles.get_mut(&id) {
            particle.position = position;
        }
    }
}
