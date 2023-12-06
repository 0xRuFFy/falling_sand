use super::world;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use rand::prelude::SliceRandom;

const GRAVITY: Vec2 = Vec2::new(0.0, -0.25);
const TIME_UNTIL_SLEEP: u32 = 40;

type PColor = &'static [Color];
const PARTICLE_DEFAULT_COLOR: &Color = &Color::rgb(0.0, 0.0, 0.0);
const PARTICLE_EMPTY_COLOR: PColor = &[];
const PARTICLE_SAND_COLOR: PColor = &[
    Color::rgb(0.965, 0.843, 0.69),
    Color::rgb(0.949, 0.824, 0.663),
    Color::rgb(0.925, 0.8, 0.635),
    Color::rgb(0.906, 0.769, 0.588),
    Color::rgb(0.882, 0.749, 0.573),
];
const PARTICLE_WATER_COLOR: PColor = &[
    Color::rgb(0.0, 0.624, 0.784),
    Color::rgb(0.0, 0.671, 0.843),
    Color::rgb(0.0, 0.71, 0.894),
    Color::rgb(0.122, 0.757, 0.918),
    Color::rgb(0.224, 0.816, 0.969),
];

struct MovementOptionGroup(&'static [IVec2]);

impl MovementOptionGroup {
    fn shuffle(&self) -> Vec<IVec2> {
        let mut rng = rand::thread_rng();
        let mut group = self.0.to_vec();
        group.shuffle(&mut rng);
        group
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

#[derive(Component, Debug, Clone, Copy)]
pub struct ParticleData {
    pub __type: Particle,
    pub position: IVec2,
    pub velocity: Vec2, // Is not IVec so it can accumulate velocity from e.g. gravity
    sleep_count: u32,
    asleep: bool,
}

impl ParticleData {
    pub fn new(__type: Particle, position: IVec2) -> Self {
        Self {
            __type,
            position,
            velocity: Vec2::new(0.0, -1.0),
            sleep_count: 0,
            asleep: false,
        }
    }

    pub fn is_asleep(&self) -> bool {
        self.asleep
    }

    pub fn wake(&mut self) {
        if self.asleep {
            self.velocity = Vec2::new(0.0, -1.0);
        }
        self.sleep_count = 0;
        self.asleep = false;
    }

    pub fn sleep(&mut self) {
        self.sleep_count += 1;
        if self.sleep_count >= TIME_UNTIL_SLEEP {
            self.velocity = Vec2::new(0.0, 0.0);
            self.asleep = true;
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

    pub fn update(&mut self, data: &mut ParticleData, world: &world::World) -> Option<IVec2> {
        // TODO: implement some sort of energy conservation where a particle wants to move in the direction
        //       of its momentum -> so for example a water particle will not randomly change direction
        //       --> Also implement some sort of energy loss so particles will stop moving even if they
        //       are not colliding with anything
        if self.is_empty() {
            return None;
        }

        let mut updated = false;
        let mut position = data.position;

        if let Some(movement) = self.movement() {
            data.velocity += GRAVITY;
            let mut momentum = data.velocity.length();
            while momentum > 1.0 {
                let mut can_break = false;
                for group in movement {
                    let mut __group = group.shuffle();
                    for dir in __group {
                        let desired_position = position + dir;
                        if world.is_empty(desired_position) {
                            momentum -= 1.0;
                            if momentum < 0.0 {
                                return Some(position);
                            }
                            position = desired_position;
                            can_break = true;
                            updated = true;
                            break;
                        }
                    }
                    if can_break {
                        break;
                    }
                }
                if !can_break {
                    break;
                }
            }
        };

        if updated {
            data.wake();
            Some(position)
        } else {
            data.sleep();
            None
        }
    }
}
