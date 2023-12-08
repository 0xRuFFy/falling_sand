mod events;
mod resources;
mod systems;

mod chunk;
mod particle;
mod world;

use bevy::prelude::*;
use systems::*;

use self::events::{DespawnParticleEvent, SpawnParticleEvent};

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
