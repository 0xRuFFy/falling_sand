use super::resources::ShowOutlines;
use crate::falling_sand::world::{World, CHUNK_SIZE};
use bevy::prelude::*;

pub fn setup(commands: &mut Commands) {
    commands.insert_resource(ShowOutlines(true));
}

pub fn toggle_visibility(mut enabled: ResMut<ShowOutlines>, key: Res<Input<KeyCode>>) {
    if key.just_pressed(KeyCode::Minus) {
        enabled.0 = !enabled.0;
    }
}

pub fn draw_outline(enabled: Res<ShowOutlines>, world: Res<World>, mut gizmos: Gizmos) {
    if !enabled.0 {
        return;
    }

    for position in world.chunk_positions() {
        let chunk_size_f32 = CHUNK_SIZE as f32;
        let size = Vec2::splat(chunk_size_f32);
        let center = position.as_vec2() * chunk_size_f32 + size / 2.0;
        gizmos.rect_2d(center, 0.0, size, Color::RED);
    }
}
