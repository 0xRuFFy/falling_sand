use bevy::prelude::*;
use bevy::utils::HashMap;

use super::particles::Particle;

#[derive(Resource)]
pub struct World {
    particles: HashMap<(usize, usize), Particle>,
}

impl World {
    pub fn new() -> Self {
        World {
            particles: HashMap::new(),
        }
    }

    pub fn is_empty(&self, position: Vec2) -> bool {
        let x = position.x as usize;
        let y = position.y as usize;
        self.particles.get(&(x, y)).is_none()
    }

    pub fn get(&self, position: Vec2) -> Option<Particle> {
        let x = position.x as usize;
        let y = position.y as usize;
        if let Some(p) = self.particles.get(&(x, y)) {
            return Some(p.clone());
        }
        None
    }

    pub fn insert(&mut self, position: Vec2, particle: Particle) {
        let x = position.x as usize;
        let y = position.y as usize;
        if particle.is_empty() {
            self.particles.remove(&(x, y));
        }
        self.particles.insert((x, y), particle);
    }

    pub fn update(&mut self, query: &mut Query<(&mut Transform, &mut Particle)>) {
        let mut changed = Vec::new();

        for (mut transform, mut particle) in query.iter_mut() {
            if particle.is_empty() {
                continue;
            }

            let position = Vec2::new(transform.translation.x, transform.translation.y);

            if let Some(new_position) = particle.update(position, &self) {
                transform.translation = new_position.extend(transform.translation.z);
                changed.push((position, new_position, particle.clone()));
            }
        }

        for (position, new_position, p_type) in changed {
            self.particles.remove(&(position.x as usize, position.y as usize));
            self.insert(new_position, p_type);
        }
    }
}
