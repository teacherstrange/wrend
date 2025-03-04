use wrend::{Id, IdName};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum AttributeId {
    AQuadVertex,
    AParticlePosition,
}

impl Id for AttributeId {}

impl IdName for AttributeId {
    fn name(&self) -> String {
        match self {
            AttributeId::AQuadVertex => String::from("a_quad_vertex"),
            AttributeId::AParticlePosition => String::from("a_particle_position"),
        }
    }
}

impl Default for AttributeId {
    fn default() -> Self {
        AttributeId::AQuadVertex
    }
}
