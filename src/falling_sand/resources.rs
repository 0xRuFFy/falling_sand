use super::particle::ParticleType;
use bevy::prelude::*;

#[derive(Resource, Deref, DerefMut)]
pub struct CurrentParticleType(pub ParticleType);

impl Default for CurrentParticleType {
    fn default() -> Self {
        Self(ParticleType::Sand)
    }
}

type PBrush = &'static [IVec2];

const BRUSH_DOT: PBrush = &[IVec2::new(0, 0)];
const BRUSH_MEDIUM: PBrush = &[
    IVec2::new(0, 1),
    IVec2::new(0, -1),
    IVec2::new(0, 3),
    IVec2::new(0, -3),
    IVec2::new(1, 1),
    IVec2::new(1, -1),
    IVec2::new(1, 3),
    IVec2::new(1, -3),
    IVec2::new(-1, 1),
    IVec2::new(-1, -1),
    IVec2::new(-1, 3),
    IVec2::new(-1, -3),
    IVec2::new(2, 0),
    IVec2::new(2, 2),
    IVec2::new(2, -2),
    IVec2::new(-2, 0),
    IVec2::new(-2, 2),
    IVec2::new(-2, -2),
    IVec2::new(3, 0),
    IVec2::new(3, 2),
    IVec2::new(3, -2),
    IVec2::new(-3, 0),
    IVec2::new(-3, 2),
    IVec2::new(-3, -2),
    IVec2::new(4, 1),
    IVec2::new(4, -1),
    IVec2::new(-4, 1),
    IVec2::new(-4, -1),
];

#[derive(Resource)]
pub struct ParticleBrush {
    __id: usize,
    brushes: Vec<PBrush>,
}

impl ParticleBrush {
    // pub fn use_medium(&mut self) {
    //     self.0 = BRUSH_MEDIUM;
    // }

    pub fn get(&self) -> PBrush {
        self.brushes[self.__id]
    }

    pub fn increase(&mut self) {
        self.__id = (self.__id + 1).min(self.brushes.len() - 1);
    }

    pub fn decrease(&mut self) {
        self.__id = (self.__id - 1).max(0);
    }
}

impl Default for ParticleBrush {
    fn default() -> Self {
        Self {
            __id: 0,
            brushes: vec![BRUSH_DOT, BRUSH_MEDIUM],
        }
    }
}

#[derive(Resource)]
pub struct SpawnTimer {
    timer: f32,
    guard: bool,
    delay: f32,
}

impl SpawnTimer {
    pub fn new(delay: f32) -> Self {
        Self {
            timer: 0.0,
            guard: false,
            delay,
        }
    }

    pub fn tick(&mut self, time: Res<Time>) {
        self.timer += time.delta_seconds();
        if self.timer >= self.delay {
            self.guard = true;
            self.timer = 0.0;
        }
    }

    pub fn guard(&mut self) -> bool {
        if self.guard {
            self.timer = 0.0;
            self.guard = false;
            return true;
        }
        false
    }
}
