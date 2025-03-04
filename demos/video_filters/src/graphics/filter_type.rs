use std::fmt::Display;

use strum::EnumIter;

use super::{FragmentShaderId, ProgramId};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, EnumIter)]
pub enum FilterType {
    #[default]
    Unfiltered,
    Grayscale,
    Invert,
    Wavy,
    GaussianBlur,
}

impl FilterType {
    /// Gets the associated ProgramId for the FilterType
    pub fn program_id(&self) -> ProgramId {
        match self {
            FilterType::Unfiltered => ProgramId::Unfiltered,
            FilterType::Grayscale => ProgramId::Grayscale,
            FilterType::Invert => ProgramId::Invert,
            FilterType::Wavy => ProgramId::Wavy,
            FilterType::GaussianBlur => ProgramId::GaussianBlur,
        }
    }

    /// Gets the associated FragmentShaderId for the FilterType
    pub fn fragment_shader_id(&self) -> FragmentShaderId {
        match self {
            FilterType::Unfiltered => FragmentShaderId::Unfiltered,
            FilterType::Grayscale => FragmentShaderId::Grayscale,
            FilterType::Invert => FragmentShaderId::Invert,
            FilterType::Wavy => FragmentShaderId::Wavy,
            FilterType::GaussianBlur => FragmentShaderId::GaussianBlur,
        }
    }
}

impl Display for FilterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FilterType::Unfiltered => write!(f, "Unfiltered"),
            FilterType::Grayscale => write!(f, "Grayscale"),
            FilterType::Invert => write!(f, "Invert"),
            FilterType::Wavy => write!(f, "Wavy"),
            FilterType::GaussianBlur => write!(f, "Gaussian Blur"),
        }
    }
}
