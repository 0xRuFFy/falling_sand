mod events;
mod resources;
mod systems;

mod chunk;
mod particle;
pub mod world;

use self::events::{DespawnParticleEvent, SpawnParticleEvent};
use bevy::prelude::*;
use systems::*;

pub struct FallingSandPlugin;

impl Plugin for FallingSandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnParticleEvent>()
            .add_event::<DespawnParticleEvent>()
            .add_systems(Startup, setup)
            .add_systems(
                PreUpdate,
                (mouse_input, (spawn_particle, despawn_particle)).chain(),
            );
    }
}
