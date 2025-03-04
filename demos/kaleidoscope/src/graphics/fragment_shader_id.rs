use wrend::Id;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum FragmentShaderId {
    GenerateCircleGradient,
    GenerateLinearGradient,
    GenerateVideoInput,
    FilterUnfiltered,
    FilterSplit,
    FilterTriangleReflection,
    FilterOffsetFragments,
    FilterMovingFragments,
}

impl Id for FragmentShaderId {}

impl Default for FragmentShaderId {
    fn default() -> Self {
        Self::FilterUnfiltered
    }
}
