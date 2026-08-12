use super::super::{
    WorthQueryApplicationCommitDenial, WorthQueryApplicationCommitDenialStage,
    WorthQueryApplicationCommitOutcome, WorthQueryApplicationCommitReceipt,
    WorthQueryApplicationStaleAttempt, WorthQueryPendingApplicationCommitReceipt,
};
use crate::domain_computation::provider_session::WorthQueryMutationGraphWorkCompletion;

pub(in crate::domain_computation::primary_graph::application_attempt) enum WorthQueryProviderProgressionOutcome
{
    Committed(WorthQueryPendingApplicationCommitReceipt),
    AlreadyCommitted(WorthQueryApplicationCommitReceipt),
    Stale(WorthQueryApplicationStaleAttempt),
    Cancelled,
    Denied(WorthQueryApplicationCommitDenial),
    Aborted,
    Indeterminate(super::super::WorthQueryApplicationUnresolvedCommitEvidence),
}

impl WorthQueryProviderProgressionOutcome {
    pub(super) fn finish(
        self,
        completion: WorthQueryMutationGraphWorkCompletion,
    ) -> Option<WorthQueryApplicationCommitOutcome> {
        Some(match self {
            Self::Committed(receipt) => {
                WorthQueryApplicationCommitOutcome::Committed(receipt.complete(completion)?)
            }
            Self::AlreadyCommitted(receipt) => {
                WorthQueryApplicationCommitOutcome::AlreadyCommitted(
                    receipt.with_retry_cleanup(completion)?,
                )
            }
            Self::Stale(stale) => WorthQueryApplicationCommitOutcome::Stale(stale),
            Self::Cancelled => WorthQueryApplicationCommitOutcome::Cancelled,
            Self::Denied(denial) => WorthQueryApplicationCommitOutcome::Denied(denial),
            Self::Aborted => WorthQueryApplicationCommitOutcome::Aborted,
            Self::Indeterminate(evidence) => {
                WorthQueryApplicationCommitOutcome::Indeterminate(evidence)
            }
        })
    }
}

pub(in crate::domain_computation::primary_graph::application_attempt) fn progression_denied(
    stage: WorthQueryApplicationCommitDenialStage,
) -> WorthQueryProviderProgressionOutcome {
    WorthQueryProviderProgressionOutcome::Denied(
        WorthQueryApplicationCommitDenial::provider_rejected(stage),
    )
}
