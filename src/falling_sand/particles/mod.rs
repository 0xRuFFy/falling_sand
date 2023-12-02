mod sand;

use bevy::prelude::*;

pub trait Particle {
    fn spawn(&self, commands: &mut Commands); // TODO: spawn location
}
