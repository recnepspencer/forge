//! Resolving one authorized compare-and-commit into a progression outcome.

use crate::domain_computation::application_aftermath::WorthQueryDispatchOutboxRecord;
use crate::domain_computation::primary_graph::provider::WorthQueryProviderIdempotencyResolution;
use crate::domain_computation::WorthQueryProviderCompareAndCommitOutcome;

use super::super::provider_recomparison::certify_provider_recomparison;
use super::super::{
    WorthQueryApplicationCommitDenialStage as DenialStage, WorthQueryApplicationIdempotencyBinding,
    WorthQueryApplicationStaleAttempt, WorthQueryPendingApplicationCommitReceipt,
};
use super::aftermath_resolution::resolve_exact_committed_aftermath;
use super::outcome::{progression_denied, WorthQueryProviderProgressionOutcome};
use super::support::parse_provider_receipt;

/// Everything needed to turn a provider compare-and-commit answer into a
/// progression outcome, including the dispatch anchor the commit carried.
pub(super) struct WorthQueryAuthorizedCompareContext<'a> {
    pub provider: &'a super::super::super::provider::WorthQueryPrimaryGraphProvider,
    pub idempotency: WorthQueryApplicationIdempotencyBinding,
    pub branch: &'a worth_relational::facade::history::BranchId,
    pub preconditions: &'a super::super::precondition_binding::WorthQueryBoundMutationPreconditions,
    pub canonical_work: worth_query_installation::facade::WorthQueryCanonicalWorkPhases,
    pub dispatch_outbox: Option<WorthQueryDispatchOutboxRecord>,
    pub aftermath_causality: Option<
        crate::domain_computation::application_aftermath::WorthQueryPendingAftermathCausality,
    >,
    pub authority_binding: super::super::WorthQueryApplicationCommitAuthorityBinding,
}

pub(super) fn finish_authorized_compare(
    compared: WorthQueryProviderCompareAndCommitOutcome,
    context: WorthQueryAuthorizedCompareContext<'_>,
) -> WorthQueryProviderProgressionOutcome {
    match compared {
        WorthQueryProviderCompareAndCommitOutcome::Committed {
            provider_receipt, ..
        } => {
            let resolved =
                parse_provider_receipt(&provider_receipt, context.provider, context.branch)
                    .ok_or("provider receipt could not be parsed after commit")
                    .and_then(|receipt| resolve_committed_components(&context, receipt));
            resolved.map_or_else(
                |detail| {
                    WorthQueryProviderProgressionOutcome::Indeterminate(
                        super::support::unknown_commit_recovery_evidence(detail),
                    )
                },
                |resolved| seal_committed_outcome(context, resolved),
            )
        }
        WorthQueryProviderCompareAndCommitOutcome::Stale(stale) => {
            WorthQueryProviderProgressionOutcome::Stale(WorthQueryApplicationStaleAttempt::new(
                stale.stale_fact_count(),
            ))
        }
        WorthQueryProviderCompareAndCommitOutcome::Denied(_) => {
            progression_denied(DenialStage::ProviderCommit)
        }
        WorthQueryProviderCompareAndCommitOutcome::Indeterminate(failure) => {
            resolve_indeterminate_commit(context, failure)
        }
    }
}

/// A commit whose answer never arrived is re-read through idempotency.
///
/// The recovery kind is carried, not re-derived: a session that failed on the
/// commit path demands commit recovery even when the record is now visible.
fn resolve_indeterminate_commit(
    context: WorthQueryAuthorizedCompareContext<'_>,
    failure: crate::domain_computation::provider_session::WorthQueryProviderSessionFailure,
) -> WorthQueryProviderProgressionOutcome {
    let evidence =
        super::super::WorthQueryApplicationUnresolvedCommitEvidence::from_provider_session_failure(
            super::super::WorthQueryApplicationCommitRecoveryKind::CommitRecoveryRequired,
            &failure,
        );
    match context
        .provider
        .resolve_idempotency_binding(context.idempotency, context.branch)
    {
        Ok(WorthQueryProviderIdempotencyResolution::Equivalent(receipt)) => {
            match resolve_committed_components(&context, receipt) {
                Ok(resolved) => seal_committed_outcome(context, resolved),
                Err(_) => WorthQueryProviderProgressionOutcome::Indeterminate(evidence),
            }
        }
        Ok(WorthQueryProviderIdempotencyResolution::Absent) => {
            WorthQueryProviderProgressionOutcome::Aborted
        }
        Ok(WorthQueryProviderIdempotencyResolution::Drift) | Err(_) => {
            WorthQueryProviderProgressionOutcome::Indeterminate(evidence)
        }
    }
}

struct WorthQueryResolvedCommitComponents {
    receipt: crate::domain_computation::primary_graph::provider::WorthQueryPrimaryGraphCommittedApplication,
    retained_preimage:
        Option<crate::domain_computation::application_aftermath::WorthQueryRetainedPreImage>,
    causality: Option<
        crate::domain_computation::application_aftermath::WorthQueryCommittedAftermathCausality,
    >,
}

fn resolve_committed_components(
    context: &WorthQueryAuthorizedCompareContext<'_>,
    receipt: crate::domain_computation::primary_graph::provider::WorthQueryPrimaryGraphCommittedApplication,
) -> Result<WorthQueryResolvedCommitComponents, &'static str> {
    let causality = resolve_exact_committed_aftermath(
        context.provider,
        context.aftermath_causality.as_ref(),
        &receipt,
    )
    .map_err(|()| "committed aftermath causality could not be recovered")?;
    let commit = receipt.commit_reference().commit_id;
    let evidence = context
        .provider
        .take_completed_commit_evidence(commit)
        .ok_or("exact commit evidence could not be recovered")?;
    let (mutation_work, retained_preimage) = evidence.into_parts();
    Ok(WorthQueryResolvedCommitComponents {
        receipt: receipt.with_mutation_work(mutation_work),
        retained_preimage,
        causality,
    })
}

fn seal_committed_outcome(
    context: WorthQueryAuthorizedCompareContext<'_>,
    resolved: WorthQueryResolvedCommitComponents,
) -> WorthQueryProviderProgressionOutcome {
    WorthQueryProviderProgressionOutcome::Committed(
        WorthQueryPendingApplicationCommitReceipt::from_provider(
            resolved.receipt,
            certify_provider_recomparison(context.preconditions),
            context.canonical_work,
            context.authority_binding,
        )
        .with_dispatch_outbox(context.dispatch_outbox)
        .with_retained_preimage(resolved.retained_preimage)
        .with_aftermath_causality(resolved.causality),
    )
}
