use super::debug_outline;
use super::ui_components::fps_display;
use crate::resources::ShowFps;
use bevy::prelude::*;

pub fn setup(mut commands: Commands, show_fps: Res<ShowFps>) {
    if **show_fps {
        fps_display::setup(&mut commands);
    }
    debug_outline::setup(&mut commands);
}
