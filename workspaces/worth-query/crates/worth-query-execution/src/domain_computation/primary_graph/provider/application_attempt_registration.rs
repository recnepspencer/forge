//! Atomic registration of Query-owned application-attempt records.

use std::collections::BTreeMap;

use super::{
    aftermath_causality::aftermath_causality_create_intent,
    dispatch_outbox::{derive_dispatch_outbox_record, WorthQueryDispatchOutboxBasis},
    idempotency, WorthQueryPrimaryGraphApplicationAttempt,
    WorthQueryPrimaryGraphApplicationDecisionFact, WorthQueryPrimaryGraphProvider,
};
use crate::domain_computation::application_aftermath::{
    dispatch_outbox_create_intent, WorthQueryDispatchOutboxRecord,
};

pub(in crate::domain_computation::primary_graph) struct WorthQueryApplicationAttemptRegistration<'a>
{
    pub(in crate::domain_computation::primary_graph) session_identity: &'a str,
    pub(in crate::domain_computation::primary_graph) facts:
        Vec<WorthQueryPrimaryGraphApplicationDecisionFact>,
    pub(in crate::domain_computation::primary_graph) expected_steps:
        Vec<crate::domain_computation::WorthQueryProvisionalEffectStep>,
    pub(in crate::domain_computation::primary_graph) batch:
        worth_relational::facade::transactions::WorkerIntentBatch,
    pub(in crate::domain_computation::primary_graph) emissions:
        super::super::application_attempt::WorthQueryAdmittedApplicationEmissionBatch,
    pub(in crate::domain_computation::primary_graph) idempotency:
        super::super::application_attempt::WorthQueryApplicationIdempotencyBinding,
    pub(in crate::domain_computation::primary_graph) branch:
        worth_relational::facade::history::BranchId,
    pub(in crate::domain_computation::primary_graph) graph_work_session:
        crate::domain_computation::provider_session::WorthQueryGraphWorkSessionIdentity,
    pub(in crate::domain_computation::primary_graph) retained_authorization_fact_count: usize,
    pub(in crate::domain_computation::primary_graph) external_effect:
        &'a worth_query_installation::facade::InstalledExternalEffectContract,
    pub(in crate::domain_computation::primary_graph) operation_slot: &'a str,
    pub(in crate::domain_computation::primary_graph) operation_version: u64,
    pub(in crate::domain_computation::primary_graph) preimage_demand:
        Option<&'a worth_query_installation::facade::InstalledPreImageDemand>,
    pub(in crate::domain_computation::primary_graph) aftermath_causality: Option<
        crate::domain_computation::application_aftermath::WorthQueryPendingAftermathCausality,
    >,
}

struct WorthQueryPreparedApplicationAttempt {
    session_identity: String,
    attempt: WorthQueryPrimaryGraphApplicationAttempt,
    dispatch_outbox: Option<WorthQueryDispatchOutboxRecord>,
}

impl WorthQueryPrimaryGraphProvider {
    /// Binds every Query-owned record that must land in the operation transaction.
    pub(in crate::domain_computation::primary_graph) fn register_application_attempt(
        &self,
        registration: WorthQueryApplicationAttemptRegistration<'_>,
    ) -> Result<Option<WorthQueryDispatchOutboxRecord>, &'static str> {
        let prepared = self.prepare_application_attempt(registration)?;
        let WorthQueryPreparedApplicationAttempt {
            session_identity,
            attempt,
            dispatch_outbox,
        } = prepared;
        let mut sessions = self
            .sessions
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if sessions
            .application_attempts
            .insert(session_identity, attempt)
            .is_some()
        {
            return Err("provider session already owns an application attempt");
        }
        Ok(dispatch_outbox)
    }

    fn prepare_application_attempt(
        &self,
        registration: WorthQueryApplicationAttemptRegistration<'_>,
    ) -> Result<WorthQueryPreparedApplicationAttempt, &'static str> {
        let WorthQueryApplicationAttemptRegistration {
            session_identity,
            facts,
            expected_steps,
            mut batch,
            emissions,
            idempotency,
            branch,
            graph_work_session,
            retained_authorization_fact_count,
            external_effect,
            operation_slot,
            operation_version,
            preimage_demand,
            aftermath_causality,
        } = registration;
        let emitted_effect_count = u64::try_from(emissions.len())
            .map_err(|_| "application emission count exceeds provider representation")?;
        let outcome_identity =
            super::super::application_attempt::WorthQueryApplicationCommitOutcomeIdentity::mint()
                .ok_or("application outcome identity space is exhausted")?;
        batch = batch.push(idempotency::idempotency_create_intent(
            self.graph.layout.provider_idempotency(),
            idempotency,
            outcome_identity,
            emitted_effect_count,
        ));
        if let Some(causality) = aftermath_causality.as_ref() {
            batch = batch.push(aftermath_causality_create_intent(
                self.graph.layout.provider_aftermath_causality(),
                causality,
                outcome_identity,
            ));
        }
        let external_payload = emissions.external_payload(external_effect)?;
        let (batch, dispatch_outbox) = self.bind_dispatch_outbox(
            batch,
            WorthQueryDispatchOutboxBasis {
                external_effect,
                external_payload: external_payload.as_deref(),
                operation_slot,
                operation_version,
                idempotency,
                outcome_identity,
                branch: &branch,
            },
        )?;
        let (facts, decision_fact_count) = bind_decision_facts(
            facts,
            &graph_work_session,
            retained_authorization_fact_count,
        )?;
        Ok(WorthQueryPreparedApplicationAttempt {
            session_identity: session_identity.to_owned(),
            attempt: WorthQueryPrimaryGraphApplicationAttempt {
                outcome_identity,
                facts,
                expected_steps,
                batch,
                emissions,
                idempotency,
                branch,
                graph_work_session,
                decision_fact_count,
                preimage_demand: preimage_demand.cloned(),
                aftermath_causality,
            },
            dispatch_outbox,
        })
    }

    fn bind_dispatch_outbox(
        &self,
        mut batch: worth_relational::facade::transactions::WorkerIntentBatch,
        basis: WorthQueryDispatchOutboxBasis<'_>,
    ) -> Result<
        (
            worth_relational::facade::transactions::WorkerIntentBatch,
            Option<WorthQueryDispatchOutboxRecord>,
        ),
        &'static str,
    > {
        let record = derive_dispatch_outbox_record(basis)
            .map_err(|_| "external-effect correlation derivation failed")?;
        if let Some(intent) = dispatch_outbox_create_intent(
            Some(self.graph.layout.provider_dispatch_outbox()),
            record.as_ref(),
        ) {
            batch = batch.push(intent);
        }
        Ok((batch, record))
    }
}

fn bind_decision_facts(
    facts: Vec<WorthQueryPrimaryGraphApplicationDecisionFact>,
    graph_work_session: &crate::domain_computation::provider_session::WorthQueryGraphWorkSessionIdentity,
    retained_authorization_fact_count: usize,
) -> Result<
    (
        BTreeMap<String, WorthQueryPrimaryGraphApplicationDecisionFact>,
        usize,
    ),
    &'static str,
> {
    let retained_facts = facts
        .iter()
        .filter_map(WorthQueryPrimaryGraphApplicationDecisionFact::session_identity);
    if retained_facts
        .clone()
        .any(|session| session != *graph_work_session)
        || retained_facts.count() != retained_authorization_fact_count
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
    Ok((facts, decision_fact_count))
}
