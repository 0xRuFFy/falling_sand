mod sand;

use super::world;
use bevy::prelude::*;
use bevy::sprite::Anchor;
pub use sand::Sand;

pub trait Particle {
    fn spawn(&self, commands: &mut Commands, position: &Vec2); // TODO: spawn location
}

pub fn base_spawn(commands: &mut Commands, position: &Vec2, particle: world::Particle) {
    if particle.is_empty() {
        return;
    }

    println!("particle: {:?}", position);
    let position = Vec3::new(position.x, position.y, 0.0);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: particle.color().unwrap_or(world::Particle::default_color()),
                custom_size: Some(Vec2::new(1.0, 1.0)),
                anchor: Anchor::BottomLeft,
                ..default()
            },
            transform: Transform::from_translation(position),
            ..default()
        },
        particle,
    ));
}
