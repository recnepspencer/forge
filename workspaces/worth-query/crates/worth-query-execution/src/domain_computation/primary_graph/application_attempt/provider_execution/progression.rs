use worth_query_installation::facade::APPLICATION_INVARIANT_SLOT;

use super::super::provider_binding::WorthQueryPreparedApplicationProviderAttempt;
use super::super::{
    provider_recomparison::{certify_provider_recomparison, recover_equivalent_commit_evidence},
    WorthQueryApplicationCommitDenial, WorthQueryApplicationCommitDenialStage as DenialStage,
    WorthQueryApplicationCommitOutcome, WorthQueryApplicationCommitReceipt,
    WorthQueryApplicationIdempotencyBinding, WorthQueryApplicationStaleAttempt,
};
use super::decision_facts::bind_provider_decision_facts;
use super::support::{denied, parse_provider_receipt};
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
    graph_work: &mut crate::domain_computation::provider_session::WorthQueryManagedMutationGraphWorkProgression<
        crate::domain_computation::primary_graph::WorthQueryApplicationSnapshotLease,
    >,
    prepared: WorthQueryPreparedApplicationProviderAttempt,
    authorization: crate::domain_computation::authorization::WorthQueryProviderAuthorizationDecisionFacts,
    commit_authorization: crate::domain_computation::authorization::WorthQueryCommitAuthorizationBasis,
    idempotency: WorthQueryApplicationIdempotencyBinding,
) -> WorthQueryApplicationCommitOutcome
where
    Schema: worth_query_installation::facade::ApplicationSchema,
{
    let staged = match running
        .admit_provider_execution_plan(graph)
        .and_then(|plan| plan.readmit())
        .and_then(|session| session.prepare())
    {
        Ok(prepared_session) => prepared_session.bind_reads_and_effects(),
        Err(_) => return denied(DenialStage::ProviderPlan),
    };
    let session_identity = staged.token_identity().to_owned();
    let WorthQueryPreparedApplicationProviderAttempt {
        facts,
        steps,
        batch,
        emissions,
    } = prepared;
    let (facts, requests) = match bind_provider_decision_facts(facts, authorization) {
        Ok(bound) => bound,
        Err(()) => return denied(DenialStage::DecisionReadSet),
    };
    if provider
        .register_application_attempt(
            staged.token_identity(),
            graph_work
                .session()
                .branch_affinity()
                .relational_branch()
                .clone(),
            facts,
            steps.clone(),
            batch,
            emissions,
            idempotency,
        )
        .is_err()
    {
        let _ = staged.abort();
        return denied(DenialStage::ProviderPlan);
    }
    let receipt = match staged.read_authority().capture_decision_read_set(requests) {
        Ok(receipt) => receipt,
        Err(_) => {
            let _ = staged.abort();
            return denied(DenialStage::DecisionReadSet);
        }
    };
    let fresh = match staged.read_authority().compare_decision_read_set(receipt) {
        Ok(WorthQueryDecisionReadSetFreshnessOutcome::Fresh(fresh)) => fresh,
        Ok(WorthQueryDecisionReadSetFreshnessOutcome::Stale(stale)) => {
            let serialization = provider.serialize_application_commit();
            let proof = match application.authorize_application_commit(
                admission,
                graph_work.session(),
                &commit_authorization,
                &serialization,
            ) {
                Ok(proof) => proof,
                Err(_) => {
                    let _ = staged.abort();
                    return denied(DenialStage::DecisionReadSet);
                }
            };
            let resolution = proof.govern((), |()| {
                provider.resolve_application_idempotency(&session_identity)
            });
            match resolution {
                Err(()) => {
                    let _ = staged.abort();
                    return denied(DenialStage::DecisionReadSet);
                }
                Ok(Ok(WorthQueryProviderIdempotencyResolution::Equivalent(receipt))) => {
                    let _ = staged.abort();
                    return WorthQueryApplicationCommitOutcome::AlreadyCommitted(
                        WorthQueryApplicationCommitReceipt::from_provider(
                            receipt,
                            recover_equivalent_commit_evidence(admission.mutation_preconditions()),
                            admission.canonical_work(),
                        ),
                    );
                }
                Ok(Ok(WorthQueryProviderIdempotencyResolution::Drift)) => {
                    let _ = staged.abort();
                    return WorthQueryApplicationCommitOutcome::Denied(
                        WorthQueryApplicationCommitDenial::idempotency_intent_drift(),
                    );
                }
                Ok(Ok(WorthQueryProviderIdempotencyResolution::Absent)) => {}
                Ok(Err(_)) => {
                    let _ = staged.abort();
                    return denied(DenialStage::Idempotency);
                }
            }
            let count = stale.stale_fact_count();
            let _ = staged.abort();
            return WorthQueryApplicationCommitOutcome::Stale(
                WorthQueryApplicationStaleAttempt::new(count),
            );
        }
        Err(_) => {
            let _ = staged.abort();
            return denied(DenialStage::DecisionReadSet);
        }
    };
    let lowered = match staged
        .effect_authority()
        .lower_provisional_program(&fresh, steps)
    {
        Ok(lowered) => lowered,
        Err(_) => {
            let _ = staged.abort();
            return denied(DenialStage::EffectLowering);
        }
    };
    let inspection = match staged.begin_provisional_attempt(fresh, lowered) {
        Ok(attempt) => attempt.materialize_proposed_state().inspect(),
        Err(_) => return denied(DenialStage::ProvisionalState),
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
            return denied(DenialStage::InvariantExecution);
        }
    };
    let progression = match inspection.admit_invariant_progression([receipt]) {
        Ok(progression) => progression,
        Err(_) => {
            inspection.discard();
            return denied(DenialStage::InvariantExecution);
        }
    };
    let candidate = match inspection.bind_invariant_progression(progression) {
        Ok(candidate) => candidate,
        Err((_, inspection)) => {
            inspection.discard();
            return denied(DenialStage::InvariantExecution);
        }
    };
    let invariant_completion = match crate::domain_computation::primary_graph::WorthQueryOperationInvariantExecutionCompletion::mint(
        *graph_work.session().identity(),
        graph_work.session().branch_affinity().relational_branch().clone(),
        candidate.invariant_receipt_identities().len(),
    ) {
        Ok(completion) => completion,
        Err(()) => {
            candidate.discard();
            return denied(DenialStage::InvariantExecution);
        }
    };
    if crate::domain_computation::provider_session::record_operation_invariant_execution_completion(
        graph_work.session_mut(),
        invariant_completion,
    )
    .is_err()
    {
        candidate.discard();
        return denied(DenialStage::InvariantExecution);
    }
    let serialization = provider.serialize_application_commit();
    let proof = match application.authorize_application_commit(
        admission,
        graph_work.session(),
        &commit_authorization,
        &serialization,
    ) {
        Ok(proof) => proof,
        Err(_) => {
            candidate.discard();
            return denied(DenialStage::DecisionReadSet);
        }
    };
    let outcome = proof.govern(candidate, |candidate| {
        match provider.resolve_application_idempotency(&session_identity) {
            Ok(WorthQueryProviderIdempotencyResolution::Absent) => {
                let outcome = finish_authorized_compare(
                    candidate.compare_and_commit(),
                    provider,
                    idempotency,
                    graph_work.session().branch_affinity().relational_branch(),
                    admission.mutation_preconditions(),
                    admission.canonical_work(),
                );
                record_effect_application(outcome, graph_work)
            }
            Ok(WorthQueryProviderIdempotencyResolution::Equivalent(receipt)) => {
                candidate.discard();
                WorthQueryApplicationCommitOutcome::AlreadyCommitted(
                    WorthQueryApplicationCommitReceipt::from_provider(
                        receipt,
                        recover_equivalent_commit_evidence(admission.mutation_preconditions()),
                        admission.canonical_work(),
                    ),
                )
            }
            Ok(WorthQueryProviderIdempotencyResolution::Drift) => {
                candidate.discard();
                WorthQueryApplicationCommitOutcome::Denied(
                    WorthQueryApplicationCommitDenial::idempotency_intent_drift(),
                )
            }
            Err(_) => {
                candidate.discard();
                denied(DenialStage::Idempotency)
            }
        }
    });
    match outcome {
        Ok(outcome) => outcome,
        Err(candidate) => {
            candidate.discard();
            WorthQueryApplicationCommitOutcome::Cancelled
        }
    }
}

fn record_effect_application(
    outcome: WorthQueryApplicationCommitOutcome,
    graph_work: &mut crate::domain_computation::provider_session::WorthQueryManagedMutationGraphWorkProgression<
        crate::domain_computation::primary_graph::WorthQueryApplicationSnapshotLease,
    >,
) -> WorthQueryApplicationCommitOutcome {
    let WorthQueryApplicationCommitOutcome::Committed(receipt) = &outcome else {
        return outcome;
    };
    let completion =
        crate::domain_computation::primary_graph::WorthQueryOperationEffectApplicationCompletion::mint(
            *graph_work.session().identity(),
            receipt,
        );
    if crate::domain_computation::provider_session::record_operation_effect_application_completion(
        graph_work.session_mut(),
        completion,
    )
    .is_ok()
    {
        outcome
    } else {
        WorthQueryApplicationCommitOutcome::Indeterminate
    }
}

fn finish_authorized_compare(
    compared: WorthQueryProviderCompareAndCommitOutcome,
    provider: &super::super::super::provider::WorthQueryPrimaryGraphProvider,
    idempotency: WorthQueryApplicationIdempotencyBinding,
    branch_id: &worth_relational::facade::history::BranchId,
    preconditions: &super::super::precondition_binding::WorthQueryBoundMutationPreconditions,
    canonical_work: worth_query_installation::facade::WorthQueryCanonicalWorkPhases,
) -> WorthQueryApplicationCommitOutcome {
    match compared {
        WorthQueryProviderCompareAndCommitOutcome::Committed {
            provider_receipt, ..
        } => parse_provider_receipt(&provider_receipt, branch_id.clone()).map_or_else(
            || WorthQueryApplicationCommitOutcome::Indeterminate,
            |receipt| {
                WorthQueryApplicationCommitOutcome::Committed(
                    WorthQueryApplicationCommitReceipt::from_provider(
                        receipt,
                        certify_provider_recomparison(preconditions),
                        canonical_work,
                    ),
                )
            },
        ),
        WorthQueryProviderCompareAndCommitOutcome::Stale(stale) => {
            WorthQueryApplicationCommitOutcome::Stale(WorthQueryApplicationStaleAttempt::new(
                stale.stale_fact_count(),
            ))
        }
        WorthQueryProviderCompareAndCommitOutcome::Denied(_) => denied(DenialStage::ProviderCommit),
        WorthQueryProviderCompareAndCommitOutcome::Indeterminate(_) => {
            resolve_indeterminate_commit(
                provider,
                idempotency,
                branch_id,
                preconditions,
                canonical_work,
            )
        }
    }
}

fn resolve_indeterminate_commit(
    provider: &super::super::super::provider::WorthQueryPrimaryGraphProvider,
    idempotency: WorthQueryApplicationIdempotencyBinding,
    branch_id: &worth_relational::facade::history::BranchId,
    preconditions: &super::super::precondition_binding::WorthQueryBoundMutationPreconditions,
    canonical_work: worth_query_installation::facade::WorthQueryCanonicalWorkPhases,
) -> WorthQueryApplicationCommitOutcome {
    match provider.resolve_idempotency_binding(idempotency, branch_id) {
        Ok(WorthQueryProviderIdempotencyResolution::Equivalent(receipt)) => {
            WorthQueryApplicationCommitOutcome::Committed(
                WorthQueryApplicationCommitReceipt::from_provider(
                    receipt,
                    certify_provider_recomparison(preconditions),
                    canonical_work,
                ),
            )
        }
        Ok(WorthQueryProviderIdempotencyResolution::Absent) => {
            WorthQueryApplicationCommitOutcome::Aborted
        }
        Ok(WorthQueryProviderIdempotencyResolution::Drift) | Err(_) => {
            WorthQueryApplicationCommitOutcome::Indeterminate
        }
    }
}
