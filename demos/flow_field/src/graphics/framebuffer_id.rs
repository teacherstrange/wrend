use wrend::Id;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FramebufferId {
    PerlinNoise,
}

impl Default for FramebufferId {
    fn default() -> Self {
        Self::PerlinNoise
    }
}

impl Id for FramebufferId {}
