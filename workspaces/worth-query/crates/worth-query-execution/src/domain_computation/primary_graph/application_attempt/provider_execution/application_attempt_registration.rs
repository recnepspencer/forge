use super::super::provider_binding::WorthQueryPreparedApplicationProviderAttempt;
use super::super::{
    WorthQueryApplicationCommitDenialStage as DenialStage, WorthQueryApplicationIdempotencyBinding,
};
use super::decision_facts::bind_provider_decision_facts;
use super::outcome::{progression_denied, WorthQueryProviderProgressionOutcome};
use crate::domain_computation::primary_graph::provider::WorthQueryApplicationAttemptRegistration;

pub(super) struct WorthQueryRegisteredProviderAttempt<'run> {
    pub(super) staged: crate::domain_computation::WorthQuerySessionBoundReadsAndEffects<'run>,
    pub(super) requests: Vec<crate::domain_computation::WorthQueryDecisionFactRequest>,
    pub(super) steps: Vec<crate::domain_computation::WorthQueryProvisionalEffectStep>,
    pub(super) session_identity: String,
    pub(super) dispatch_outbox:
        Option<crate::domain_computation::application_aftermath::WorthQueryDispatchOutboxRecord>,
}

pub(super) struct WorthQueryProviderAttemptRegistrationContext<'a, Schema, Operation, Input, Scope>
{
    pub(super) provider:
        &'a std::sync::Arc<super::super::super::provider::WorthQueryPrimaryGraphProvider>,
    pub(super) admission:
        &'a crate::domain_computation::primary_graph::WorthQueryAdmittedApplicationOperation<
            Schema,
            Operation,
            Input,
            Scope,
        >,
    pub(super) idempotency: WorthQueryApplicationIdempotencyBinding,
    pub(super) aftermath_causality: Option<
        &'a crate::domain_computation::application_aftermath::WorthQueryPendingAftermathCausality,
    >,
}

pub(super) fn register_provider_attempt<'run, Schema, Operation, Input, Scope>(
    staged: crate::domain_computation::WorthQuerySessionBoundReadsAndEffects<'run>,
    prepared: WorthQueryPreparedApplicationProviderAttempt,
    authorization: crate::domain_computation::authorization::WorthQueryProviderAuthorizationDecisionFacts,
    context: WorthQueryProviderAttemptRegistrationContext<'_, Schema, Operation, Input, Scope>,
) -> Result<WorthQueryRegisteredProviderAttempt<'run>, WorthQueryProviderProgressionOutcome> {
    let WorthQueryPreparedApplicationProviderAttempt {
        facts,
        steps,
        batch,
        emissions,
        preimage_demand,
    } = prepared;
    let (facts, requests) = match bind_provider_decision_facts(facts, authorization) {
        Ok(bound) => bound,
        Err(()) => {
            let _ = staged.abort();
            return Err(progression_denied(DenialStage::DecisionReadSet));
        }
    };
    let session_identity = staged.token_identity().to_owned();
    let dispatch_outbox =
        context
            .provider
            .register_application_attempt(WorthQueryApplicationAttemptRegistration {
                session_identity: staged.token_identity(),
                facts,
                expected_steps: steps.clone(),
                batch,
                emissions,
                idempotency: context.idempotency,
                branch: context.admission.graph_work().branch().relational().clone(),
                graph_work_session: context.admission.graph_work_session_identity(),
                retained_authorization_fact_count: context
                    .admission
                    .graph_work_decision_fact_count(),
                external_effect: context.admission.allowed_graph_contract().external_effect(),
                operation_slot: context.admission.operation(),
                operation_version: context.admission.binding_identity().generation(),
                preimage_demand: preimage_demand.as_ref(),
                aftermath_causality: context.aftermath_causality.cloned(),
            });
    match dispatch_outbox {
        Ok(dispatch_outbox) => Ok(WorthQueryRegisteredProviderAttempt {
            staged,
            requests,
            steps,
            session_identity,
            dispatch_outbox,
        }),
        Err(_) => {
            let _ = staged.abort();
            Err(progression_denied(DenialStage::ProviderPlan))
        }
    }
}
