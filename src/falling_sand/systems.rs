use super::events::SpawnParticleEvent;
use super::particles::Particle;
use super::world;
use crate::falling_sand::resources::{Brush, SpawnTimer};
use crate::systems::PIXELS_PER_UNIT;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn setup(mut commands: Commands) {
    commands.insert_resource(world::World::new(0));
    // commands.insert_resource(SpawnTimer::new(0.25));
    commands.insert_resource(SpawnTimer::default());
    commands.insert_resource(Time::<Fixed>::from_seconds(0.016));
    commands.insert_resource(Brush::default());
}

pub fn update(mut spawn_timer: ResMut<SpawnTimer>, time: Res<Time>) {
    spawn_timer.tick(time)
}

pub fn fixed_update(
    mut world: ResMut<world::World>,
    mut query: Query<(&mut Transform, &mut Particle)>,
) {
    world.update(&mut query);
}

pub fn spawn_particle_mouse(
    mouse_button_input: Res<Input<MouseButton>>,
    mut spawn_event: EventWriter<SpawnParticleEvent>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if !mouse_button_input.any_pressed([MouseButton::Left, MouseButton::Right]) {
        return;
    }
    if let Ok(window) = window_query.get_single() {
        if let Some(position) = window.cursor_position() {
            let position = IVec2::new(
                position.x as i32 / PIXELS_PER_UNIT as i32,
                (window.height() - position.y) as i32 / PIXELS_PER_UNIT as i32,
            );

            if mouse_button_input.pressed(MouseButton::Left) {
                spawn_event.send(SpawnParticleEvent::new(position, Particle::Sand))
            } else if mouse_button_input.pressed(MouseButton::Right) {
                spawn_event.send(SpawnParticleEvent::new(position, Particle::Water))
            }
        }
    }
}
