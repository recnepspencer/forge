use crate::workload_platform::planar_boolean_events::PlanarBooleanIntervalEventKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanOverlapChainBoundaryRole {
    FullOverlapSpan,
    OverlapStartBoundary,
    OverlapInteriorFragment,
    OverlapEndBoundary,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanOverlapChainPosture {
    PartialOverlap,
    IdenticalParallel,
    IdenticalAntiParallel,
    DifferentParameterization,
}

impl PlanarBooleanOverlapChainPosture {
    pub(crate) fn from_interval_kind(kind: PlanarBooleanIntervalEventKind) -> Self {
        match kind {
            PlanarBooleanIntervalEventKind::IdenticalSameDirection => Self::IdenticalParallel,
            PlanarBooleanIntervalEventKind::IdenticalAntiParallel => Self::IdenticalAntiParallel,
            PlanarBooleanIntervalEventKind::PartialOverlap => Self::PartialOverlap,
            PlanarBooleanIntervalEventKind::ContainmentOverlap => Self::DifferentParameterization,
        }
    }
}
