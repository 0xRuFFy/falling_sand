use super::particle::{Particle, ParticleTag};
use super::resources::CurrentParticleType;
use crate::utils::VecTransform;
use bevy::prelude::*;
use bevy::utils::HashMap;
use rand::seq::SliceRandom;

// pub const CHUNK_SIZE: usize = 16;

#[derive(Resource, Debug)]
pub struct World {
    entities: Vec<Entity>,
    particles: HashMap<Entity, Particle>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            particles: HashMap::new(),
        }
    }

    pub fn empty_at(&self, position: IVec2) -> bool {
        self.get_at(position).is_none()
    }

    pub fn get_at(&self, position: IVec2) -> Option<&Particle> {
        // self.entitiy_positions
        //     .get(&position)
        //     .and_then(|id| self.particles.get(id))
        self.particles
            .iter()
            .find(|(_, particle)| particle.position == position)
            .map(|(_, particle)| particle)
    }

    pub fn insert(
        &mut self,
        commands: &mut Commands,
        particle_type: &Res<CurrentParticleType>,
        position: IVec2,
    ) {
        if !self.empty_at(position) {
            return;
        }
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

    pub fn update_single(&mut self, id: Entity, position: IVec2) {
        if let Some(particle) = self.particles.get_mut(&id) {
            particle.position = position;
        }
    }

    pub fn update(&mut self, query: &mut Query<&mut Transform, With<ParticleTag>>) {
        // println!("{:#?}", self);
        let mut new_entities: HashMap<IVec2, Vec<Entity>> = HashMap::new();
        for (id, particle) in self.particles.iter() {
            if let Some(new_position) = particle.movement(self) {
                new_entities
                    .entry(new_position)
                    .or_insert_with(Vec::new)
                    .push(*id);
            }
        }

        // println!("{:#?}", new_entities);
        let mut rng = rand::thread_rng();
        for (new_position, ids) in new_entities {
            let pick = ids.choose(&mut rng).unwrap();
            self.update_single(*pick, new_position);
            if let Ok(mut transform) = query.get_mut(*pick) {
                transform.translation = new_position.as_vec3();
            }
        }
    }
}
