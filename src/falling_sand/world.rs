use super::particles::{Particle, ParticleData};
use bevy::prelude::*;
use bevy::utils::HashMap;
use itertools::Itertools;

#[derive(Resource)]
pub struct World {
    particles: HashMap<(i32, i32), Entity>,
    to_wake: Vec<(i32, i32)>,
    ground_level: i32,
}

impl World {
    pub fn new(ground_level: i32) -> Self {
        World {
            particles: HashMap::new(),
            to_wake: Vec::new(),
            ground_level,
        }
    }

    pub fn is_empty(&self, position: IVec2) -> bool {
        let x = position.x;
        let y = position.y;
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

        let x = position.x;
        let y = position.y;

        self.particles
            .insert((x, y), particle.spawn(commands, &position).unwrap());
    }

    fn update_position(&mut self, old_position: IVec2, new_position: IVec2) {
        if self.is_empty(old_position) || !self.is_empty(new_position) {
            return;
        }
        let old = (old_position.x, old_position.y);
        let new = (new_position.x, new_position.y);
        let id = self.particles.get(&old).unwrap().clone();

        self.particles.remove(&old);
        self.particles.insert(new, id);
    }

    pub fn update(&mut self, query: &mut Query<(&mut Transform, &mut ParticleData)>) {
        let to_update: Vec<(i32, i32)> = self
            .particles
            .iter()
            .filter(|(_, id)| {
                let (_, data) = query.get(**id).unwrap();
                !data.is_asleep() || self.to_wake.contains(&(data.position.x, data.position.y))
            })
            .map(|(key, _)| key.clone())
            .collect();
        self.to_wake = Vec::new();
        for key in to_update.iter().sorted() {
            let (mut transform, mut data) = query.get_mut(self.particles[key]).unwrap();

            if let Some(new_position) = data.__type.clone().update(&mut data, &self) {
                transform.translation = new_position.as_vec2().extend(transform.translation.z);
                self.update_position(data.position, new_position);
                data.position = new_position;
                for i in 0..3 {
                    for j in 0..3 {
                        if i == 1 && j == 1 {
                            continue;
                        }
                        self.to_wake
                            .push((data.position.x + i - 1, data.position.y + j - 1));
                    }
                }
            }
        }
    }
}
