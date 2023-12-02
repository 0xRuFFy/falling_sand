use super::world::{self, Particle};
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

    pub fn execute(&self, commands: &mut Commands) {
        if self.particle.is_empty() {
            return;
        }
        self.particle.get().unwrap().spawn(commands, &self.position);
    }
}

pub fn spawn_particle(
    mut commands: Commands,
    mut spanw_event: EventReader<SpawnParticleEvent>,
    mut world: ResMut<world::World>,
) {
    for event in spanw_event.read() {
        // TODO: check spawn location
        // TODO: add to quadtree/grid
        if world.is_empty(event.position) {
            world.insert(event.position, event.particle);
            event.execute(&mut commands);
        }
    }
}
