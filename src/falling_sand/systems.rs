use super::events::{DespawnParticleEvent, SpawnParticleEvent};
use super::particle::{ParticleTag, ParticleType};
use super::resources::{CurrentParticleType, ParticleBrush, SpawnTimer};
use super::world::World;
use crate::systems::PIXELS_PER_UNIT;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn setup(mut commands: Commands) {
    commands.insert_resource(World::new());
    commands.insert_resource(CurrentParticleType::default());
    commands.insert_resource(Time::<Fixed>::from_seconds(0.016));
    commands.insert_resource(ParticleBrush::default());
    commands.insert_resource(SpawnTimer::new(0.08));
}

pub fn update(mut spawn_timer: ResMut<SpawnTimer>, time: Res<Time>) {
    spawn_timer.tick(time)
}

pub fn fixed_update(mut world: ResMut<World>, mut query: Query<&mut Transform, With<ParticleTag>>) {
    world.update(&mut query);
}

pub fn spawn_particle(
    mut commands: Commands,
    mut events: EventReader<SpawnParticleEvent>,
    mut world: ResMut<World>,
    mut spawn_timer: ResMut<SpawnTimer>,
    __type: Res<CurrentParticleType>,
    brush: Res<ParticleBrush>,
) {
    if !spawn_timer.guard() {
        return;
    }
    for event in events.read() {
        for offset in brush.get() {
            world.insert(&mut commands, &__type, event.position + *offset);
        }
    }
}

pub fn despawn_particle(
    mut commands: Commands,
    mut events: EventReader<DespawnParticleEvent>,
    mut world: ResMut<World>,
) {
    for event in events.read() {
        world.remove(&mut commands, event.position);
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

pub fn keyboard_input(
    mut brush: ResMut<ParticleBrush>,
    mut current_type: ResMut<CurrentParticleType>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Minus) {
        brush.decrease();
    } else if keyboard_input.just_pressed(KeyCode::Plus) {
        brush.increase();
    }

    if keyboard_input.just_pressed(KeyCode::Key1) {
        current_type.0 = ParticleType::Sand;
    } else if keyboard_input.just_pressed(KeyCode::Key2) {
        current_type.0 = ParticleType::Water;
    }
}
