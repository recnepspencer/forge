//! Closed application commit receipt.

mod construction;
mod pending;
mod projection;
mod publication_source;

use worth_query_installation::facade::WorthQueryCanonicalWorkPhases;
use worth_relational::facade::history::{CommitId, CommitReference};

use crate::domain_computation::application_aftermath::{
    WorthQueryCommittedAftermathCausality, WorthQueryDispatchOutboxRecord,
    WorthQueryExternalEffectDispatch, WorthQueryRetainedPreImage,
};

use super::super::{
    WorthQueryApplicationCommitAuthorityBinding, WorthQueryApplicationCommitTerminalEvidence,
    WorthQueryMutationPreconditionComparisonEvidence,
};

pub(in crate::domain_computation::primary_graph) use pending::WorthQueryPendingApplicationCommitReceipt;
pub(in crate::domain_computation::primary_graph) use projection::WorthQueryCommittedReceiptProjection;
pub use publication_source::{
    WorthQueryApplicationCommitPublicationExternalEffect,
    WorthQueryApplicationCommitPublicationSource,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryApplicationCommitReceipt {
    pub(super) authoritative_provider_session:
        crate::domain_computation::provider_session::WorthQueryProviderSessionTerminalBinding,
    pub(super) outcome_identity: Option<super::super::WorthQueryApplicationCommitOutcomeIdentity>,
    pub(super) provider_runtime_instance_id: u64,
    pub(super) commit: CommitReference,
    pub(super) changed_record_count: usize,
    pub(super) emitted_effect_count: usize,
    pub(super) mutation_work:
        Option<super::super::super::provider::WorthQueryPrimaryMutationWorkEvidence>,
    pub(super) precondition_comparison: WorthQueryMutationPreconditionComparisonEvidence,
    pub(super) canonical_work: WorthQueryCanonicalWorkPhases,
    pub(super) terminal: WorthQueryApplicationCommitTerminalEvidence,
    pub(super) committed_dispatch_outbox:
        Option<super::super::super::provider::WorthQueryCommittedDispatchOutboxBinding>,
    pub(super) external_dispatch: Option<WorthQueryExternalEffectDispatch>,
    pub(super) external_dispatch_preparation_denial:
        Option<super::super::WorthQueryExternalDispatchPreparationDenial>,
    /// R8.62 / C1 — derived from admission at construction; never caller-supplied.
    pub(super) authority_binding: WorthQueryApplicationCommitAuthorityBinding,
    /// R8.2 — exact inverse pre-image slice retained from the decision read-set.
    pub(super) retained_preimage: Option<WorthQueryRetainedPreImage>,
    pub(super) aftermath_causality: Option<WorthQueryCommittedAftermathCausality>,
    pub(super) expected_retry_session: Option<
        crate::domain_computation::provider_session::WorthQueryProviderSessionTerminalBinding,
    >,
}

impl Eq for WorthQueryApplicationCommitReceipt {}

impl WorthQueryApplicationCommitReceipt {
    pub const fn outcome_identity(
        &self,
    ) -> Option<super::super::WorthQueryApplicationCommitOutcomeIdentity> {
        self.outcome_identity
    }

    pub const fn provider_runtime_instance_id(&self) -> u64 {
        self.provider_runtime_instance_id
    }

    pub const fn commit_id(&self) -> CommitId {
        self.commit.commit_id
    }

    pub const fn commit_reference(&self) -> &CommitReference {
        &self.commit
    }

    pub const fn changed_record_count(&self) -> usize {
        self.changed_record_count
    }

    pub const fn emitted_effect_count(&self) -> usize {
        self.emitted_effect_count
    }

    pub fn publication_source(&self) -> WorthQueryApplicationCommitPublicationSource {
        WorthQueryApplicationCommitPublicationSource::from_receipt(self)
    }

    pub fn mutation_work(
        &self,
    ) -> Option<&super::super::super::provider::WorthQueryPrimaryMutationWorkEvidence> {
        self.mutation_work.as_ref()
    }

    pub const fn precondition_comparison(
        &self,
    ) -> &WorthQueryMutationPreconditionComparisonEvidence {
        &self.precondition_comparison
    }

    pub const fn canonical_work(&self) -> WorthQueryCanonicalWorkPhases {
        self.canonical_work
    }

    pub const fn terminal(&self) -> &WorthQueryApplicationCommitTerminalEvidence {
        &self.terminal
    }

    pub fn dispatch_outbox(&self) -> Option<&WorthQueryDispatchOutboxRecord> {
        self.committed_dispatch_outbox
            .as_ref()
            .map(super::super::super::provider::WorthQueryCommittedDispatchOutboxBinding::record)
    }

    pub(in crate::domain_computation) fn committed_dispatch_outbox(
        &self,
    ) -> Option<&super::super::super::provider::WorthQueryCommittedDispatchOutboxBinding> {
        self.committed_dispatch_outbox.as_ref()
    }

    pub const fn external_dispatch(&self) -> Option<&WorthQueryExternalEffectDispatch> {
        self.external_dispatch.as_ref()
    }

    pub const fn external_dispatch_preparation_denial(
        &self,
    ) -> Option<super::super::WorthQueryExternalDispatchPreparationDenial> {
        self.external_dispatch_preparation_denial
    }

    pub const fn authority_binding(&self) -> &WorthQueryApplicationCommitAuthorityBinding {
        &self.authority_binding
    }

    pub const fn installed_operation(&self) -> &[u8; 32] {
        self.authority_binding.installed_operation()
    }

    pub(crate) const fn runtime_authority(
        &self,
    ) -> crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity {
        self.authority_binding.runtime_authority()
    }

    pub(crate) const fn installed_aftermath(
        &self,
    ) -> Option<&worth_query_installation::facade::WorthQueryInstalledAftermathContract> {
        self.authority_binding.installed_aftermath()
    }

    pub const fn published_aftermath_posture(
        &self,
    ) -> Option<worth_query_installation::facade::PublishedAftermathPosture> {
        match self.authority_binding.installed_aftermath() {
            Some(aftermath) => Some(aftermath.published_posture()),
            None => None,
        }
    }

    pub const fn principal_scope(
        &self,
    ) -> &crate::domain_computation::authorization::WorthQueryOperationScopeBinding {
        self.authority_binding.principal_scope()
    }

    pub const fn idempotency_binding(
        &self,
    ) -> super::super::WorthQueryApplicationIdempotencyBinding {
        self.authority_binding.idempotency_binding()
    }

    pub const fn retained_preimage(&self) -> Option<&WorthQueryRetainedPreImage> {
        self.retained_preimage.as_ref()
    }

    pub const fn aftermath_causality(&self) -> Option<&WorthQueryCommittedAftermathCausality> {
        self.aftermath_causality.as_ref()
    }

    pub(in crate::domain_computation::primary_graph) fn with_aftermath_causality(
        mut self,
        causality: Option<WorthQueryCommittedAftermathCausality>,
    ) -> Self {
        self.aftermath_causality = causality;
        self
    }

    pub(in crate::domain_computation::primary_graph) fn with_external_dispatch(
        mut self,
        dispatch: WorthQueryExternalEffectDispatch,
    ) -> Self {
        self.canonical_work = self
            .canonical_work
            .with_external_dispatch_work(dispatch.canonical_work());
        self.external_dispatch = Some(dispatch);
        self
    }

    pub(in crate::domain_computation::primary_graph) fn with_external_dispatch_preparation_denial(
        mut self,
        denial: super::super::WorthQueryExternalDispatchPreparationDenial,
    ) -> Self {
        self.external_dispatch_preparation_denial = Some(denial);
        self
    }

    pub fn is_same_authoritative_commit(&self, other: &Self) -> bool {
        self.provider_runtime_instance_id == other.provider_runtime_instance_id
            && self.terminal.branch() == other.terminal.branch()
            && self.commit == other.commit
    }
}
