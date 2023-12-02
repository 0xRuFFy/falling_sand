use crate::systems::{VIEWPORT_RELATIVE_HEIGHT, VIEWPORT_RELATIVE_WIDTH};
use bevy::prelude::*;

const PARTICLE_DEFAULT_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);
const PARTICLE_SAND_COLOR: Color = Color::rgb(1.0, 0.824, 0.196);

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum Particle {
    Empty,
    Sand,
}

impl Particle {
    pub fn default_color() -> Color {
        PARTICLE_DEFAULT_COLOR
    }

    pub fn color(&self) -> Option<Color> {
        match self {
            Particle::Empty => None,
            Particle::Sand => Some(PARTICLE_SAND_COLOR),
        }
    }
}

pub struct World {
    grid: [[Particle; VIEWPORT_RELATIVE_WIDTH]; VIEWPORT_RELATIVE_HEIGHT],
}

impl World {
    pub fn new() -> Self {
        World {
            grid: [[Particle::Empty; VIEWPORT_RELATIVE_WIDTH]; VIEWPORT_RELATIVE_HEIGHT],
        }
    }
}
