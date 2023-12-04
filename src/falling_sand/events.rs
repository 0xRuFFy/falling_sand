use super::particles::Particle;
use super::world;
use crate::falling_sand::resources::SpawnTimer;
use bevy::prelude::*;

#[derive(Event)]
pub struct SpawnParticleEvent {
    position: Vec2,
    particle: Particle,
}

impl SpawnParticleEvent {
    pub fn new(position: Vec2, particle: Particle) -> Self {
        SpawnParticleEvent {
            position,
            particle,
        }
    }
}

pub fn spawn_particle(
    mut commands: Commands,
    mut spawn_event: EventReader<SpawnParticleEvent>,
    mut world: ResMut<world::World>,
    mut spawn_timer: ResMut<SpawnTimer>,
) {
    for event in spawn_event.read() {
        if !spawn_timer.guard() {
            return;
        }

        if world.is_empty(event.position) {
            world.insert(event.position, event.particle);
            event.particle.spawn(&mut commands, &event.position);
        }
    }
}
