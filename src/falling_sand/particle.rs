use super::world;
use crate::utils::VecTransform;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use rand::prelude::SliceRandom;

const PARTICLE_SIZE: Vec2 = Vec2::new(1.0, 1.0);
const SLEEP_TIMER: u8 = 50;

type PColor = &'static [Color];
const DEFAULT_COLOR: &Color = &Color::rgb(0.0, 0.0, 0.0);
const SAND_COLOR: PColor = &[
    Color::rgb(0.965, 0.843, 0.69),
    Color::rgb(0.949, 0.824, 0.663),
    Color::rgb(0.925, 0.8, 0.635),
    Color::rgb(0.906, 0.769, 0.588),
    Color::rgb(0.882, 0.749, 0.573),
];
const WATER_COLOR: PColor = &[
    Color::rgb(0.0, 0.624, 0.784),
    Color::rgb(0.0, 0.671, 0.843),
    Color::rgb(0.0, 0.71, 0.894),
    Color::rgb(0.122, 0.757, 0.918),
    Color::rgb(0.224, 0.816, 0.969),
];

struct MovementOptionGroup(&'static [IVec2]);

impl MovementOptionGroup {
    fn shuffled(&self) -> Vec<IVec2> {
        let mut shuffled = self.0.to_vec();
        shuffled.shuffle(&mut rand::thread_rng());
        shuffled
    }
}

type PMovement = &'static [MovementOptionGroup];
const SAND_MOVEMENT: PMovement = &[
    MovementOptionGroup(&[IVec2::new(0, -1)]),
    MovementOptionGroup(&[IVec2::new(1, -1), IVec2::new(-1, -1)]),
];
const WATER_MOVEMENT: PMovement = &[
    MovementOptionGroup(&[IVec2::new(0, -1)]),
    MovementOptionGroup(&[IVec2::new(1, 0), IVec2::new(-1, 0)]),
    MovementOptionGroup(&[IVec2::new(1, -1), IVec2::new(-1, -1)]),
];

#[derive(Component)]
pub struct ParticleTag;

#[derive(Debug, Clone, Copy)]
pub enum ParticleType {
    Sand,
    Water,
}

impl ParticleType {
    pub fn create(self, commands: &mut Commands, position: IVec2) -> Particle {
        Particle {
            __type: self,
            __id: commands
                .spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: *self.color().unwrap_or(DEFAULT_COLOR),
                            custom_size: Some(PARTICLE_SIZE),
                            anchor: Anchor::BottomLeft,
                            ..default()
                        },
                        transform: Transform::from_translation(position.as_vec3()),
                        ..default()
                    },
                    ParticleTag,
                ))
                .id(),
            sleep_counter: SLEEP_TIMER,
            momentum: 0.0,
            position,
        }
    }

    fn color(&self) -> Option<&Color> {
        match self {
            ParticleType::Sand => SAND_COLOR,
            ParticleType::Water => WATER_COLOR,
        }
        .choose(&mut rand::thread_rng())
    }

    fn movement(&self) -> Option<PMovement> {
        Some(match self {
            ParticleType::Sand => SAND_MOVEMENT,
            ParticleType::Water => WATER_MOVEMENT,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Particle {
    __type: ParticleType,
    __id: Entity,
    sleep_counter: u8,
    momentum: f32,
    pub position: IVec2,
}

impl Particle {
    pub fn id(&self) -> &Entity {
        &self.__id
    }

    pub fn asleep(&self) -> bool {
        self.sleep_counter == 0
    }

    pub fn sleep(&mut self) {
        self.momentum = 0.0;
        self.sleep_counter = self.sleep_counter.saturating_sub(1);
    }

    pub fn wake(&mut self) {
        self.sleep_counter = SLEEP_TIMER;
    }

    pub fn accelerate(&mut self, amount: f32) {
        self.momentum += amount;
    }

    pub fn movement(&self, world: &world::World) -> Option<IVec2> {
        // println!("{:?}", self.momentum)
        let mut changed = false;
        let mut updated_position = self.position;
        let mut last_dir = IVec2::ZERO;
        if let Some(groups) = self.__type.movement() {
            let mut momentum = self.momentum.abs(); // NOTE: TEMPORARY
            while momentum > 0.0 {
                let mut dead_end = true;
                for group in groups {
                    let mut shuffled = group.shuffled();
                    shuffled.insert(0, last_dir);
                    for offset in shuffled {
                        let new_position = updated_position + offset;
                        if world.empty_at(new_position) {
                            changed = true;
                            dead_end = false;
                            updated_position = new_position;
                            momentum -= 1.0;
                            last_dir = offset;
                            break;
                        }
                    }
                    if !dead_end {
                        break;
                    }
                }
                if dead_end {
                    break;
                }
            }
        }

        if changed {
            Some(updated_position)
        } else {
            None
        }
    }
}
