use bevy::prelude::*;

pub type BrushT = &'static [Vec2];

const BRUSH_DOT: BrushT = &[Vec2::new(0.0, 0.0)];
const BRUSH_SMALL: BrushT = &[
    Vec2::new(0.0, 0.0),
    Vec2::new(1.0, 0.0),
    Vec2::new(-1.0, 0.0),
    Vec2::new(0.0, 1.0),
    Vec2::new(0.0, -1.0),
];
const BRUSH_MEDIUM: BrushT = &[
    Vec2::new(0.0, 0.0),
    Vec2::new(1.0, 0.0),
    Vec2::new(1.0, 1.0),
    Vec2::new(1.0, -1.0),
    Vec2::new(2.0, 0.0),
    Vec2::new(-1.0, 0.0),
    Vec2::new(-1.0, 1.0),
    Vec2::new(-1.0, -1.0),
    Vec2::new(-2.0, 0.0),
    Vec2::new(0.0, 1.0),
    Vec2::new(0.0, -1.0),
    Vec2::new(0.0, 2.0),
    Vec2::new(0.0, -2.0),
];
const BRUSH_BIG: BrushT = &[
    Vec2::new(0.0, 0.0),
    Vec2::new(1.0, 0.0),
    Vec2::new(1.0, 1.0),
    Vec2::new(1.0, -1.0),
    Vec2::new(1.0, 2.0),
    Vec2::new(1.0, -2.0),
    Vec2::new(2.0, 0.0),
    Vec2::new(2.0, 1.0),
    Vec2::new(2.0, -1.0),
    Vec2::new(3.0, 0.0),
    Vec2::new(-1.0, 0.0),
    Vec2::new(-1.0, 1.0),
    Vec2::new(-1.0, -1.0),
    Vec2::new(-1.0, 2.0),
    Vec2::new(-1.0, -2.0),
    Vec2::new(-2.0, 0.0),
    Vec2::new(-2.0, 1.0),
    Vec2::new(-2.0, -1.0),
    Vec2::new(-3.0, 0.0),
    Vec2::new(0.0, 1.0),
    Vec2::new(0.0, -1.0),
    Vec2::new(0.0, 2.0),
    Vec2::new(0.0, -2.0),
];
/*
 *     + + +
 *   + + + + +
 * + + + + + + +
 * + + + + + + +
 * + + + + + + +
 *   + + + + +
 *     + + +
 */
const BRUSH_LARGE: BrushT = &[
    Vec2::new(0.0, 0.0),
    Vec2::new(0.0, 1.0),
    Vec2::new(0.0, 2.0),
    Vec2::new(0.0, 3.0),
    Vec2::new(0.0, -3.0),
    Vec2::new(0.0, -2.0),
    Vec2::new(0.0, -1.0),
    Vec2::new(1.0, 0.0),
    Vec2::new(1.0, 1.0),
    Vec2::new(1.0, 2.0),
    Vec2::new(1.0, 3.0),
    Vec2::new(1.0, -3.0),
    Vec2::new(1.0, -2.0),
    Vec2::new(1.0, -1.0),
    Vec2::new(2.0, 0.0),
    Vec2::new(2.0, 1.0),
    Vec2::new(2.0, 2.0),
    Vec2::new(2.0, -2.0),
    Vec2::new(2.0, -1.0),
    Vec2::new(3.0, 0.0),
    Vec2::new(3.0, 1.0),
    Vec2::new(3.0, -1.0),
    Vec2::new(-3.0, 0.0),
    Vec2::new(-3.0, 1.0),
    Vec2::new(-3.0, -1.0),
    Vec2::new(-2.0, 0.0),
    Vec2::new(-2.0, 1.0),
    Vec2::new(-2.0, 2.0),
    Vec2::new(-2.0, -2.0),
    Vec2::new(-2.0, -1.0),
    Vec2::new(-1.0, 0.0),
    Vec2::new(-1.0, 1.0),
    Vec2::new(-1.0, 2.0),
    Vec2::new(-1.0, 3.0),
    Vec2::new(-1.0, -3.0),
    Vec2::new(-1.0, -2.0),
    Vec2::new(-1.0, -1.0),
];

#[derive(Resource)]
pub struct Brush(BrushT);

impl Brush {
    pub fn use_dot(&mut self) {
        self.0 = BRUSH_DOT;
    }

    pub fn use_small(&mut self) {
        self.0 = BRUSH_SMALL;
    }

    pub fn use_medium(&mut self) {
        self.0 = BRUSH_MEDIUM;
    }

    pub fn use_big(&mut self) {
        self.0 = BRUSH_BIG;
    }

    pub fn use_large(&mut self) {
        self.0 = BRUSH_LARGE;
    }

    pub fn get(&self) -> BrushT {
        self.0
    }
}

impl Default for Brush {
    fn default() -> Self {
        let mut brush = Brush(&[]);
        // brush.use_dot();
        // brush.use_small();
        // brush.use_medium();
        // brush.use_big();
        brush.use_large();
        brush
    }
}

const SPAWN_DELAY: f32 = 0.08;

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
            // ..default()
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

impl Default for SpawnTimer {
    fn default() -> Self {
        Self {
            timer: 0.0,
            guard: false,
            delay: SPAWN_DELAY,
        }
    }
}
