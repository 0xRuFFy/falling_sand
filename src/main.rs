mod components;
mod plugins;
mod resources;
mod states;
mod systems;
mod utils;

mod falling_sand;
mod ui;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use falling_sand::FallingSandPlugin;
use plugins::CustomDefaultPlugin;
use systems::{setup, testing_exit};
use ui::UiPlugin;

fn main() {
    App::new()
        .add_plugins(CustomDefaultPlugin)
        .add_systems(PreStartup, setup)
        .add_plugins((FrameTimeDiagnosticsPlugin, UiPlugin, FallingSandPlugin))
        .add_systems(Update, testing_exit)
        .run();
}
