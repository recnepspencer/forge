#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TouchedGraphParityResidueClassification {
    OrdinaryPathCarried,
    Deleted,
    CappedNonOrdinary,
    QueryGap,
    BlockedOutsideRoadmap,
}

impl TouchedGraphParityResidueClassification {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OrdinaryPathCarried => "ordinary-path-carried",
            Self::Deleted => "deleted",
            Self::CappedNonOrdinary => "capped-non-ordinary",
            Self::QueryGap => "query-gap",
            Self::BlockedOutsideRoadmap => "blocked-outside-roadmap",
        }
    }
}
