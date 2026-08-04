use worth_query_installation::facade::APPLICATION_INVARIANT_SLOT;

use super::super::provider_binding::WorthQueryPreparedApplicationProviderAttempt;
use super::super::{
    provider_recomparison::{certify_provider_recomparison, recover_equivalent_commit_evidence},
    WorthQueryApplicationCommitDenial, WorthQueryApplicationCommitDenialStage as DenialStage,
    WorthQueryApplicationCommitReceipt, WorthQueryApplicationIdempotencyBinding,
    WorthQueryApplicationStaleAttempt, WorthQueryPendingApplicationCommitReceipt,
};
use super::decision_facts::bind_provider_decision_facts;
use super::outcome::{progression_denied, WorthQueryProviderProgressionOutcome};
use super::support::parse_provider_receipt;
use crate::domain_computation::primary_graph::provider::WorthQueryProviderIdempotencyResolution;
use crate::domain_computation::{
    WorthQueryDecisionReadSetFreshnessOutcome, WorthQueryInvariantStateLocator,
    WorthQueryProviderCompareAndCommitOutcome,
};

pub(super) fn execute_provider_progression<Schema, Operation, Input, Scope>(
    application: &crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime<
        Schema,
    >,
    running: &mut crate::domain_computation::WorthQueryRunningDirectRun,
    graph: &worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority,
    provider: &std::sync::Arc<super::super::super::provider::WorthQueryPrimaryGraphProvider>,
    admission: &crate::domain_computation::primary_graph::WorthQueryAdmittedApplicationOperation<
        Schema,
        Operation,
        Input,
        Scope,
    >,
    prepared: WorthQueryPreparedApplicationProviderAttempt,
    authorization: crate::domain_computation::authorization::WorthQueryProviderAuthorizationDecisionFacts,
    commit_authorization: crate::domain_computation::authorization::WorthQueryCommitAuthorizationBasis,
    idempotency: WorthQueryApplicationIdempotencyBinding,
    mutation_run: &crate::domain_computation::provider_session::WorthQueryMutationRunBinding,
    serialization: &super::super::super::provider::WorthQueryApplicationCommitSerialization<'_>,
) -> WorthQueryProviderProgressionOutcome
where
    Schema: worth_query_installation::facade::ApplicationSchema,
{
    let staged = match running
        .admit_provider_execution_plan(graph)
        .and_then(|plan| plan.readmit())
        .and_then(|session| session.prepare())
    {
        Ok(prepared_session) => prepared_session.bind_reads_and_effects(),
        Err(_) => return progression_denied(DenialStage::ProviderPlan),
    };
    if !mutation_run.admits(staged.plan()) {
        let _ = staged.abort();
        return progression_denied(DenialStage::ProviderPlan);
    }
    let session_identity = staged.token_identity().to_owned();
    let WorthQueryPreparedApplicationProviderAttempt {
        facts,
        steps,
        batch,
        emissions,
    } = prepared;
    let (facts, requests) = match bind_provider_decision_facts(facts, authorization) {
        Ok(bound) => bound,
        Err(()) => return progression_denied(DenialStage::DecisionReadSet),
    };
    if provider
        .register_application_attempt(
            staged.token_identity(),
            facts,
            steps.clone(),
            batch,
            emissions,
            idempotency,
            admission.graph_work().branch().relational().clone(),
            admission.graph_work_session_identity(),
            admission.graph_work_decision_fact_count(),
        )
        .is_err()
    {
        let _ = staged.abort();
        return progression_denied(DenialStage::ProviderPlan);
    }
    let receipt = match staged.read_authority().capture_decision_read_set(requests) {
        Ok(receipt) => receipt,
        Err(_) => {
            let _ = staged.abort();
            return progression_denied(DenialStage::DecisionReadSet);
        }
    };
    let fresh = match staged.read_authority().compare_decision_read_set(receipt) {
        Ok(WorthQueryDecisionReadSetFreshnessOutcome::Fresh(fresh)) => fresh,
        Ok(WorthQueryDecisionReadSetFreshnessOutcome::Stale(stale)) => {
            let proof = match application.authorize_application_commit(
                admission,
                &commit_authorization,
                serialization,
            ) {
                Ok(proof) => proof,
                Err(_) => {
                    let _ = staged.abort();
                    return progression_denied(DenialStage::DecisionReadSet);
                }
            };
            let resolution = proof.govern((), |()| {
                provider.resolve_application_idempotency(&session_identity)
            });
            match resolution {
                Err(()) => {
                    let _ = staged.abort();
                    return progression_denied(DenialStage::DecisionReadSet);
                }
                Ok(Ok(WorthQueryProviderIdempotencyResolution::Equivalent(receipt))) => {
                    let _ = staged.abort();
                    return WorthQueryProviderProgressionOutcome::AlreadyCommitted(
                        WorthQueryApplicationCommitReceipt::from_recovered_provider(
                            receipt,
                            recover_equivalent_commit_evidence(admission.mutation_preconditions()),
                            admission.canonical_work(),
                        ),
                    );
                }
                Ok(Ok(WorthQueryProviderIdempotencyResolution::Drift)) => {
                    let _ = staged.abort();
                    return WorthQueryProviderProgressionOutcome::Denied(
                        WorthQueryApplicationCommitDenial::idempotency_intent_drift(),
                    );
                }
                Ok(Ok(WorthQueryProviderIdempotencyResolution::Absent)) => {}
                Ok(Err(_)) => {
                    let _ = staged.abort();
                    return progression_denied(DenialStage::Idempotency);
                }
            }
            let count = stale.stale_fact_count();
            let _ = staged.abort();
            return WorthQueryProviderProgressionOutcome::Stale(
                WorthQueryApplicationStaleAttempt::new(count),
            );
        }
        Err(_) => {
            let _ = staged.abort();
            return progression_denied(DenialStage::DecisionReadSet);
        }
    };
    let lowered = match staged
        .effect_authority()
        .lower_provisional_program(&fresh, steps)
    {
        Ok(lowered) => lowered,
        Err(_) => {
            let _ = staged.abort();
            return progression_denied(DenialStage::EffectLowering);
        }
    };
    let inspection = match staged.begin_provisional_attempt(fresh, lowered) {
        Ok(attempt) => attempt.materialize_proposed_state().inspect(),
        Err(_) => return progression_denied(DenialStage::ProvisionalState),
    };
    let locators = inspection
        .facts()
        .iter()
        .map(|fact| {
            WorthQueryInvariantStateLocator::new("application-proposed-state", fact.identity())
        })
        .collect::<Result<Vec<_>, _>>();
    let receipt = match locators.and_then(|locators| {
        inspection
            .select_installed_invariant(APPLICATION_INVARIANT_SLOT)?
            .admit_state_load_plan(locators)?
            .execute()
    }) {
        Ok(receipt) => receipt,
        Err(_) => {
            inspection.discard();
            return progression_denied(DenialStage::InvariantExecution);
        }
    };
    let progression = match inspection.admit_invariant_progression([receipt]) {
        Ok(progression) => progression,
        Err(_) => {
            inspection.discard();
            return progression_denied(DenialStage::InvariantExecution);
        }
    };
    let candidate = match inspection.bind_invariant_progression(progression) {
        Ok(candidate) => candidate,
        Err((_, inspection)) => {
            inspection.discard();
            return progression_denied(DenialStage::InvariantExecution);
        }
    };
    let proof = match application.authorize_application_commit(
        admission,
        &commit_authorization,
        serialization,
    ) {
        Ok(proof) => proof,
        Err(_) => {
            candidate.discard();
            return progression_denied(DenialStage::DecisionReadSet);
        }
    };
    let outcome = proof.govern(candidate, |candidate| {
        match provider.resolve_application_idempotency(&session_identity) {
            Ok(WorthQueryProviderIdempotencyResolution::Absent) => finish_authorized_compare(
                candidate.compare_and_commit(),
                provider,
                idempotency,
                admission.graph_work().branch().relational(),
                admission.mutation_preconditions(),
                admission.canonical_work(),
            ),
            Ok(WorthQueryProviderIdempotencyResolution::Equivalent(receipt)) => {
                candidate.discard();
                WorthQueryProviderProgressionOutcome::AlreadyCommitted(
                    WorthQueryApplicationCommitReceipt::from_recovered_provider(
                        receipt,
                        recover_equivalent_commit_evidence(admission.mutation_preconditions()),
                        admission.canonical_work(),
                    ),
                )
            }
            Ok(WorthQueryProviderIdempotencyResolution::Drift) => {
                candidate.discard();
                WorthQueryProviderProgressionOutcome::Denied(
                    WorthQueryApplicationCommitDenial::idempotency_intent_drift(),
                )
            }
            Err(_) => {
                candidate.discard();
                progression_denied(DenialStage::Idempotency)
            }
        }
    });
    match outcome {
        Ok(outcome) => outcome,
        Err(candidate) => {
            candidate.discard();
            WorthQueryProviderProgressionOutcome::Cancelled
        }
    }
}

fn finish_authorized_compare(
    compared: WorthQueryProviderCompareAndCommitOutcome,
    provider: &super::super::super::provider::WorthQueryPrimaryGraphProvider,
    idempotency: WorthQueryApplicationIdempotencyBinding,
    branch: &worth_relational::facade::history::BranchId,
    preconditions: &super::super::precondition_binding::WorthQueryBoundMutationPreconditions,
    canonical_work: worth_query_installation::facade::WorthQueryCanonicalWorkPhases,
) -> WorthQueryProviderProgressionOutcome {
    match compared {
        WorthQueryProviderCompareAndCommitOutcome::Committed {
            provider_receipt, ..
        } => parse_provider_receipt(&provider_receipt, branch)
            .and_then(|receipt| {
                provider
                    .completed_mutation_work()
                    .map(|work| receipt.with_mutation_work(work))
            })
            .map_or_else(
                || WorthQueryProviderProgressionOutcome::Indeterminate,
                |receipt| {
                    WorthQueryProviderProgressionOutcome::Committed(
                        WorthQueryPendingApplicationCommitReceipt::from_provider(
                            receipt,
                            certify_provider_recomparison(preconditions),
                            canonical_work,
                        ),
                    )
                },
            ),
        WorthQueryProviderCompareAndCommitOutcome::Stale(stale) => {
            WorthQueryProviderProgressionOutcome::Stale(WorthQueryApplicationStaleAttempt::new(
                stale.stale_fact_count(),
            ))
        }
        WorthQueryProviderCompareAndCommitOutcome::Denied(_) => {
            progression_denied(DenialStage::ProviderCommit)
        }
        WorthQueryProviderCompareAndCommitOutcome::Indeterminate(_) => {
            resolve_indeterminate_commit(
                provider,
                idempotency,
                branch,
                preconditions,
                canonical_work,
            )
        }
    }
}

fn resolve_indeterminate_commit(
    provider: &super::super::super::provider::WorthQueryPrimaryGraphProvider,
    idempotency: WorthQueryApplicationIdempotencyBinding,
    branch: &worth_relational::facade::history::BranchId,
    preconditions: &super::super::precondition_binding::WorthQueryBoundMutationPreconditions,
    canonical_work: worth_query_installation::facade::WorthQueryCanonicalWorkPhases,
) -> WorthQueryProviderProgressionOutcome {
    match provider.resolve_idempotency_binding(idempotency, branch) {
        Ok(WorthQueryProviderIdempotencyResolution::Equivalent(receipt)) => {
            match provider.completed_mutation_work() {
                Some(work) => WorthQueryProviderProgressionOutcome::Committed(
                    WorthQueryPendingApplicationCommitReceipt::from_provider(
                        receipt.with_mutation_work(work),
                        certify_provider_recomparison(preconditions),
                        canonical_work,
                    ),
                ),
                None => WorthQueryProviderProgressionOutcome::Indeterminate,
            }
        }
        Ok(WorthQueryProviderIdempotencyResolution::Absent) => {
            WorthQueryProviderProgressionOutcome::Aborted
        }
        Ok(WorthQueryProviderIdempotencyResolution::Drift) | Err(_) => {
            WorthQueryProviderProgressionOutcome::Indeterminate
        }
    }
}
