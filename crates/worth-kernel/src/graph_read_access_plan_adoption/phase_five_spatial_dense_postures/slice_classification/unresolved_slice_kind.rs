#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessUnresolvedSliceKind {
    SpatialGraphRead,
    DenseFrontierRead,
    BroadBooleanPredicateRead,
    KernelGraphRead,
    CarriedCapabilityGap,
    DeniedOrRequiredQueryPosture,
    MissingQueryReadFamilyArtifact,
    UnknownCoveredGraphRead,
}

impl WorthGraphReadAccessUnresolvedSliceKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SpatialGraphRead => "spatial_graph_read",
            Self::DenseFrontierRead => "dense_frontier_read",
            Self::BroadBooleanPredicateRead => "broad_boolean_predicate_read",
            Self::KernelGraphRead => "kernel_graph_read",
            Self::CarriedCapabilityGap => "carried_capability_gap",
            Self::DeniedOrRequiredQueryPosture => "denied_or_required_query_posture",
            Self::MissingQueryReadFamilyArtifact => "missing_query_read_family_artifact",
            Self::UnknownCoveredGraphRead => "unknown_covered_graph_read",
        }
    }

    pub const fn is_dense_or_broad(self) -> bool {
        matches!(
            self,
            Self::DenseFrontierRead | Self::BroadBooleanPredicateRead
        )
    }
}
