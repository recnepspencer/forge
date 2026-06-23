#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanCollinearRelationKind {
    Disjoint,
    EndpointTouch,
    PartialOverlap,
    ContainmentOverlap,
    IdenticalSameDirection,
    IdenticalAntiParallel,
}

impl PlanarBooleanCollinearRelationKind {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Disjoint => "disjoint",
            Self::EndpointTouch => "endpoint-touch",
            Self::PartialOverlap => "partial-overlap",
            Self::ContainmentOverlap => "containment-overlap",
            Self::IdenticalSameDirection => "identical-same-direction",
            Self::IdenticalAntiParallel => "identical-anti-parallel",
        }
    }

    pub(crate) fn has_interval_basis(self) -> bool {
        matches!(
            self,
            Self::PartialOverlap
                | Self::ContainmentOverlap
                | Self::IdenticalSameDirection
                | Self::IdenticalAntiParallel
        )
    }
}
