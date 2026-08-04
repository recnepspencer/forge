mod application_decision_fact;
mod commit_causality;
mod committed_application;
mod decision_facts;
mod graph_participation;
mod idempotency;
mod invariant_execution;
mod mutation_work;
mod provisional_state;
mod resource_support;
mod session_lifecycle;

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use super::WorthQueryPrimaryGraphIntegrationHandle;
pub(super) use application_decision_fact::WorthQueryPrimaryGraphApplicationDecisionFact;
pub(super) use committed_application::WorthQueryPrimaryGraphCommittedApplication;
pub(super) use idempotency::WorthQueryProviderIdempotencyResolution;
pub use mutation_work::WorthQueryPrimaryMutationWorkEvidence;

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
    pub(super) invariant_work: BTreeMap<String, WorthQueryPrimaryMutationWorkEvidence>,
    pub(super) completed_mutation_work: Option<WorthQueryPrimaryMutationWorkEvidence>,
    pub(super) next_overlay: u64,
}

pub(super) struct WorthQueryPrimaryGraphOverlay {
    pub(super) facts: Vec<crate::domain_computation::WorthQueryProposedFact>,
}

pub(super) struct WorthQueryPrimaryGraphApplicationAttempt {
    pub(super) facts: BTreeMap<String, WorthQueryPrimaryGraphApplicationDecisionFact>,
    pub(super) expected_steps: Vec<crate::domain_computation::WorthQueryProvisionalEffectStep>,
    pub(super) batch: worth_relational::facade::transactions::WorkerIntentBatch,
    pub(super) emissions: super::application_attempt::WorthQueryAdmittedApplicationEmissionBatch,
    pub(super) idempotency: super::application_attempt::WorthQueryApplicationIdempotencyBinding,
    pub(super) branch: worth_relational::facade::history::BranchId,
    pub(super) graph_work_session:
        crate::domain_computation::provider_session::WorthQueryGraphWorkSessionIdentity,
    pub(super) decision_fact_count: usize,
}

impl WorthQueryPrimaryGraphProvider {
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

    pub(super) fn register_application_attempt(
        &self,
        session_identity: &str,
        facts: Vec<WorthQueryPrimaryGraphApplicationDecisionFact>,
        expected_steps: Vec<crate::domain_computation::WorthQueryProvisionalEffectStep>,
        mut batch: worth_relational::facade::transactions::WorkerIntentBatch,
        emissions: super::application_attempt::WorthQueryAdmittedApplicationEmissionBatch,
        idempotency: super::application_attempt::WorthQueryApplicationIdempotencyBinding,
        branch: worth_relational::facade::history::BranchId,
        graph_work_session:
            crate::domain_computation::provider_session::WorthQueryGraphWorkSessionIdentity,
        retained_authorization_fact_count: usize,
    ) -> Result<(), &'static str> {
        let emitted_effect_count = u64::try_from(emissions.len())
            .map_err(|_| "application emission count exceeds provider representation")?;
        batch = batch.push(idempotency::idempotency_create_intent(
            self.graph.layout.provider_idempotency(),
            idempotency,
            emitted_effect_count,
        ));
        if facts
            .iter()
            .filter_map(WorthQueryPrimaryGraphApplicationDecisionFact::session_identity)
            .any(|session| session != graph_work_session)
            || facts
                .iter()
                .filter_map(WorthQueryPrimaryGraphApplicationDecisionFact::session_identity)
                .count()
                != retained_authorization_fact_count
        {
            return Err("provider decision facts do not close over the graph-work session");
        }
        let decision_fact_count = facts.len();
        let facts = facts
            .into_iter()
            .map(|fact| (fact.locator_identity(), fact))
            .collect::<BTreeMap<_, _>>();
        if facts.len() != decision_fact_count {
            return Err("provider decision facts contain duplicate identities");
        }
        let mut sessions = self
            .sessions
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if sessions
            .application_attempts
            .insert(
                session_identity.to_owned(),
                WorthQueryPrimaryGraphApplicationAttempt {
                    facts,
                    expected_steps,
                    batch,
                    emissions,
                    idempotency,
                    branch,
                    graph_work_session,
                    decision_fact_count,
                },
            )
            .is_some()
        {
            return Err("provider session already owns an application attempt");
        }
        Ok(())
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

    pub(crate) fn completed_mutation_work(&self) -> Option<WorthQueryPrimaryMutationWorkEvidence> {
        self.sessions
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .completed_mutation_work
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
