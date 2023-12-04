use super::world;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use rand::prelude::SliceRandom;

// TODO: FIX THIS MESS

const PARTICLE_DEFAULT_COLOR: &Color = &Color::rgb(0.0, 0.0, 0.0);
const PARTICLE_EMPTY_COLOR: &'static [Color] = &[];
const PARTICLE_SAND_COLOR: &'static [Color] = &[Color::rgb(1.0, 0.824, 0.196)];

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum Particle {
    Empty,
    Sand,
}

impl Particle {
    pub fn is_empty(&self) -> bool {
        *self == Particle::Empty
    }

    pub fn color(&self) -> Option<&Color> {
        let mut rng = rand::thread_rng();
        match self {
            Particle::Empty => PARTICLE_EMPTY_COLOR,
            Particle::Sand => PARTICLE_SAND_COLOR,
        }
        .choose(&mut rng)
    }

    pub fn spawn(&self, commands: &mut Commands, position: &Vec2) {
        if self.is_empty() {
            return;
        }

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: *self.color().unwrap_or(PARTICLE_DEFAULT_COLOR),
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    anchor: Anchor::BottomLeft,
                    ..default()
                },
                transform: Transform::from_translation(position.extend(0.0)),
                ..default()
            },
            *self,
        ));
    }

    pub fn update(&mut self, position: Vec2, world: &world::World) -> Option<Vec2> {
        // if position.y <= 0.0 {
        //     return None;
        // }

        // None
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
        }
    }
}
