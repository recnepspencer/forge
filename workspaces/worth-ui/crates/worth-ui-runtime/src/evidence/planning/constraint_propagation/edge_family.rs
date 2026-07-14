#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiConstraintPropagationEdgeFamily {
    ParentAvailableSpace,
    ChildIntrinsicContribution,
    SiblingNegotiation,
    EqualShareDistribution,
    BoundedReconciliation,
    ViewportInput,
    ScrollViewportInput,
    PortalAnchorInput,
    DurableResizeInput,
}

impl UiConstraintPropagationEdgeFamily {
    pub(crate) const fn rank(self) -> u8 {
        match self {
            Self::ParentAvailableSpace => 0,
            Self::ChildIntrinsicContribution => 1,
            Self::SiblingNegotiation => 2,
            Self::EqualShareDistribution => 3,
            Self::BoundedReconciliation => 4,
            Self::ViewportInput => 5,
            Self::ScrollViewportInput => 6,
            Self::PortalAnchorInput => 7,
            Self::DurableResizeInput => 8,
        }
    }
}
