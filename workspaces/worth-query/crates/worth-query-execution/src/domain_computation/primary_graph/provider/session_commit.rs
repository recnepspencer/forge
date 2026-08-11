//! Named responsibilities for committing one prepared provider session.

mod evidence;
mod outbox_commit;
mod preimage_retention;
mod prepared_session;
mod relational_commit;

pub(in crate::domain_computation::primary_graph) use evidence::WorthQueryCompletedCommitEvidenceStore;
pub(in crate::domain_computation) use outbox_commit::WorthQueryCommittedDispatchOutboxBinding;
pub(in crate::domain_computation::primary_graph) use outbox_commit::{
    WorthQueryCommittedDispatchOutboxBindingDenial, WorthQueryCommittedDispatchOutboxReceiptSeal,
};
pub(crate) use preimage_retention::WorthQueryRetainedPreImageSeal;
pub(in crate::domain_computation::primary_graph::provider) use relational_commit::WorthQueryCommittedApplicationPublicationSeal;
pub(in crate::domain_computation::primary_graph) use relational_commit::{
    WorthQueryMutationWorkCommitSeal, WorthQueryPrimaryGraphCommitEvidence,
};

use super::WorthQueryPrimaryGraphProvider;
use crate::domain_computation::{
    WorthQueryProviderSessionFailure, WorthQueryProviderSessionProtocolStage,
};

pub(super) fn commit_prepared_session(
    provider: &WorthQueryPrimaryGraphProvider,
    affinity: crate::domain_computation::WorthQueryProviderSessionAffinityIdentity,
) -> Result<String, WorthQueryProviderSessionFailure> {
    let prepared = prepared_session::take_prepared_session(provider, affinity)?;
    #[cfg(test)]
    if provider.take_rejected_commit_before_transaction() {
        return Err(provider_failure(
            WorthQueryProviderSessionProtocolStage::Commit,
            "injected rejection before the atomic application transaction",
        ));
    }
    prepared.validate_decision_work()?;
    relational_commit::commit_owner_validated(provider, prepared)
}

pub(super) fn provider_failure(
    stage: WorthQueryProviderSessionProtocolStage,
    detail: &'static str,
) -> WorthQueryProviderSessionFailure {
    WorthQueryProviderSessionFailure::new(
        crate::domain_computation::WorthQueryProviderSessionDenialKind::ProviderRejected,
        stage,
        detail,
        crate::domain_computation::WorthQueryProviderSessionProtocolCounters::default(),
    )
}
