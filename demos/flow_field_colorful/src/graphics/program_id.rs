use wrend::Id;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ProgramId {
    PerlinNoise,
    PassThrough,
    UpdateParticles,
    DrawParticles,
}

impl Id for ProgramId {}

impl Default for ProgramId {
    fn default() -> Self {
        Self::PassThrough
    }
}
