use super::{base_spawn, Particle};
use crate::falling_sand::world;
use bevy::prelude::*;

pub struct Sand {}

impl Sand {
    pub fn new() -> Self {
        Sand {}
    }
}

impl Particle for Sand {
    fn spawn(&self, commands: &mut Commands, position: &Vec2) {
        base_spawn(commands, position, world::Particle::Sand);
    }
}
