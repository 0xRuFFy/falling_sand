use super::particles;
use bevy::prelude::*;

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

    pub fn get(&self) -> Option<Box<dyn particles::Particle>> {
        Some(match self {
            Particle::Empty => return None,
            Particle::Sand => Box::new(particles::Sand::new()),
        })
    }
}

#[derive(Resource)]
pub struct World {
    // grid: [[Particle; VIEWPORT_RELATIVE_WIDTH]; VIEWPORT_RELATIVE_HEIGHT], // TODO: swap for sparse quadtree
}

impl World {
    pub fn new() -> Self {
        World {
            // grid: [[Particle::Empty; VIEWPORT_RELATIVE_WIDTH]; VIEWPORT_RELATIVE_HEIGHT],
        }
    }

    pub fn update(&mut self) {
        // TODO: Update
    }
}
