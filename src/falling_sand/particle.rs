use bevy::prelude::*;
use bevy::sprite::Anchor;
use rand::prelude::SliceRandom;

use super::world::CHUNK_SIZE;

const PARTICLE_SIZE: Vec2 = Vec2::new(1.0, 1.0);

type PColor = &'static [Color];
const PARTICLE_DEFAULT_COLOR: &Color = &Color::rgb(0.0, 0.0, 0.0);
const PARTICLE_SAND_COLOR: PColor = &[
    Color::rgb(0.965, 0.843, 0.69),
    Color::rgb(0.949, 0.824, 0.663),
    Color::rgb(0.925, 0.8, 0.635),
    Color::rgb(0.906, 0.769, 0.588),
    Color::rgb(0.882, 0.749, 0.573),
];

#[derive(Clone, Copy)]
pub enum ParticleType {
    Sand,
}

impl ParticleType {
    pub fn create(&self, position: IVec2, velocity: IVec2) -> Particle {
        Particle {
            __type: self.clone(),
            position,
            velocity,
        }
    }

    pub fn color(&self) -> Option<&Color> {
        let mut rng = rand::thread_rng();
        match self {
            ParticleType::Sand => PARTICLE_SAND_COLOR,
        }
        .choose(&mut rng)
    }
}

#[derive(Component, Clone, Copy)]
pub struct Particle {
    __type: ParticleType,
    pub position: IVec2,
    pub velocity: IVec2,
}

impl Particle {
    pub fn spawn(&self, commands: &mut Commands) -> Entity {
        commands
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: *self.__type.color().unwrap_or(PARTICLE_DEFAULT_COLOR),
                        custom_size: Some(PARTICLE_SIZE),
                        anchor: Anchor::BottomLeft,
                        ..default()
                    },
                    transform: Transform::from_translation(self.as_vec3_position()),
                    ..default()
                },
                *self,
            ))
            .id()
    }

    pub fn in_chunk_index(&self) -> usize {
        // let index = self.position.rem_euclid(IVec2::splat(CHUNK_SIZE as i32));
        // index.x as usize * CHUNK_SIZE + index.y as usize
        Self::in_chunk_index_position(self.position)
    }

    pub fn as_vec2_position(&self) -> Vec2 {
        self.position.as_vec2()
    }

    pub fn as_vec2_velocity(&self) -> Vec2 {
        self.velocity.as_vec2()
    }

    pub fn as_vec3_position(&self) -> Vec3 {
        self.as_vec2_position().extend(0.0)
    }

    pub fn as_vec3_velocity(&self) -> Vec3 {
        self.as_vec2_velocity().extend(0.0)
    }

    pub fn in_chunk_index_position(position: IVec2) -> usize {
        let index = position.rem_euclid(IVec2::splat(CHUNK_SIZE as i32));
        index.x as usize * CHUNK_SIZE + index.y as usize
    }
}
