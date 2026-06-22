#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PlanarBooleanIntervalEventKind {
    PartialOverlap,
    ContainmentOverlap,
    IdenticalSameDirection,
    IdenticalAntiParallel,
}

impl PlanarBooleanIntervalEventKind {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::PartialOverlap => "partial-overlap",
            Self::ContainmentOverlap => "containment-overlap",
            Self::IdenticalSameDirection => "identical-same-direction",
            Self::IdenticalAntiParallel => "identical-anti-parallel",
        }
    }
}
