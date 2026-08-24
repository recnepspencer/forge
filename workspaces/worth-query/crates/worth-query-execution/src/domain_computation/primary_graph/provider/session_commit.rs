//! Named responsibilities for committing one prepared provider session.

mod evidence;
mod outbox_commit;
mod receipt_basis_retention;

pub(crate) use super::application_attempt_state::WorthQueryRetainedPreImageSeal;
pub(in crate::domain_computation::primary_graph) use super::application_attempt_state::{
    WorthQueryMutationWorkCommitSeal, WorthQueryPreImageRetentionWork,
};
pub(in crate::domain_computation::primary_graph) use evidence::WorthQueryCompletedCommitEvidenceStore;
pub(in crate::domain_computation) use outbox_commit::WorthQueryCommittedDispatchOutboxBinding;
pub(in crate::domain_computation::primary_graph) use outbox_commit::{
    WorthQueryCommittedDispatchOutboxBindingDenial, WorthQueryCommittedDispatchOutboxReceiptSeal,
    WorthQueryCommittedDispatchOutboxResolution,
};
pub(in crate::domain_computation::primary_graph) use receipt_basis_retention::{
    WorthQueryReceiptBasisRetentionStore, WorthQueryRetainedApplicationCommitBasis,
};

use super::WorthQueryPrimaryGraphProvider;
use crate::domain_computation::{
    WorthQueryProviderSessionFailure, WorthQueryProviderSessionProtocolStage,
};

pub(super) fn commit_prepared_session(
    provider: &WorthQueryPrimaryGraphProvider,
    session: crate::domain_computation::WorthQueryProviderSessionView<'_>,
) -> Result<
    crate::domain_computation::WorthQueryProviderTerminalDescription,
    WorthQueryProviderSessionFailure,
> {
    super::application_attempt_state::commit_prepared_application(provider, session)
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
