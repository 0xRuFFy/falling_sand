mod debug_outline;
mod systems;
mod ui_components;

use self::systems::setup;
use self::ui_components::fps_display;
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (
                (debug_outline::toggle_visibility, fps_display::update),
                debug_outline::draw_outline,
            )
                .chain(),
        );
    }
}
