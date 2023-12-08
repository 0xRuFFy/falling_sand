use super::events::{DespawnParticleEvent, SpawnParticleEvent};
use super::resources::CurrentParticleType;
use super::world::World;
use crate::systems::PIXELS_PER_UNIT;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

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

pub fn despawn_particle(
    mut commands: Commands,
    mut events: EventReader<DespawnParticleEvent>,
    mut world: ResMut<World>,
) {
    for event in events.read() {
        world.remove_particle(&mut commands, event.position);
    }
}

pub fn mouse_input(
    mut spawn_events: EventWriter<SpawnParticleEvent>,
    mut despawn_events: EventWriter<DespawnParticleEvent>,
    mouse_button_input: Res<Input<MouseButton>>,
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
                spawn_events.send(SpawnParticleEvent::new(position));
            } else if mouse_button_input.pressed(MouseButton::Right) {
                despawn_events.send(DespawnParticleEvent::new(position));
            }
        }
    }
}
