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
const PARTICLE_WATER_COLOR: PColor = &[
    Color::rgb(0.392, 0.49, 0.961),
    Color::rgb(0.314, 0.322, 0.871),
    Color::rgb(0.463, 0.471, 0.91),
    Color::rgb(0.267, 0.278, 0.988),
];

struct MovementOptionGroup(&'static [IVec2]);

impl MovementOptionGroup {
    fn choose(&self) -> Option<&IVec2> {
        let mut rng = rand::thread_rng();
        self.0.choose(&mut rng)
    }
}

type PMovement = &'static [MovementOptionGroup];
const PARTICLE_SAND_MOVEMENT: PMovement = &[
    MovementOptionGroup(&[IVec2::new(0, -1)]),
    MovementOptionGroup(&[IVec2::new(1, -1), IVec2::new(-1, -1)]),
];

const PARTICLE_WATER_MOVEMENT: PMovement = &[
    MovementOptionGroup(&[IVec2::new(0, -1)]),
    MovementOptionGroup(&[IVec2::new(1, 0), IVec2::new(-1, 0)]),
];

#[derive(Component)]
pub struct ParticleData {
    pub __type: Particle,
    pub position: IVec2,
}

impl ParticleData {
    pub fn new(__type: Particle, position: IVec2) -> Self {
        Self {
            __type,
            position,
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Particle {
    Empty,
    Sand,
    Water,
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
            Particle::Water => PARTICLE_WATER_COLOR,
        }
        .choose(&mut rng)
    }

    fn movement(&self) -> Option<PMovement> {
        match self {
            Particle::Empty => None,
            Particle::Sand => Some(PARTICLE_SAND_MOVEMENT),
            Particle::Water => Some(PARTICLE_WATER_MOVEMENT),
        }
    }

    pub fn spawn(&self, commands: &mut Commands, position: &IVec2) -> Option<Entity> {
        if self.is_empty() {
            return None;
        }

        Some(
            commands
                .spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: *self.color().unwrap_or(PARTICLE_DEFAULT_COLOR),
                            custom_size: Some(Vec2::new(1.0, 1.0)),
                            anchor: Anchor::BottomLeft,
                            ..default()
                        },
                        transform: Transform::from_translation(position.extend(0).as_vec3()),
                        ..default()
                    },
                    ParticleData::new(*self, *position),
                    *self, // TODO: Remove
                ))
                .id(),
        )
    }

    pub fn update(&mut self, data: &ParticleData, world: &world::World) -> Option<IVec2> {
        // TODO: Stop using a const speed and switch it for gravity -> need to chage the collision
        //       logic to account speed != 1.
        if self.is_empty() {
            return None;
        }

        if let Some(movement) = self.movement() {
            for group in movement {
                let dir = group.choose().unwrap();
                let desired_position = data.position + *dir;
                if world.is_empty(desired_position) {
                    return Some(desired_position);
                }
            }
        };

        None
    }
}
