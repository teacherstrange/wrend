use webgl::renderer::{id::Id, id_name::IdName};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum UniformId {
    UNow,
}

impl Id for UniformId {}

impl Default for UniformId {
    fn default() -> Self {
        Self::UNow
    }
}

impl IdName for UniformId {
    fn name(&self) -> String {
        match self {
            UniformId::UNow => "u_now".to_string(),
        }
    }
}