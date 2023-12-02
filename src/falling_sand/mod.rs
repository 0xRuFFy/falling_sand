mod events;
mod systems;

mod particles;
mod world;

use self::events::{spawn_particle, SpawnParticleEvent};
use self::systems::{setup, spawn_particle_mouse, update};
use bevy::prelude::*;

pub struct FallingSandPlugin;

impl Plugin for FallingSandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnParticleEvent>()
            .add_systems(Startup, setup)
            .add_systems(PreUpdate, (spawn_particle_mouse, spawn_particle).chain())
            .add_systems(Update, update);
    }
}
