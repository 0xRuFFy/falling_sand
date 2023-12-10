use super::particle::ParticleType;
use bevy::prelude::*;

#[derive(Resource, Deref, DerefMut)]
pub struct CurrentParticleType(pub ParticleType);

impl Default for CurrentParticleType {
    fn default() -> Self {
        Self(ParticleType::Sand)
    }
}
