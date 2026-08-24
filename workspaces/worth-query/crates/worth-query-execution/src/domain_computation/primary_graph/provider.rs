mod aftermath_causality;
mod application_attempt_state;
mod application_attempt_work;
mod application_decision_fact;
mod application_touch_admission;
mod commit_causality;
pub(super) mod committed_dispatch_outbox;
mod conditional_commit_journal;
mod decision_facts;
pub(in crate::domain_computation::primary_graph) mod fault_port;
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

use std::sync::{Arc, Mutex};

pub(super) use super::application_attempt::WorthQueryPrimaryGraphApplicationAttempt;
use super::WorthQueryPrimaryGraphIntegrationHandle;
pub(super) use application_attempt_state::WorthQueryPrimaryGraphCommittedApplication;
#[cfg(test)]
pub(crate) use application_attempt_work::WorthQueryApplicationAttemptWorkSnapshot;
pub(in crate::domain_computation) use application_decision_fact::WorthQueryPrimaryGraphApplicationDecisionFact;
#[cfg(test)]
pub(in crate::domain_computation::primary_graph) use committed_dispatch_outbox::commit_and_observe_fixture;
#[cfg(test)]
pub(in crate::domain_computation) use committed_dispatch_outbox::{
    commit_distinct_records_and_admit_fixture, commit_observe_and_admit_fixture,
    commit_observe_and_admit_twice_fixture,
};
pub use committed_dispatch_outbox::{
    WorthQueryCommittedDispatchOutboxObservation, WorthQueryCommittedDispatchOutboxReadDenial,
    WorthQueryCommittedDispatchOutboxReadWork,
};
pub(super) use idempotency::WorthQueryProviderIdempotencyResolution;
pub use mutation_work::{WorthQueryPrimaryMutationWorkEvidence, WorthQueryTouchedRecordIdentity};
pub(in crate::domain_computation) use session_commit::WorthQueryCommittedDispatchOutboxBinding;
pub(crate) use session_commit::WorthQueryRetainedPreImageSeal;
pub(super) use session_commit::{
    WorthQueryCommittedDispatchOutboxBindingDenial, WorthQueryCommittedDispatchOutboxReceiptSeal,
};

pub(super) struct WorthQueryPrimaryGraphProvider {
    pub(super) graph: WorthQueryPrimaryGraphIntegrationHandle,
    resource_support: resource_support::WorthQueryPrimaryGraphResourceSupport,
    commit_serialization: Mutex<()>,
    pub(super) live_delivery: super::live_delivery::WorthQueryLiveDeliverySource,
    attempts: Mutex<application_attempt_state::WorthQueryPrimaryGraphApplicationAttemptStore>,
    application_attempt_work: application_attempt_work::WorthQueryApplicationAttemptWorkLedger,
    completed_commit_evidence: Mutex<session_commit::WorthQueryCompletedCommitEvidenceStore>,
    conditional_commit_journal:
        Mutex<conditional_commit_journal::WorthQueryConditionalCommitJournal>,
    conditional_maintenance_failure: Mutex<Option<String>>,
    fault_port: Arc<dyn fault_port::WorthQueryPrimaryGraphFaultPort>,
}

pub(in crate::domain_computation) struct WorthQueryApplicationCommitSerialization<'provider> {
    _guard: std::sync::MutexGuard<'provider, ()>,
}

impl WorthQueryPrimaryGraphProvider {
    pub(super) fn install(
        graph: WorthQueryPrimaryGraphIntegrationHandle,
        fault_port: Arc<dyn fault_port::WorthQueryPrimaryGraphFaultPort>,
    ) -> (
        Arc<crate::domain_computation::provider_session::graph_provider::bounded_step::provider_anchor::WorthQueryGraphProviderAnchor>,
        Arc<Self>,
    ){
        let provider = Arc::new(Self {
            graph,
            resource_support: resource_support::WorthQueryPrimaryGraphResourceSupport::install(),
            commit_serialization: Mutex::new(()),
            live_delivery: super::live_delivery::WorthQueryLiveDeliverySource::default(),
            attempts: Mutex::new(
                application_attempt_state::WorthQueryPrimaryGraphApplicationAttemptStore::default(),
            ),
            application_attempt_work: Default::default(),
            completed_commit_evidence: Mutex::new(
                session_commit::WorthQueryCompletedCommitEvidenceStore::default(),
            ),
            conditional_commit_journal: Mutex::new(Default::default()),
            conditional_maintenance_failure: Mutex::new(None),
            fault_port,
        });
        let anchor = Arc::new(
            crate::domain_computation::provider_session::graph_provider::bounded_step::provider_anchor::WorthQueryGraphProviderAnchor::install_invariant_capable::<
                WorthQueryPrimaryLogicalGraph,
                Arc<Self>,
            >(Arc::clone(&provider)),
        );
        (anchor, provider)
    }

    pub(in crate::domain_computation::primary_graph) fn conditional_commit_sequence(&self) -> u64 {
        self.conditional_commit_journal
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .latest_sequence()
    }

    pub(in crate::domain_computation::primary_graph) fn replace_conditional_commit_routes(
        &self,
        records: impl IntoIterator<Item = worth_relational::facade::transactions::RecordRef>,
        include_whole_graph: bool,
    ) {
        self.conditional_commit_journal
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .replace_routes(records, include_whole_graph);
    }

    pub(in crate::domain_computation::primary_graph) fn record_conditional_commit(
        &self,
        commit: &worth_relational::facade::history::CommitReference,
        records: impl IntoIterator<Item = worth_relational::facade::transactions::RecordRef>,
    ) -> std::collections::BTreeSet<worth_relational::facade::identity::KindId> {
        let records = records.into_iter().collect::<Vec<_>>();
        let entity_kinds = self.graph.with_runtime(|runtime| {
            let previous_version = worth_relational::facade::identity::VersionId(
                commit.version_id.0.saturating_sub(1),
            );
            records
                .iter()
                .filter_map(|record| {
                    let worth_relational::facade::transactions::RecordRef::Entity(entity) = record
                    else {
                        return None;
                    };
                    runtime
                        .read_truth()
                        .visible_entity_at_version(*entity, commit.version_id)
                        .or_else(|| {
                            runtime
                                .read_truth()
                                .visible_entity_at_version(*entity, previous_version)
                        })
                        .map(|record| record.kind.kind_id)
                })
                .collect::<Vec<_>>()
        });
        self.conditional_commit_journal
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .record(commit.commit_id, records);
        entity_kinds.into_iter().collect()
    }

    pub(in crate::domain_computation::primary_graph) fn conditional_entity_kind(
        &self,
        entity: &str,
    ) -> Option<worth_relational::facade::identity::KindId> {
        self.graph.layout.entity_kind(entity)
    }

    pub(in crate::domain_computation::primary_graph) fn record_conditional_maintenance_failure(
        &self,
        detail: impl Into<String>,
    ) {
        *self
            .conditional_maintenance_failure
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(detail.into());
    }

    pub(in crate::domain_computation::primary_graph) fn clear_conditional_maintenance_failure(
        &self,
    ) {
        *self
            .conditional_maintenance_failure
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner()) = None;
    }

    pub(in crate::domain_computation::primary_graph) fn conditional_maintenance_failure(
        &self,
    ) -> Option<String> {
        self.conditional_maintenance_failure
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone()
    }

    pub(in crate::domain_computation::primary_graph) fn conditional_commits_after_records(
        &self,
        sequence: u64,
        maximum: usize,
        records: impl IntoIterator<Item = worth_relational::facade::transactions::RecordRef>,
        include_whole_graph: bool,
    ) -> Result<conditional_commit_journal::WorthQueryConditionalCommitBatch, &'static str> {
        self.conditional_commit_journal
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .after_records(sequence, maximum, records, include_whole_graph)
    }

    pub(super) fn application_resource_support(
        &self,
    ) -> worth_query_admission::facade::resource_admission::WorthQueryExecutionResourceSupportSnapshot
    {
        self.resource_support.snapshot()
    }

    pub(in crate::domain_computation::primary_graph) fn bind_application_idempotency_intent(
        &self,
        batch: worth_relational::facade::transactions::WorkerIntentBatch,
        idempotency: super::application_attempt::WorthQueryApplicationIdempotencyBinding,
        outcome_identity: super::application_attempt::WorthQueryApplicationCommitOutcomeIdentity,
        emitted_effect_count: u64,
    ) -> worth_relational::facade::transactions::WorkerIntentBatch {
        batch.push(idempotency::idempotency_create_intent(
            self.graph.layout.provider_idempotency(),
            idempotency,
            outcome_identity,
            emitted_effect_count,
        ))
    }

    pub(in crate::domain_computation::primary_graph) fn bind_application_aftermath_causality_intent(
        &self,
        batch: worth_relational::facade::transactions::WorkerIntentBatch,
        causality: &crate::domain_computation::application_aftermath::WorthQueryPendingAftermathCausality,
        outcome_identity: super::application_attempt::WorthQueryApplicationCommitOutcomeIdentity,
    ) -> worth_relational::facade::transactions::WorkerIntentBatch {
        batch.push(aftermath_causality::aftermath_causality_create_intent(
            self.graph.layout.provider_aftermath_causality(),
            causality,
            outcome_identity,
        ))
    }

    pub(in crate::domain_computation::primary_graph) fn bind_application_dispatch_outbox(
        &self,
        mut batch: worth_relational::facade::transactions::WorkerIntentBatch,
        basis: dispatch_outbox::WorthQueryDispatchOutboxBasis<'_>,
    ) -> Result<
        (
            worth_relational::facade::transactions::WorkerIntentBatch,
            Option<
                crate::domain_computation::application_aftermath::WorthQueryPendingDispatchOutbox,
            >,
        ),
        &'static str,
    > {
        let record = dispatch_outbox::derive_dispatch_outbox_record(basis)
            .map_err(|_| "external-effect correlation derivation failed")?;
        let pending =
            crate::domain_computation::application_aftermath::bind_dispatch_outbox_create_intent(
                Some(self.graph.layout.provider_dispatch_outbox()),
                record.as_ref(),
            );
        if let Some((intent, _)) = &pending {
            batch = batch.push(intent.clone());
        }
        Ok((batch, pending.map(|(_, pending)| pending)))
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
    pub(super) fn application_attempt_resource_count(&self) -> usize {
        self.attempts
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .resource_count()
    }

    #[cfg(test)]
    pub(super) fn application_attempt_work(&self) -> WorthQueryApplicationAttemptWorkSnapshot {
        self.application_attempt_work.snapshot()
    }

    pub(in crate::domain_computation::primary_graph) fn observe_managed_application_bridge_plan(
        &self,
    ) {
        self.application_attempt_work.observe_managed_bridge_plan();
    }

    pub(in crate::domain_computation::primary_graph) fn observe_managed_application_cleanup(&self) {
        self.application_attempt_work.observe_managed_cleanup();
    }

    pub(in crate::domain_computation::primary_graph) fn observe_external_dispatch_admission(&self) {
        self.application_attempt_work
            .observe_external_dispatch_admission();
    }

    pub(super) fn take_lost_commit_response(&self) -> bool {
        self.take_fault(fault_port::WorthQueryPrimaryGraphFault::LostCommitResponse)
    }

    pub(super) fn take_rejected_session_prepare(&self) -> bool {
        self.take_fault(fault_port::WorthQueryPrimaryGraphFault::RejectedSessionPreparation)
    }

    pub(super) fn take_rejected_commit_before_transaction(&self) -> bool {
        self.take_fault(fault_port::WorthQueryPrimaryGraphFault::RejectedCommitBeforeTransaction)
    }

    pub(super) fn take_failed_index_publication(&self) -> bool {
        self.take_fault(fault_port::WorthQueryPrimaryGraphFault::FailedIndexPublication)
    }

    pub(super) fn observe_completed_application(
        &self,
        commit: &worth_relational::facade::history::CommitReference,
    ) -> Option<WorthQueryPrimaryGraphCommittedApplication> {
        self.completed_commit_evidence
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .observe(commit)
    }

    pub(in crate::domain_computation::primary_graph) fn observe_completed_application_for_session(
        &self,
        session: &crate::domain_computation::provider_session::WorthQueryProviderSessionTerminalBinding,
    ) -> Option<WorthQueryPrimaryGraphCommittedApplication> {
        self.completed_commit_evidence
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .observe_session(session)
    }

    pub(super) fn take_skipped_invariant_owner_execution(&self) -> bool {
        self.take_fault(fault_port::WorthQueryPrimaryGraphFault::SkippedInvariantOwnerExecution)
    }

    pub(super) fn take_relational_invariant_violation(&self) -> bool {
        self.take_fault(fault_port::WorthQueryPrimaryGraphFault::RelationalInvariantViolation)
    }

    #[cfg(test)]
    pub(super) fn take_undeclared_application_touch(&self) -> bool {
        self.take_fault(fault_port::WorthQueryPrimaryGraphFault::UndeclaredApplicationTouch)
    }

    fn take_fault(&self, fault: fault_port::WorthQueryPrimaryGraphFault) -> bool {
        self.fault_port.take(fault)
    }
}

pub(super) struct WorthQueryPrimaryLogicalGraph;
