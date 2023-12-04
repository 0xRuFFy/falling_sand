use super::particles::Particle;
use bevy::prelude::*;
use bevy::utils::HashMap;
use itertools::Itertools;

#[derive(Resource)]
pub struct World {
    particles: HashMap<(usize, usize), Entity>,
    ground_level: f32,
}

impl World {
    pub fn new(ground_level: f32) -> Self {
        World {
            particles: HashMap::new(),
            ground_level,
        }
    }

    pub fn is_empty(&self, position: Vec2) -> bool {
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

    pub fn insert(&mut self, commands: &mut Commands, position: Vec2, particle: Particle) {
        if particle.is_empty() || !self.is_empty(position) {
            return;
        }

        let x = position.x as usize;
        let y = position.y as usize;

        self.particles
            .insert((x, y), particle.spawn(commands, &position).unwrap());
    }

    fn update_position(&mut self, old_position: Vec2, new_position: Vec2) {
        if self.is_empty(old_position) || !self.is_empty(new_position) {
            return;
        }
        let old = (old_position.x as usize, old_position.y as usize);
        let new = (new_position.x as usize, new_position.y as usize);
        let id = self.particles.get(&old).unwrap().clone();

        self.particles.remove(&old);
        self.particles.insert(new, id);
    }

    pub fn update(&mut self, query: &mut Query<(&mut Transform, &mut Particle)>) {
        // TODO: implement as sleep state for particles that are not moving /
        //       wake a particle up if a nearby particle is moving then go back to sleep
        //       -> this should reduce the number of updated particles drastically!

        for key in self.particles.clone().keys().sorted() {
            let (mut transform, mut particle) = query.get_mut(self.particles[key]).unwrap();

            let position = transform.translation.xy();

            if let Some(new_position) = particle.update(position, &self) {
                transform.translation = new_position.extend(transform.translation.z);
                self.update_position(position, new_position);
            }
        }
    }
}
