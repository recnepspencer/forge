//! Committed application receipt and pending completion (R8.62 / C1).

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

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryApplicationCommitReceipt {
    outcome_identity: Option<super::super::WorthQueryApplicationCommitOutcomeIdentity>,
    provider_runtime_instance_id: u64,
    commit: CommitReference,
    changed_record_count: usize,
    emitted_effect_count: usize,
    mutation_work: Option<super::super::super::provider::WorthQueryPrimaryMutationWorkEvidence>,
    precondition_comparison: WorthQueryMutationPreconditionComparisonEvidence,
    canonical_work: WorthQueryCanonicalWorkPhases,
    terminal: WorthQueryApplicationCommitTerminalEvidence,
    dispatch_outbox: Option<WorthQueryDispatchOutboxRecord>,
    external_dispatch: Option<WorthQueryExternalEffectDispatch>,
    /// R8.62 / C1 — derived from admission at construction; never caller-supplied.
    authority_binding: WorthQueryApplicationCommitAuthorityBinding,
    /// R8.2 — exact inverse pre-image slice retained from the decision read-set.
    retained_preimage: Option<WorthQueryRetainedPreImage>,
    aftermath_causality: Option<WorthQueryCommittedAftermathCausality>,
}

impl Eq for WorthQueryApplicationCommitReceipt {}

pub(in crate::domain_computation::primary_graph) struct WorthQueryPendingApplicationCommitReceipt {
    provider: super::super::super::provider::WorthQueryPrimaryGraphCommittedApplication,
    precondition_comparison: WorthQueryMutationPreconditionComparisonEvidence,
    canonical_work: WorthQueryCanonicalWorkPhases,
    dispatch_outbox: Option<WorthQueryDispatchOutboxRecord>,
    authority_binding: WorthQueryApplicationCommitAuthorityBinding,
    retained_preimage: Option<WorthQueryRetainedPreImage>,
    aftermath_causality: Option<WorthQueryCommittedAftermathCausality>,
}

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

    /// The dispatch record this commit durably co-committed, if the operation
    /// declared an external effect. `None` is the honest ordinary case.
    pub const fn dispatch_outbox(&self) -> Option<&WorthQueryDispatchOutboxRecord> {
        self.dispatch_outbox.as_ref()
    }

    /// What production dispatch observed for the co-committed record.
    pub const fn external_dispatch(&self) -> Option<&WorthQueryExternalEffectDispatch> {
        self.external_dispatch.as_ref()
    }

    /// Admitted operation, principal scope, and idempotency binding (R8.62).
    pub const fn authority_binding(&self) -> &WorthQueryApplicationCommitAuthorityBinding {
        &self.authority_binding
    }

    /// Installed operation identity retained from admission.
    pub const fn installed_operation(&self) -> &[u8; 32] {
        self.authority_binding.installed_operation()
    }

    /// Query runtime whose admission produced this commit (R8.28).
    pub(crate) const fn runtime_authority(
        &self,
    ) -> crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity {
        self.authority_binding.runtime_authority()
    }

    /// Exact compiled aftermath retained from the admitted installed operation.
    pub(crate) const fn installed_aftermath(
        &self,
    ) -> Option<&worth_query_installation::facade::WorthQueryInstalledAftermathContract> {
        self.authority_binding.installed_aftermath()
    }

    /// Closed descriptive posture carried by the exact admitted installed
    /// operation. This is publication input, never execution authority.
    pub const fn published_aftermath_posture(
        &self,
    ) -> Option<worth_query_installation::facade::PublishedAftermathPosture> {
        match self.authority_binding.installed_aftermath() {
            Some(aftermath) => Some(aftermath.published_posture()),
            None => None,
        }
    }

    /// Admitted principal scope retained from admission.
    pub const fn principal_scope(
        &self,
    ) -> &crate::domain_computation::authorization::WorthQueryOperationScopeBinding {
        self.authority_binding.principal_scope()
    }

    /// Idempotency binding retained from the admitted attempt.
    pub const fn idempotency_binding(
        &self,
    ) -> super::super::WorthQueryApplicationIdempotencyBinding {
        self.authority_binding.idempotency_binding()
    }

    /// Exact pre-image slice retained for recorded-inverse undo (R8.2).
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

    /// Attaches one completed dispatch attempt and its canonical cost.
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

    pub fn is_same_authoritative_commit(&self, other: &Self) -> bool {
        self.provider_runtime_instance_id == other.provider_runtime_instance_id
            && self.terminal.branch() == other.terminal.branch()
            && self.commit == other.commit
    }

    pub(in crate::domain_computation::primary_graph) fn from_recovered_provider(
        provider: super::super::super::provider::WorthQueryPrimaryGraphCommittedApplication,
        precondition_comparison: WorthQueryMutationPreconditionComparisonEvidence,
        canonical_work: WorthQueryCanonicalWorkPhases,
        authority_binding: WorthQueryApplicationCommitAuthorityBinding,
    ) -> Self {
        let terminal =
            WorthQueryApplicationCommitTerminalEvidence::recovered(provider.branch().clone());
        Self {
            outcome_identity: provider.application_outcome_identity(),
            provider_runtime_instance_id: provider.runtime_instance_id(),
            commit: provider.commit_reference().clone(),
            changed_record_count: provider.changed_record_count(),
            emitted_effect_count: provider.emitted_effect_count(),
            mutation_work: provider.mutation_work().cloned(),
            precondition_comparison,
            canonical_work,
            terminal,
            dispatch_outbox: None,
            external_dispatch: None,
            authority_binding,
            retained_preimage: None,
            aftermath_causality: None,
        }
    }

    pub(in crate::domain_computation::primary_graph) fn with_retry_inspection(
        mut self,
        completion: crate::domain_computation::provider_session::WorthQueryMutationGraphWorkCompletion,
    ) -> Option<Self> {
        self.terminal = self.terminal.with_retry_inspection(completion)?;
        Some(self)
    }
}

impl WorthQueryPendingApplicationCommitReceipt {
    pub(in crate::domain_computation::primary_graph) const fn from_provider(
        provider: super::super::super::provider::WorthQueryPrimaryGraphCommittedApplication,
        precondition_comparison: WorthQueryMutationPreconditionComparisonEvidence,
        canonical_work: WorthQueryCanonicalWorkPhases,
        authority_binding: WorthQueryApplicationCommitAuthorityBinding,
    ) -> Self {
        Self {
            provider,
            precondition_comparison,
            canonical_work,
            dispatch_outbox: None,
            authority_binding,
            retained_preimage: None,
            aftermath_causality: None,
        }
    }

    pub(in crate::domain_computation::primary_graph) fn with_dispatch_outbox(
        mut self,
        dispatch_outbox: Option<WorthQueryDispatchOutboxRecord>,
    ) -> Self {
        self.dispatch_outbox = dispatch_outbox;
        self
    }

    pub(in crate::domain_computation::primary_graph) fn with_retained_preimage(
        mut self,
        retained_preimage: Option<WorthQueryRetainedPreImage>,
    ) -> Self {
        self.retained_preimage = retained_preimage;
        self
    }

    pub(in crate::domain_computation::primary_graph) fn with_aftermath_causality(
        mut self,
        causality: Option<WorthQueryCommittedAftermathCausality>,
    ) -> Self {
        self.aftermath_causality = causality;
        self
    }

    pub(in crate::domain_computation::primary_graph) fn complete(
        self,
        completion: crate::domain_computation::provider_session::WorthQueryMutationGraphWorkCompletion,
    ) -> Option<WorthQueryApplicationCommitReceipt> {
        if self.provider.branch() != completion.relational_branch() {
            return None;
        }
        let mutation_work = self.provider.mutation_work()?.clone();
        Some(WorthQueryApplicationCommitReceipt {
            outcome_identity: self.provider.application_outcome_identity(),
            provider_runtime_instance_id: self.provider.runtime_instance_id(),
            commit: self.provider.commit_reference().clone(),
            changed_record_count: self.provider.changed_record_count(),
            emitted_effect_count: self.provider.emitted_effect_count(),
            mutation_work: Some(mutation_work),
            precondition_comparison: self.precondition_comparison,
            canonical_work: self.canonical_work,
            terminal: WorthQueryApplicationCommitTerminalEvidence::executed(completion),
            dispatch_outbox: self.dispatch_outbox,
            external_dispatch: None,
            authority_binding: self.authority_binding,
            retained_preimage: self.retained_preimage,
            aftermath_causality: self.aftermath_causality,
        })
    }
}
