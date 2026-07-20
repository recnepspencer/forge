#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WorthUiAllocationReplanDenial {
    ConstraintSetDenied,
    CandidateProjectionUnavailable,
}
