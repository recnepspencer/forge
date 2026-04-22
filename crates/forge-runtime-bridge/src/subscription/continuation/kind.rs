#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionContinuationKind {
    Unchanged,
    OneToOneReplace,
    OneToManySplit,
    MergeLikeContinue,
    BranchLocalContinue,
    RejectedUnsupported,
    RejectedAmbiguous,
    RejectedAuthorityDenied,
    RejectedBranchLeak,
}

impl BridgeSubscriptionContinuationKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unchanged => "unchanged",
            Self::OneToOneReplace => "one_to_one_replace",
            Self::OneToManySplit => "one_to_many_split",
            Self::MergeLikeContinue => "merge_like_continue",
            Self::BranchLocalContinue => "branch_local_continue",
            Self::RejectedUnsupported => "rejected_unsupported",
            Self::RejectedAmbiguous => "rejected_ambiguous",
            Self::RejectedAuthorityDenied => "rejected_authority_denied",
            Self::RejectedBranchLeak => "rejected_branch_leak",
        }
    }

    pub(super) const fn is_rejected(self) -> bool {
        matches!(
            self,
            Self::RejectedUnsupported
                | Self::RejectedAmbiguous
                | Self::RejectedAuthorityDenied
                | Self::RejectedBranchLeak
        )
    }
}
