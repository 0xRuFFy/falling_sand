use bevy::prelude::*;
use bevy::sprite::Anchor;
use rand::prelude::SliceRandom;

const PARTICLE_SIZE: Vec2 = Vec2::new(1.0, 1.0);

type PColor = &'static [Color];
const DEFAULT_COLOR: &Color = &Color::rgb(0.0, 0.0, 0.0);
const SAND_COLOR: PColor = &[
    Color::rgb(0.965, 0.843, 0.69),
    Color::rgb(0.949, 0.824, 0.663),
    Color::rgb(0.925, 0.8, 0.635),
    Color::rgb(0.906, 0.769, 0.588),
    Color::rgb(0.882, 0.749, 0.573),
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

#[derive(Clone, Copy)]
pub enum ParticleType {
    Sand,
}

impl ParticleType {
    pub fn create(&self) -> Particle {
        Particle {
            __type: self.clone(),
        }
    }

    fn color(&self) -> Option<&Color> {
        match self {
            ParticleType::Sand => SAND_COLOR,
        }
        .choose(&mut rand::thread_rng())
    }

    fn movement(&self) -> Option<PMovement> {
        Some(match self {
            ParticleType::Sand => SAND_MOVEMENT,
        })
    }
}

#[derive(Component, Clone, Copy)]
pub struct Particle {
    __type: ParticleType,
}

impl Particle {
    pub fn spawn(&self, commands: &mut Commands) -> Entity {
        commands
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: *self.__type.color().unwrap_or(DEFAULT_COLOR),
                        custom_size: Some(PARTICLE_SIZE),
                        anchor: Anchor::BottomLeft,
                        ..default()
                    },
                    // transform: Transform::from_translation(self.as_vec3_position()),
                    ..default()
                },
                *self,
            ))
            .id()
    }
}
