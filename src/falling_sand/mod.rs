mod systems;

mod particles;
mod world;

use bevy::prelude::*;

use self::systems::setup;

pub struct FallingSandPlugin;

impl Plugin for FallingSandPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
