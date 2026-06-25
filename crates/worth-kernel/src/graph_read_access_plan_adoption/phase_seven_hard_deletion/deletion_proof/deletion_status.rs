#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessHardDeletionStatus {
    Deleted,
    CappedResidue,
    TypedQueryGapWithRemovalTrigger,
    Unresolved,
}

impl WorthGraphReadAccessHardDeletionStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Deleted => "deleted",
            Self::CappedResidue => "capped_residue",
            Self::TypedQueryGapWithRemovalTrigger => "typed_query_gap_with_removal_trigger",
            Self::Unresolved => "unresolved",
        }
    }

    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::Unresolved)
    }
}
