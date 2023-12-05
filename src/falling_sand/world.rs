use super::particles::{Particle, ParticleData};
use bevy::prelude::*;
use bevy::utils::HashMap;
use itertools::Itertools;

#[derive(Resource)]
pub struct World {
    particles: HashMap<(usize, usize), Entity>,
    ground_level: i32,
}

impl World {
    pub fn new(ground_level: i32) -> Self {
        World {
            particles: HashMap::new(),
            ground_level,
        }
    }

    pub fn is_empty(&self, position: IVec2) -> bool {
        let x = position.x as usize;
        let y = position.y as usize;
        self.ground_level <= position.y && self.particles.get(&(x, y)).is_none()
    }

    // pub fn get(&self, position: Vec2) -> Option<Particle> {
    //     let x = position.x as usize;
    //     let y = position.y as usize;
    //     if let Some(p) = self.particles.get(&(x, y)) {
    //         return Some(p.clone());
    //     }
    //     None
    // }

    pub fn insert(&mut self, commands: &mut Commands, position: IVec2, particle: Particle) {
        if particle.is_empty() || !self.is_empty(position) {
            return;
        }

        let x = position.x as usize;
        let y = position.y as usize;

        self.particles
            .insert((x, y), particle.spawn(commands, &position).unwrap());
    }

    fn update_position(&mut self, old_position: IVec2, new_position: IVec2) {
        if self.is_empty(old_position) || !self.is_empty(new_position) {
            return;
        }
        let old = (old_position.x as usize, old_position.y as usize);
        let new = (new_position.x as usize, new_position.y as usize);
        let id = self.particles.get(&old).unwrap().clone();

        self.particles.remove(&old);
        self.particles.insert(new, id);
    }

    pub fn update(&mut self, query: &mut Query<(&mut Transform, &mut ParticleData)>) {
        // TODO: implement as sleep state for particles that are not moving /
        //       wake a particle up if a nearby particle is moving then go back to sleep
        //       -> this should reduce the number of updated particles drastically!

        // NOTE: in case of getting only none sleeping particles:
        //       Don't clone all particles, but only those that are not sleeping here
        for key in self.particles.clone().keys().sorted() {
            let (mut transform, mut data) = query.get_mut(self.particles[key]).unwrap();

            if let Some(new_position) = data.__type.clone().update(&data, &self) {
                transform.translation = new_position.as_vec2().extend(transform.translation.z);
                self.update_position(data.position, new_position);
                data.position = new_position;
            }
        }
    }
}
