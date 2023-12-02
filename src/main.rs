mod components;
mod plugins;
mod resources;
mod states;
mod systems;

mod falling_sand;
mod ui;

use bevy::prelude::*;
use plugins::CustomDefaultPlugin;
use systems::{setup, testing_exit};
use ui::UiPlugin;

fn main() {
    App::new()
        .add_plugins(CustomDefaultPlugin)
        .add_systems(PreStartup, setup)
        .add_plugins((UiPlugin,))
        .add_systems(Update, testing_exit)
        .run();
}
