use crate::runtime::WorthUiActivationGateDenialReason;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPlanSwapDenialReason {
    ActivationGateDenied(WorthUiActivationGateDenialReason),
    CandidateExecutionPlanDigestMismatch,
    #[cfg(test)]
    InjectedFailureAfterArtifactMutation,
    #[cfg(test)]
    InjectedFailureBeforeCommit,
}
