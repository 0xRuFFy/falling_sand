use bevy::prelude::*;

use super::{particles, world};

pub fn setup(mut commands: Commands) {
    particles::base_spawn(&mut commands, Vec2::new(0.0, 0.0), world::Particle::Sand);
}
