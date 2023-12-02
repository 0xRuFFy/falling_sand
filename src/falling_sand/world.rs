use super::particles;
use bevy::{prelude::*, utils::HashMap};
use std::sync::Arc;

const PARTICLE_DEFAULT_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);
const PARTICLE_SAND_COLOR: Color = Color::rgb(1.0, 0.824, 0.196);

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum Particle {
    Empty,
    Sand,
}

impl Particle {
    pub fn is_empty(&self) -> bool {
        *self == Particle::Empty
    }

    pub fn default_color() -> Color {
        PARTICLE_DEFAULT_COLOR
    }

    pub fn color(&self) -> Option<Color> {
        match self {
            Particle::Empty => None,
            Particle::Sand => Some(PARTICLE_SAND_COLOR),
        }
    }

    pub fn get(&self) -> Option<Box<dyn particles::Particle + Send + Sync>> {
        Some(match self {
            Particle::Empty => return None,
            Particle::Sand => Box::new(particles::Sand::new()),
        })
    }

    pub fn update(&mut self, position: Vec2, world: &World) -> Option<Vec2> {
        if position.y <= 0.0 {
            return None;
        }

        match self {
            Particle::Empty => None,
            Particle::Sand => {
                let mut desired_position = Vec2::new(position.x, position.y - 1.0);
                if world.is_empty(desired_position) {
                    return Some(desired_position);
                }

                desired_position.x -= 1.0;
                if world.is_empty(desired_position) {
                    return Some(desired_position);
                }

                desired_position.x += 2.0;
                if world.is_empty(desired_position) {
                    return Some(desired_position);
                }

                None
            }
        }
    }
}

type TParticle = Box<dyn particles::Particle + Send + Sync>;

#[derive(Resource)]
pub struct World {
    particles: HashMap<(usize, usize), TParticle>,
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
            return Some(p.get_type());
        }
        None
    }

    pub fn insert(&mut self, position: Vec2, particle: Particle) {
        let x = position.x as usize;
        let y = position.y as usize;
        if particle.is_empty() {
            self.particles.remove(&(x, y));
        }
        self.particles.insert((x, y), particle.get().unwrap());
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
