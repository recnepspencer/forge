use super::super::provider_execution::{
    bind_provider_decision_facts, progression_denied, WorthQueryProviderAttemptRegistrationContext,
    WorthQueryProviderProgressionOutcome, WorthQueryRegisteredProviderAttempt,
};
use super::super::WorthQueryApplicationCommitDenialStage as DenialStage;
use super::WorthQueryPreparedApplicationProviderAttempt;
use crate::domain_computation::primary_graph::provider::WorthQueryApplicationAttemptRegistration;

/// Proves that the effect owner consumed a completed provider attempt before
/// handing its associations to Primary Graph registration.
pub(in crate::domain_computation::primary_graph) struct WorthQueryProviderEffectRegistrationSeal {
    _owner_mint: (),
}

impl WorthQueryProviderEffectRegistrationSeal {
    fn mint() -> Self {
        Self { _owner_mint: () }
    }
}

pub(super) fn register_provider_attempt<'run, Schema, Operation, Input, Scope>(
    prepared: WorthQueryPreparedApplicationProviderAttempt,
    staged: crate::domain_computation::WorthQuerySessionBoundReadsAndEffects<'run>,
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
            return abort_registration(staged, DenialStage::DecisionReadSet);
        }
    };
    let dispatch_outbox =
        context
            .provider()
            .register_application_attempt(WorthQueryApplicationAttemptRegistration {
                effect_owner: WorthQueryProviderEffectRegistrationSeal::mint(),
                provider_session_binding: staged.provider_session_terminal_binding(),
                facts,
                expected_steps: steps.clone(),
                batch,
                emissions,
                idempotency: context.idempotency(),
                branch: context
                    .admission()
                    .graph_work()
                    .branch()
                    .relational()
                    .clone(),
                graph_work_session: context.admission().graph_work_session_identity(),
                retained_authorization_fact_count: context
                    .admission()
                    .graph_work_decision_fact_count(),
                external_effect: context
                    .admission()
                    .allowed_graph_contract()
                    .external_effect(),
                operation_slot: context.admission().operation(),
                operation_version: context.admission().binding_identity().generation(),
                preimage_demand: preimage_demand.as_ref(),
                aftermath_causality: context.aftermath_causality().cloned(),
            });
    match dispatch_outbox {
        Ok(dispatch_outbox) => Ok(WorthQueryRegisteredProviderAttempt::from_registration(
            staged,
            requests,
            steps,
            dispatch_outbox,
        )),
        Err(_) => abort_registration(staged, DenialStage::ProviderPlan),
    }
}

fn abort_registration<'run, T>(
    staged: crate::domain_computation::WorthQuerySessionBoundReadsAndEffects<'run>,
    stage: DenialStage,
) -> Result<T, WorthQueryProviderProgressionOutcome> {
    let _ = staged.abort();
    Err(progression_denied(stage))
}
