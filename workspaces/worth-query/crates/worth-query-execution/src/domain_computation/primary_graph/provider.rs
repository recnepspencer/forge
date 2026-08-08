mod aftermath_causality;
mod application_attempt_registration;
mod application_decision_fact;
mod commit_causality;
mod committed_application;
pub(super) mod committed_dispatch_outbox;
mod decision_facts;
// The items inside already declare `pub(in ...primary_graph)`; the module
// declaration is what actually gated them.
pub(in crate::domain_computation::primary_graph) mod dispatch_outbox;
mod graph_participation;
mod idempotency;
mod invariant_execution;
mod mutation_work;
mod provisional_state;
mod resource_support;
mod session_commit;
mod session_lifecycle;

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use super::WorthQueryPrimaryGraphIntegrationHandle;
pub(super) use application_attempt_registration::WorthQueryApplicationAttemptRegistration;
pub(super) use application_decision_fact::WorthQueryPrimaryGraphApplicationDecisionFact;
pub(super) use committed_application::WorthQueryPrimaryGraphCommittedApplication;
pub use committed_dispatch_outbox::{
    WorthQueryCommittedDispatchOutboxObservation, WorthQueryCommittedDispatchOutboxReadDenial,
    WorthQueryCommittedDispatchOutboxReadWork,
};
pub(super) use idempotency::WorthQueryProviderIdempotencyResolution;
pub use mutation_work::{WorthQueryPrimaryMutationWorkEvidence, WorthQueryTouchedRecordIdentity};

pub(super) struct WorthQueryPrimaryGraphProvider {
    pub(super) graph: WorthQueryPrimaryGraphIntegrationHandle,
    resource_support: resource_support::WorthQueryPrimaryGraphResourceSupport,
    commit_serialization: Mutex<()>,
    pub(super) live_delivery: super::live_delivery::WorthQueryLiveDeliverySource,
    pub(super) sessions: Mutex<WorthQueryPrimaryGraphProviderSessions>,
    #[cfg(test)]
    lose_next_commit_response: std::sync::atomic::AtomicBool,
    #[cfg(test)]
    reject_next_session_prepare: std::sync::atomic::AtomicBool,
    #[cfg(test)]
    reject_next_commit_before_transaction: std::sync::atomic::AtomicBool,
    #[cfg(test)]
    fail_next_index_publication: std::sync::atomic::AtomicBool,
    #[cfg(test)]
    skip_next_invariant_owner_execution: std::sync::atomic::AtomicBool,
    #[cfg(test)]
    violate_next_relational_invariant: std::sync::atomic::AtomicBool,
}

pub(in crate::domain_computation) struct WorthQueryApplicationCommitSerialization<'provider> {
    _guard: std::sync::MutexGuard<'provider, ()>,
}

#[derive(Default)]
pub(super) struct WorthQueryPrimaryGraphProviderSessions {
    pub(super) overlays: BTreeMap<String, WorthQueryPrimaryGraphOverlay>,
    pub(super) session_overlays: BTreeMap<String, String>,
    pub(super) application_attempts: BTreeMap<String, WorthQueryPrimaryGraphApplicationAttempt>,
    pub(super) validated_mutations:
        BTreeMap<String, worth_relational::facade::transactions::ValidatedRelationalMutation>,
    pub(super) invariant_work:
        BTreeMap<String, mutation_work::WorthQueryPrimaryMutationWorkCounters>,
    pub(super) completed_commit_evidence: session_commit::WorthQueryCompletedCommitEvidenceStore,
    #[cfg(test)]
    pub(super) observed_completed_mutation_work: Option<WorthQueryPrimaryMutationWorkEvidence>,
    pub(super) next_overlay: u64,
}

/// One application attempt's registration inputs.
///
/// The external-effect contract travels with the attempt so the dispatch
/// record is bound in the same batch as idempotency, in the same transaction.
pub(super) struct WorthQueryPrimaryGraphOverlay {
    pub(super) facts: Vec<crate::domain_computation::WorthQueryProposedFact>,
}

pub(super) struct WorthQueryPrimaryGraphApplicationAttempt {
    pub(super) outcome_identity:
        super::application_attempt::WorthQueryApplicationCommitOutcomeIdentity,
    pub(super) facts: BTreeMap<String, WorthQueryPrimaryGraphApplicationDecisionFact>,
    pub(super) expected_steps: Vec<crate::domain_computation::WorthQueryProvisionalEffectStep>,
    pub(super) batch: worth_relational::facade::transactions::WorkerIntentBatch,
    pub(super) emissions: super::application_attempt::WorthQueryAdmittedApplicationEmissionBatch,
    pub(super) idempotency: super::application_attempt::WorthQueryApplicationIdempotencyBinding,
    pub(super) branch: worth_relational::facade::history::BranchId,
    pub(super) graph_work_session:
        crate::domain_computation::provider_session::WorthQueryGraphWorkSessionIdentity,
    pub(super) decision_fact_count: usize,
    pub(super) preimage_demand: Option<worth_query_installation::facade::InstalledPreImageDemand>,
    pub(super) aftermath_causality: Option<
        crate::domain_computation::application_aftermath::WorthQueryPendingAftermathCausality,
    >,
}

impl WorthQueryPrimaryGraphProvider {
    pub(in crate::domain_computation::primary_graph) fn committed_branch_head(
        &self,
        branch: &worth_relational::facade::history::BranchId,
        expected: worth_relational::facade::history::CommitId,
    ) -> Option<worth_relational::facade::history::CommitReference> {
        self.graph.with_runtime(|runtime| {
            runtime
                .history()
                .branch_head(branch)
                .filter(|head| head.commit_id == expected)
                .cloned()
        })
    }

    pub(super) fn install(
        graph: WorthQueryPrimaryGraphIntegrationHandle,
    ) -> (
        Arc<crate::domain_computation::provider_session::graph_provider::bounded_step::provider_anchor::WorthQueryGraphProviderAnchor>,
        Arc<Self>,
    ){
        let provider = Arc::new(Self {
            graph,
            resource_support: resource_support::WorthQueryPrimaryGraphResourceSupport::install(),
            commit_serialization: Mutex::new(()),
            live_delivery: super::live_delivery::WorthQueryLiveDeliverySource::default(),
            sessions: Mutex::new(WorthQueryPrimaryGraphProviderSessions::default()),
            #[cfg(test)]
            lose_next_commit_response: std::sync::atomic::AtomicBool::new(false),
            #[cfg(test)]
            reject_next_session_prepare: std::sync::atomic::AtomicBool::new(false),
            #[cfg(test)]
            reject_next_commit_before_transaction: std::sync::atomic::AtomicBool::new(false),
            #[cfg(test)]
            fail_next_index_publication: std::sync::atomic::AtomicBool::new(false),
            #[cfg(test)]
            skip_next_invariant_owner_execution: std::sync::atomic::AtomicBool::new(false),
            #[cfg(test)]
            violate_next_relational_invariant: std::sync::atomic::AtomicBool::new(false),
        });
        let anchor = Arc::new(
            crate::domain_computation::provider_session::graph_provider::bounded_step::provider_anchor::WorthQueryGraphProviderAnchor::install_invariant_capable::<
                WorthQueryPrimaryLogicalGraph,
                Arc<Self>,
            >(Arc::clone(&provider)),
        );
        (anchor, provider)
    }

    pub(super) fn application_resource_support(
        &self,
    ) -> worth_query_admission::facade::resource_admission::WorthQueryExecutionResourceSupportSnapshot
    {
        self.resource_support.snapshot()
    }

    pub(super) fn serialize_application_commit(
        &self,
    ) -> WorthQueryApplicationCommitSerialization<'_> {
        WorthQueryApplicationCommitSerialization {
            _guard: self
                .commit_serialization
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner()),
        }
    }

    #[cfg(test)]
    pub(crate) fn lose_next_commit_response(&self) {
        self.lose_next_commit_response
            .store(true, std::sync::atomic::Ordering::Release);
    }

    #[cfg(test)]
    pub(crate) fn reject_next_session_prepare(&self) {
        self.reject_next_session_prepare
            .store(true, std::sync::atomic::Ordering::Release);
    }

    #[cfg(test)]
    pub(crate) fn reject_next_commit_before_transaction(&self) {
        self.reject_next_commit_before_transaction
            .store(true, std::sync::atomic::Ordering::Release);
    }

    #[cfg(test)]
    pub(crate) fn fail_next_index_publication(&self) {
        self.fail_next_index_publication
            .store(true, std::sync::atomic::Ordering::Release);
    }

    #[cfg(test)]
    pub(crate) fn skip_next_invariant_owner_execution(&self) {
        self.skip_next_invariant_owner_execution
            .store(true, std::sync::atomic::Ordering::Release);
    }

    #[cfg(test)]
    pub(crate) fn violate_next_relational_invariant(&self) {
        self.violate_next_relational_invariant
            .store(true, std::sync::atomic::Ordering::Release);
    }

    #[cfg(test)]
    pub(super) fn take_lost_commit_response(&self) -> bool {
        self.lose_next_commit_response
            .swap(false, std::sync::atomic::Ordering::AcqRel)
    }

    #[cfg(test)]
    pub(super) fn take_rejected_session_prepare(&self) -> bool {
        self.reject_next_session_prepare
            .swap(false, std::sync::atomic::Ordering::AcqRel)
    }

    #[cfg(test)]
    pub(super) fn take_rejected_commit_before_transaction(&self) -> bool {
        self.reject_next_commit_before_transaction
            .swap(false, std::sync::atomic::Ordering::AcqRel)
    }

    #[cfg(test)]
    pub(super) fn take_failed_index_publication(&self) -> bool {
        self.fail_next_index_publication
            .swap(false, std::sync::atomic::Ordering::AcqRel)
    }

    #[cfg(test)]
    pub(crate) fn completed_mutation_work(&self) -> Option<WorthQueryPrimaryMutationWorkEvidence> {
        self.sessions
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .observed_completed_mutation_work
            .clone()
    }

    pub(super) fn take_completed_commit_evidence(
        &self,
        commit: worth_relational::facade::history::CommitId,
    ) -> Option<session_commit::WorthQueryPrimaryGraphCommitEvidence> {
        self.sessions
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .completed_commit_evidence
            .take(commit)
    }

    #[cfg(test)]
    pub(super) fn take_skipped_invariant_owner_execution(&self) -> bool {
        self.skip_next_invariant_owner_execution
            .swap(false, std::sync::atomic::Ordering::AcqRel)
    }

    #[cfg(test)]
    pub(super) fn take_relational_invariant_violation(&self) -> bool {
        self.violate_next_relational_invariant
            .swap(false, std::sync::atomic::Ordering::AcqRel)
    }
}

pub(super) struct WorthQueryPrimaryLogicalGraph;
