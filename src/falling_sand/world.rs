use super::particle::{Particle, ParticleTag};
use super::resources::CurrentParticleType;
use crate::utils::VecTransform;
use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};
use rand::seq::SliceRandom;

// pub const CHUNK_SIZE: usize = 16;
const GRAVITY: f32 = -0.25;
const NEIGHBORS: [IVec2; 8] = [
    IVec2::new(0, 1),
    IVec2::new(0, -1),
    IVec2::new(1, 0),
    IVec2::new(-1, 0),
    IVec2::new(1, 1),
    IVec2::new(1, -1),
    IVec2::new(-1, 1),
    IVec2::new(-1, -1),
];

#[derive(Resource, Debug)]
pub struct World {
    entities: Vec<Entity>,
    particles: HashMap<Entity, Particle>,
    to_wake: HashSet<Entity>,
    ground_level: i32,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            particles: HashMap::new(),
            to_wake: HashSet::new(),
            ground_level: 0,
        }
    }

    pub fn empty_at(&self, position: IVec2) -> bool {
        position.y >= self.ground_level && self.get_at(position).is_none()
    }

    pub fn get_at(&self, position: IVec2) -> Option<&Particle> {
        self.particles
            .iter()
            .find(|(_, particle)| particle.position == position)
            .map(|(_, particle)| particle)
    }

    fn get_neighbors(&self, position: IVec2) -> Vec<Entity> {
        let mut neighbors = Vec::new();
        for offset in NEIGHBORS {
            neighbors.push(position + offset);
        }
        self.particles
            .iter()
            .filter(|(_, particle)| neighbors.contains(&particle.position))
            .map(|(id, _)| *id)
            .collect()
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
        // TODO: FIX SLEEPING WATER
        let mut rng = rand::thread_rng();
        let mut count = 0;

        let mut to_update = self
            .particles
            .iter()
            .filter(|(_, particle)| !particle.asleep() || self.to_wake.contains(particle.id()))
            .map(|(id, _)| id.clone())
            .collect::<Vec<_>>();

        self.to_wake.clear();
        to_update
            .iter_mut()
            .fold(HashMap::<IVec2, Vec<Entity>>::new(), |mut map, id| {
                count += 1;
                self.particles.get_mut(id).unwrap().accelerate(GRAVITY);
                if let Some(new_position) = self.particles.get(id).unwrap().movement(self) {
                    map.entry(new_position).or_insert_with(Vec::new).push(*id);
                    self.particles.get_mut(id).unwrap().wake();
                    for neighbor in self.get_neighbors(new_position) {
                        if neighbor == *id {
                            continue;
                        }
                        self.to_wake.insert(neighbor);
                    }
                } else {
                    self.particles.get_mut(id).unwrap().sleep();
                }
                map
            })
            .iter()
            .for_each(|(new_position, ids)| {
                let pick = ids.choose(&mut rng).unwrap();
                self.update_single(*pick, *new_position);
                if let Ok(mut transform) = query.get_mut(*pick) {
                    transform.translation = new_position.as_vec3();
                }
            });

        // println!("Particles: {}/{}", count, self.particles.len());
    }
}
