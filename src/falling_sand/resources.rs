use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct SpawnTimer {
    timer: f32,
    guard: bool,
}

impl SpawnTimer {
    const SPAWN_DELAY: f32 = 0.08;

    pub fn new(&self) -> Self {
        Self::default()
    }

    pub fn tick(&mut self, time: Res<Time>) {
        self.timer += time.delta_seconds();
        if self.timer >= Self::SPAWN_DELAY {
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
