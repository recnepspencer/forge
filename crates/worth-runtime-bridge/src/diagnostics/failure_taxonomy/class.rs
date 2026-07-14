#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BridgeTemporalAsyncFailureClass {
    TemporalBasisFailure,
    TemporalReadinessFailure,
    AsyncIdentityFailure,
    CompletionAdmissionFailure,
    SupersessionFailure,
    RetryRevalidationFailure,
    OrderingFailure,
    ResumeBasisFailure,
    PreviewBoundaryFailure,
    PolicyRemaskFailure,
    WritebackBoundaryFailure,
}

impl BridgeTemporalAsyncFailureClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TemporalBasisFailure => "temporal_basis",
            Self::TemporalReadinessFailure => "temporal_readiness",
            Self::AsyncIdentityFailure => "async_identity",
            Self::CompletionAdmissionFailure => "completion_admission",
            Self::SupersessionFailure => "supersession",
            Self::RetryRevalidationFailure => "retry_revalidation",
            Self::OrderingFailure => "ordering",
            Self::ResumeBasisFailure => "resume_basis",
            Self::PreviewBoundaryFailure => "preview_boundary",
            Self::PolicyRemaskFailure => "policy_remask",
            Self::WritebackBoundaryFailure => "writeback_boundary",
        }
    }
}
