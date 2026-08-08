use super::super::{
    provider_recomparison::recover_equivalent_commit_evidence,
    WorthQueryApplicationCommitAuthorityBinding, WorthQueryApplicationCommitDenial,
    WorthQueryApplicationCommitReceipt, WorthQueryApplicationStaleAttempt,
};
use super::outcome::{progression_denied, WorthQueryProviderProgressionOutcome};
use crate::domain_computation::primary_graph::{
    provider::{
        WorthQueryApplicationCommitSerialization, WorthQueryPrimaryGraphProvider,
        WorthQueryProviderIdempotencyResolution,
    },
    WorthQueryAdmittedApplicationOperation, WorthQueryPrimaryGraphApplicationRuntime,
};
use crate::domain_computation::{
    authorization::WorthQueryCommitAuthorizationBasis, WorthQueryDecisionFactRequest,
    WorthQueryDecisionReadSetFreshnessOutcome, WorthQueryFreshDecisionReadSet,
    WorthQuerySessionBoundReadsAndEffects,
};
use worth_query_installation::facade::ApplicationSchema;

use super::super::WorthQueryApplicationCommitDenialStage as DenialStage;

pub(super) struct WorthQueryProviderReadSetContext<'a, 'provider, Schema, Operation, Input, Scope> {
    pub(super) application: &'a WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    pub(super) provider: &'a WorthQueryPrimaryGraphProvider,
    pub(super) admission:
        &'a WorthQueryAdmittedApplicationOperation<Schema, Operation, Input, Scope>,
    pub(super) commit_authorization: &'a WorthQueryCommitAuthorizationBasis,
    pub(super) session_identity: &'a str,
    pub(super) serialization: &'a WorthQueryApplicationCommitSerialization<'provider>,
    pub(super) idempotency: super::super::WorthQueryApplicationIdempotencyBinding,
}

pub(super) enum WorthQueryProviderReadSetProgression<'run> {
    Fresh {
        staged: WorthQuerySessionBoundReadsAndEffects<'run>,
        read_set: WorthQueryFreshDecisionReadSet,
    },
    Terminal(WorthQueryProviderProgressionOutcome),
}

pub(super) fn compare_provider_read_set<'run, Schema, Operation, Input, Scope>(
    staged: WorthQuerySessionBoundReadsAndEffects<'run>,
    requests: Vec<WorthQueryDecisionFactRequest>,
    context: WorthQueryProviderReadSetContext<'_, '_, Schema, Operation, Input, Scope>,
) -> WorthQueryProviderReadSetProgression<'run>
where
    Schema: ApplicationSchema,
    Input: Clone + Send + Sync + 'static,
{
    let receipt = match staged.read_authority().capture_decision_read_set(requests) {
        Ok(receipt) => receipt,
        Err(_) => {
            let _ = staged.abort();
            return WorthQueryProviderReadSetProgression::Terminal(progression_denied(
                DenialStage::DecisionReadSet,
            ));
        }
    };
    match staged.read_authority().compare_decision_read_set(receipt) {
        Ok(WorthQueryDecisionReadSetFreshnessOutcome::Fresh(read_set)) => {
            WorthQueryProviderReadSetProgression::Fresh { staged, read_set }
        }
        Ok(WorthQueryDecisionReadSetFreshnessOutcome::Stale(stale)) => {
            resolve_stale_provider_read_set(staged, stale.stale_fact_count(), context)
        }
        Err(_) => {
            let _ = staged.abort();
            WorthQueryProviderReadSetProgression::Terminal(progression_denied(
                DenialStage::DecisionReadSet,
            ))
        }
    }
}

fn resolve_stale_provider_read_set<'run, Schema, Operation, Input, Scope>(
    staged: WorthQuerySessionBoundReadsAndEffects<'run>,
    stale_fact_count: usize,
    context: WorthQueryProviderReadSetContext<'_, '_, Schema, Operation, Input, Scope>,
) -> WorthQueryProviderReadSetProgression<'run>
where
    Schema: ApplicationSchema,
    Input: Clone + Send + Sync + 'static,
{
    let proof = match context.application.authorize_application_commit(
        context.admission,
        context.commit_authorization,
        context.serialization,
    ) {
        Ok(proof) => proof,
        Err(_) => {
            let _ = staged.abort();
            return WorthQueryProviderReadSetProgression::Terminal(progression_denied(
                DenialStage::DecisionReadSet,
            ));
        }
    };
    let resolution = proof.govern((), |()| {
        context
            .provider
            .resolve_application_idempotency(context.session_identity)
    });
    let outcome = match resolution {
        Err(()) => progression_denied(DenialStage::DecisionReadSet),
        Ok(Ok(WorthQueryProviderIdempotencyResolution::Equivalent(receipt))) => {
            WorthQueryProviderProgressionOutcome::AlreadyCommitted(
                WorthQueryApplicationCommitReceipt::from_recovered_provider(
                    receipt,
                    recover_equivalent_commit_evidence(context.admission.mutation_preconditions()),
                    context.admission.canonical_work(),
                    WorthQueryApplicationCommitAuthorityBinding::from_admission(
                        context.admission,
                        context.idempotency,
                    ),
                ),
            )
        }
        Ok(Ok(WorthQueryProviderIdempotencyResolution::Drift)) => {
            WorthQueryProviderProgressionOutcome::Denied(
                WorthQueryApplicationCommitDenial::idempotency_intent_drift(),
            )
        }
        Ok(Ok(WorthQueryProviderIdempotencyResolution::Absent)) => {
            WorthQueryProviderProgressionOutcome::Stale(WorthQueryApplicationStaleAttempt::new(
                stale_fact_count,
            ))
        }
        Ok(Err(_)) => progression_denied(DenialStage::Idempotency),
    };
    let _ = staged.abort();
    WorthQueryProviderReadSetProgression::Terminal(outcome)
}
