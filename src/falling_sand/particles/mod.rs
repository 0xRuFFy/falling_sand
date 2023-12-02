mod sand;

use bevy::prelude::*;

use super::world;

pub trait Particle {
    fn spawn(&self, commands: &mut Commands); // TODO: spawn location
}

pub fn base_spawn(commands: &mut Commands, position: Vec2, particle: world::Particle) {
    if particle == world::Particle::Empty {
        return;
    }
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: particle.color().unwrap_or(world::Particle::default_color()),
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..default()
            },
            transform: Transform::from_translation(position.extend(0.0)),
            ..default()
        },
        particle,
    ));
}
