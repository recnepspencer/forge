use super::super::super::{
    provider_recomparison::recover_equivalent_commit_evidence,
    WorthQueryApplicationCommitAuthorityBinding, WorthQueryApplicationCommitDenial,
    WorthQueryApplicationCommitReceipt, WorthQueryApplicationStaleAttempt,
    WorthQueryCommittedReceiptProjection,
};
use super::super::outcome::{progression_denied, WorthQueryProviderProgressionOutcome};
use crate::domain_computation::primary_graph::provider::WorthQueryProviderIdempotencyResolution;
use crate::domain_computation::{
    WorthQueryDecisionFactRequest, WorthQueryDecisionReadSetFreshnessOutcome,
    WorthQueryFreshDecisionReadSet, WorthQuerySessionBoundReadsAndEffects,
};
use worth_query_installation::facade::ApplicationSchema;

use super::super::super::WorthQueryApplicationCommitDenialStage as DenialStage;

pub(in crate::domain_computation::primary_graph::application_attempt) struct WorthQueryStaleEquivalentCommitReceiptPermit
{
    provider_session:
        crate::domain_computation::provider_session::WorthQueryProviderSessionTerminalBinding,
}

impl WorthQueryStaleEquivalentCommitReceiptPermit {
    fn mint(
        provider_session: crate::domain_computation::provider_session::WorthQueryProviderSessionTerminalBinding,
    ) -> Self {
        Self { provider_session }
    }

    pub(in crate::domain_computation::primary_graph::application_attempt) fn into_provider_session(
        self,
    ) -> crate::domain_computation::provider_session::WorthQueryProviderSessionTerminalBinding {
        self.provider_session
    }
}

pub(super) enum WorthQueryProviderReadSetProgression<'run> {
    Fresh(WorthQueryFreshProviderAttempt<'run>),
    Terminal(WorthQueryProviderProgressionOutcome),
}

pub(super) struct WorthQueryFreshProviderAttempt<'run> {
    pub(super) staged: WorthQuerySessionBoundReadsAndEffects<'run>,
    pub(super) read_set: WorthQueryFreshDecisionReadSet,
}

pub(super) fn compare_provider_read_set<'run, Schema, Operation, Input, Scope>(
    staged: WorthQuerySessionBoundReadsAndEffects<'run>,
    requests: Vec<WorthQueryDecisionFactRequest>,
    authority: &super::advance::WorthQueryApplicationCommitProgressionAuthority<
        '_,
        '_,
        Schema,
        Operation,
        Input,
        Scope,
    >,
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
            WorthQueryProviderReadSetProgression::Fresh(WorthQueryFreshProviderAttempt {
                staged,
                read_set,
            })
        }
        Ok(WorthQueryDecisionReadSetFreshnessOutcome::Stale(stale)) => {
            resolve_stale_provider_read_set(staged, stale.stale_fact_count(), authority)
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
    authority: &super::advance::WorthQueryApplicationCommitProgressionAuthority<
        '_,
        '_,
        Schema,
        Operation,
        Input,
        Scope,
    >,
) -> WorthQueryProviderReadSetProgression<'run>
where
    Schema: ApplicationSchema,
    Input: Clone + Send + Sync + 'static,
{
    let proof = match authority.application().authorize_application_commit(
        authority.admission(),
        authority.commit_authorization(),
        authority.serialization(),
    ) {
        Ok(proof) => proof,
        Err(_) => {
            let _ = staged.abort();
            return WorthQueryProviderReadSetProgression::Terminal(progression_denied(
                DenialStage::DecisionReadSet,
            ));
        }
    };
    let provider_session_affinity = staged.provider_session_affinity().identity();
    let provider_session = staged.provider_session_terminal_binding();
    let resolution = proof.govern((), |()| {
        authority
            .provider()
            .resolve_application_idempotency(provider_session_affinity)
    });
    let outcome = match resolution {
        Err(()) => progression_denied(DenialStage::DecisionReadSet),
        Ok(Ok(WorthQueryProviderIdempotencyResolution::Equivalent(receipt))) => {
            match WorthQueryCommittedReceiptProjection::resolve(receipt) {
                Ok(projection) => {
                    let receipt = WorthQueryApplicationCommitReceipt::from_stale_equivalent(
                        WorthQueryStaleEquivalentCommitReceiptPermit::mint(provider_session),
                        projection,
                        recover_equivalent_commit_evidence(
                            authority.admission().mutation_preconditions(),
                        ),
                        authority.admission().canonical_work(),
                        WorthQueryApplicationCommitAuthorityBinding::from_admission(
                            authority.admission(),
                            authority.idempotency(),
                        ),
                    );
                    WorthQueryProviderProgressionOutcome::AlreadyCommitted(receipt)
                }
                Err(_) => progression_denied(DenialStage::Idempotency),
            }
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
