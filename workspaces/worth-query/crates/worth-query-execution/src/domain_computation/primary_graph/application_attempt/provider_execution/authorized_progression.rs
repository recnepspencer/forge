use super::super::{
    provider_recomparison::recover_equivalent_commit_evidence,
    WorthQueryApplicationCommitAuthorityBinding, WorthQueryApplicationCommitDenial,
    WorthQueryApplicationCommitDenialStage as DenialStage, WorthQueryApplicationCommitReceipt,
    WorthQueryApplicationIdempotencyBinding,
};
use super::aftermath_resolution::resolve_exact_committed_aftermath;
use super::commit_resolution::{finish_authorized_compare, WorthQueryAuthorizedCompareContext};
use super::outcome::{progression_denied, WorthQueryProviderProgressionOutcome};
use crate::domain_computation::primary_graph::provider::WorthQueryProviderIdempotencyResolution;

pub(super) struct WorthQueryAuthorizedProviderCommitContext<
    'a,
    'provider,
    Schema,
    Operation,
    Input,
    Scope,
> {
    pub(super) application:
        &'a crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime<
            Schema,
        >,
    pub(super) provider:
        &'a std::sync::Arc<super::super::super::provider::WorthQueryPrimaryGraphProvider>,
    pub(super) admission:
        &'a crate::domain_computation::primary_graph::WorthQueryAdmittedApplicationOperation<
            Schema,
            Operation,
            Input,
            Scope,
        >,
    pub(super) commit_authorization:
        &'a crate::domain_computation::authorization::WorthQueryCommitAuthorizationBasis,
    pub(super) serialization:
        &'a super::super::super::provider::WorthQueryApplicationCommitSerialization<'provider>,
    pub(super) idempotency: WorthQueryApplicationIdempotencyBinding,
    pub(super) session_identity: &'a str,
    pub(super) dispatch_outbox:
        Option<crate::domain_computation::application_aftermath::WorthQueryDispatchOutboxRecord>,
    pub(super) aftermath_causality: Option<
        &'a crate::domain_computation::application_aftermath::WorthQueryPendingAftermathCausality,
    >,
}

pub(super) fn resolve_authorized_provider_commit<Schema, Operation, Input, Scope>(
    candidate: crate::domain_computation::WorthQueryInvariantApprovedProposedState<'_>,
    context: WorthQueryAuthorizedProviderCommitContext<'_, '_, Schema, Operation, Input, Scope>,
) -> WorthQueryProviderProgressionOutcome
where
    Schema: worth_query_installation::facade::ApplicationSchema,
    Input: Clone + Send + Sync + 'static,
{
    let proof = match context.application.authorize_application_commit(
        context.admission,
        context.commit_authorization,
        context.serialization,
    ) {
        Ok(proof) => proof,
        Err(_) => {
            candidate.discard();
            return progression_denied(DenialStage::DecisionReadSet);
        }
    };
    match proof.govern(candidate, |candidate| {
        resolve_idempotency_under_authority(candidate, &context)
    }) {
        Ok(outcome) => outcome,
        Err(candidate) => {
            candidate.discard();
            WorthQueryProviderProgressionOutcome::Cancelled
        }
    }
}

fn resolve_idempotency_under_authority<Schema, Operation, Input, Scope>(
    candidate: crate::domain_computation::WorthQueryInvariantApprovedProposedState<'_>,
    context: &WorthQueryAuthorizedProviderCommitContext<'_, '_, Schema, Operation, Input, Scope>,
) -> WorthQueryProviderProgressionOutcome
where
    Input: Clone + Send + Sync + 'static,
{
    match context
        .provider
        .resolve_application_idempotency(context.session_identity)
    {
        Ok(WorthQueryProviderIdempotencyResolution::Absent) => finish_authorized_compare(
            candidate.compare_and_commit(),
            WorthQueryAuthorizedCompareContext {
                provider: context.provider,
                idempotency: context.idempotency,
                branch: context.admission.graph_work().branch().relational(),
                preconditions: context.admission.mutation_preconditions(),
                canonical_work: context.admission.canonical_work(),
                dispatch_outbox: context.dispatch_outbox.clone(),
                aftermath_causality: context.aftermath_causality.cloned(),
                authority_binding: WorthQueryApplicationCommitAuthorityBinding::from_admission(
                    context.admission,
                    context.idempotency,
                ),
            },
        ),
        Ok(WorthQueryProviderIdempotencyResolution::Equivalent(receipt)) => {
            candidate.discard();
            resolve_equivalent_commit(receipt, context)
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
}

fn resolve_equivalent_commit<Schema, Operation, Input, Scope>(
    receipt: super::super::super::provider::WorthQueryPrimaryGraphCommittedApplication,
    context: &WorthQueryAuthorizedProviderCommitContext<'_, '_, Schema, Operation, Input, Scope>,
) -> WorthQueryProviderProgressionOutcome
where
    Input: Clone + Send + Sync + 'static,
{
    match resolve_exact_committed_aftermath(context.provider, context.aftermath_causality, &receipt)
    {
        Ok(causality) => WorthQueryProviderProgressionOutcome::AlreadyCommitted(
            WorthQueryApplicationCommitReceipt::from_recovered_provider(
                receipt,
                recover_equivalent_commit_evidence(context.admission.mutation_preconditions()),
                context.admission.canonical_work(),
                WorthQueryApplicationCommitAuthorityBinding::from_admission(
                    context.admission,
                    context.idempotency,
                ),
            )
            .with_aftermath_causality(causality),
        ),
        Err(()) => WorthQueryProviderProgressionOutcome::Denied(
            WorthQueryApplicationCommitDenial::idempotency_intent_drift(),
        ),
    }
}
