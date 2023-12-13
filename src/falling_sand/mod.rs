mod events;
mod resources;
mod systems;

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
                (
                    (mouse_input, keyboard_input),
                    (spawn_particle, despawn_particle),
                )
                    .chain(),
            )
            .add_systems(Update, update)
            .add_systems(FixedUpdate, fixed_update);
    }
}
