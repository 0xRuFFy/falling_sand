use super::particles::Particle;
use super::world;
use crate::falling_sand::resources::{Brush, SpawnTimer};
use bevy::prelude::*;
use std::ops::Add;

#[derive(Event)]
pub struct SpawnParticleEvent {
    position: IVec2,
    particle: Particle,
}

impl SpawnParticleEvent {
    pub fn new(position: IVec2, particle: Particle) -> Self {
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
    brush: Res<Brush>,
) {
    for event in spawn_event.read() {
        if !spawn_timer.guard() {
            return;
        }

        for offset in brush.get() {
            let position = event.position.add(*offset);
            if world.is_empty(position) {
                world.insert(&mut commands, position, event.particle);
                // event.particle.spawn(&mut commands, &event.position);
            }
        }
    }
}
