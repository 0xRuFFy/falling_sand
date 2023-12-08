use crate::systems::PIXELS_PER_UNIT;

use super::{events::SpawnParticleEvent, resources::CurrentParticleType, world::World};
use bevy::{prelude::*, window::PrimaryWindow};

pub fn setup(mut commands: Commands) {
    commands.insert_resource(World::new());
    commands.insert_resource(CurrentParticleType::default());
}

pub fn spawn_particle(
    mut commands: Commands,
    mut events: EventReader<SpawnParticleEvent>,
    mut world: ResMut<World>,
    __type: Res<CurrentParticleType>,
) {
    for event in events.read() {
        world.add_particle(&mut commands, __type.create(event.position, IVec2::ZERO));
    }
}

pub fn mouse_input(
    mut spawn_events: EventWriter<SpawnParticleEvent>,
    mouse_button_input: Res<Input<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if !mouse_button_input.pressed(MouseButton::Left) {
        return;
    }

    if let Ok(window) = window_query.get_single() {
        if let Some(position) = window.cursor_position() {
            let position = IVec2::new(
                position.x as i32 / PIXELS_PER_UNIT as i32,
                (window.height() - position.y) as i32 / PIXELS_PER_UNIT as i32,
            );

            spawn_events.send(SpawnParticleEvent::new(position));
        }
    }
}
