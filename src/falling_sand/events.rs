use bevy::prelude::*;

#[derive(Event)]
pub struct SpawnParticleEvent {
    pub position: IVec2,
}

impl SpawnParticleEvent {
    pub fn new(position: IVec2) -> Self {
        Self { position }
    }
}
