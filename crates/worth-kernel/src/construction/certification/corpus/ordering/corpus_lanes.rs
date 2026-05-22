#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) enum PrimitiveConstructionCorpusAuthoringOrderLane {
    Canonical,
    Reversed,
    RejectedFirst,
    RoleClustered,
}

impl PrimitiveConstructionCorpusAuthoringOrderLane {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Canonical => "canonical",
            Self::Reversed => "reversed",
            Self::RejectedFirst => "rejected_first",
            Self::RoleClustered => "role_clustered",
        }
    }

    pub(crate) fn all() -> [Self; 4] {
        [
            Self::Canonical,
            Self::Reversed,
            Self::RejectedFirst,
            Self::RoleClustered,
        ]
    }
}
