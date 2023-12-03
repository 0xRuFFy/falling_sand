use super::particles::{base_spawn, Particle};
use super::world;
use bevy::prelude::*;

#[derive(Event)]
pub struct SpawnParticleEvent {
    position: Vec2,
    particle: Particle,
}

impl SpawnParticleEvent {
    pub fn new(position: Vec2, particle: Particle) -> Self {
        SpawnParticleEvent { position, particle }
    }
}

pub fn spawn_particle(
    mut commands: Commands,
    mut spanw_event: EventReader<SpawnParticleEvent>,
    mut world: ResMut<world::World>,
) {
    for event in spanw_event.read() {
        if world.is_empty(event.position) {
            world.insert(event.position, event.particle);
            base_spawn(&mut commands, &event.position, event.particle);
        }
    }
}
