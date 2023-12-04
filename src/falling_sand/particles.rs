use super::world;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use rand::prelude::SliceRandom;

// TODO: FIX THIS MESS

type PColor = &'static [Color];
const PARTICLE_DEFAULT_COLOR: &Color = &Color::rgb(0.0, 0.0, 0.0);
const PARTICLE_EMPTY_COLOR: PColor = &[];
const PARTICLE_SAND_COLOR: PColor = &[
    Color::rgb(1.0, 0.824, 0.196),
    Color::rgb(0.949, 0.8, 0.141),
    Color::rgb(0.949, 0.733, 0.141),
    Color::rgb(0.878, 0.702, 0.212),
    Color::rgb(0.961, 0.812, 0.392),
];

struct MovementOptionGroup(&'static [Vec2]);

impl MovementOptionGroup {
    fn choose(&self) -> Option<&Vec2> {
        let mut rng = rand::thread_rng();
        self.0.choose(&mut rng)
    }
}

type PMovement = &'static [MovementOptionGroup];
const PARTICLE_SAND_MOVEMENT: PMovement = &[
    MovementOptionGroup(&[Vec2::new(0.0, -1.0)]),
    MovementOptionGroup(&[Vec2::new(1.0, -1.0), Vec2::new(-1.0, -1.0)]),
];

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum Particle {
    Empty,
    Sand,
}

impl Particle {
    pub fn is_empty(&self) -> bool {
        *self == Particle::Empty
    }

    fn color(&self) -> Option<&Color> {
        let mut rng = rand::thread_rng();
        match self {
            Particle::Empty => PARTICLE_EMPTY_COLOR,
            Particle::Sand => PARTICLE_SAND_COLOR,
        }
        .choose(&mut rng)
    }

    fn movement(&self) -> Option<PMovement> {
        match self {
            Particle::Empty => None,
            Particle::Sand => Some(PARTICLE_SAND_MOVEMENT),
        }
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
        if let Some(movement) = self.movement() {
            for group in movement {
                let dir = group.choose().unwrap();
                let desired_position = Vec2::new(position.x + dir.x, position.y + dir.y);
                if world.is_empty(desired_position) {
                    return Some(desired_position);
                }
            }
        };

        None
    }
}
