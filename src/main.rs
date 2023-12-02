mod components;
mod plugins;
mod resources;
mod states;
mod systems;

use bevy::prelude::*;
use plugins::CustomDefaultPlugin;
use systems::{setup, testing_exit};

fn main() {
    App::new()
        .add_plugins(CustomDefaultPlugin)
        .add_systems(PreStartup, setup)
        .add_systems(Update, testing_exit)
        .run();
}
