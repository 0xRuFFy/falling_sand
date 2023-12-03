use super::world;
use bevy::prelude::*;
use bevy::sprite::Anchor;

pub fn base_spawn(commands: &mut Commands, position: &Vec2, particle: Particle) {
    if particle.is_empty() {
        return;
    }

    let position = Vec3::new(position.x, position.y, 0.0);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: particle.color().unwrap_or(Particle::default_color()),
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

// TODO: FIX THIS MESS

const PARTICLE_DEFAULT_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);
const PARTICLE_SAND_COLOR: Color = Color::rgb(1.0, 0.824, 0.196);
const PARTICLE_WATER_COLOR: Color = Color::rgb(0.063, 0.459, 0.91);

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum Particle {
    Empty,
    Sand,
    Water,
}

impl Particle {
    pub fn is_empty(&self) -> bool {
        *self == Particle::Empty
    }

    pub fn default_color() -> Color {
        PARTICLE_DEFAULT_COLOR
    }

    pub fn color(&self) -> Option<Color> {
        Some(match self {
            Particle::Empty => return None,
            Particle::Sand => PARTICLE_SAND_COLOR,
            Particle::Water => PARTICLE_WATER_COLOR,
        })
    }

    pub fn update(&mut self, position: Vec2, world: &world::World) -> Option<Vec2> {
        if position.y <= 0.0 {
            return None;
        }

        match self {
            Particle::Empty => None,
            Particle::Sand => {
                let mut desired_position = Vec2::new(position.x, position.y - 1.0);
                if world.is_empty(desired_position) {
                    return Some(desired_position);
                }

                desired_position.x -= 1.0;
                if world.is_empty(desired_position) {
                    return Some(desired_position);
                }

                desired_position.x += 2.0;
                if world.is_empty(desired_position) {
                    return Some(desired_position);
                }

                None
            }
            Particle::Water => {
                let mut desired_position = Vec2::new(position.x, position.y - 1.0);
                if world.is_empty(desired_position) {
                    return Some(desired_position);
                }

                desired_position.x -= 1.0;
                if world.is_empty(desired_position) {
                    return Some(desired_position);
                }

                desired_position.x += 2.0;
                if world.is_empty(desired_position) {
                    return Some(desired_position);
                }

                desired_position.y += 1.0;
                if world.is_empty(desired_position) {
                    return Some(desired_position);
                }

                desired_position.y -= 2.0;
                if world.is_empty(desired_position) {
                    return Some(desired_position);
                }

                None
            }
        }
    }
}
