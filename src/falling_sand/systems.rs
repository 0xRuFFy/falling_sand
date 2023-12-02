use super::events::SpawnParticleEvent;
use super::world::{self, Particle};
use crate::systems::PIXELS_PER_UNIT;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn setup(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    if let Ok(window) = window_query.get_single() {
        commands.insert_resource(world::World::new(
            // (window.width() * PIXELS_PER_UNIT) as usize,
            // (window.height() * PIXELS_PER_UNIT) as usize,
        ));
    }
}

pub fn update(mut world: ResMut<world::World>, mut query: Query<(&mut Transform, &mut Particle)>) {
    world.update(&mut query);
}

pub fn spawn_particle_mouse(
    mouse_button_input: Res<Input<MouseButton>>,
    mut spawn_event: EventWriter<SpawnParticleEvent>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if mouse_button_input.pressed(MouseButton::Left) {
        if let Ok(window) = window_query.get_single() {
            if let Some(position) = window.cursor_position() {
                let position = Vec2::new(
                    (position.x as u32 / PIXELS_PER_UNIT as u32) as f32,
                    ((window.height() - position.y) as u32 / PIXELS_PER_UNIT as u32) as f32,
                );
                spawn_event.send(SpawnParticleEvent::new(position, world::Particle::Sand))
            }
        }
    }
}
