use super::events::SpawnParticleEvent;
use super::particles::base_spawn;
use super::world;
use crate::systems::PIXELS_PER_UNIT;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn setup(mut commands: Commands) {
    commands.insert_resource(world::World::new());
    base_spawn(&mut commands, &Vec2::new(0., 0.), world::Particle::Sand);
}

pub fn update(mut world: ResMut<world::World>) {
    world.update();
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
                    position.x / PIXELS_PER_UNIT,
                    (window.height() - position.y) / PIXELS_PER_UNIT,
                );
                spawn_event.send(SpawnParticleEvent::new(position, world::Particle::Sand))
            }
        }
    }
}
