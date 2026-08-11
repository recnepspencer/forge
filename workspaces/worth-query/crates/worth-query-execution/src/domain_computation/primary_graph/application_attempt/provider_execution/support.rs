use super::super::{
    WorthQueryApplicationCommitDenial, WorthQueryApplicationCommitDenialStage,
    WorthQueryApplicationCommitOutcome,
};

pub(in crate::domain_computation) fn application_resource_request(
    contracts: &worth_query_installation::facade::WorthQueryCompiledApplicationOperationContracts,
) -> Option<worth_query_declaration::facade::domain_computation::WorthQueryExecutionResourceRequest>
{
    let envelope = contracts.execution_strategy()?.envelope();
    worth_query_declaration::facade::domain_computation::WorthQueryExecutionResourceRequest::new(
        envelope.scale_ceilings().clone(),
        envelope.resource_ceilings().clone(),
        envelope.cancellation_safe_point().clone(),
    )
    .ok()
}

pub(in crate::domain_computation::primary_graph) fn parse_provider_receipt(
    value: &str,
    provider: &crate::domain_computation::primary_graph::provider::WorthQueryPrimaryGraphProvider,
    branch: &worth_relational::facade::history::BranchId,
) -> Option<
    crate::domain_computation::primary_graph::provider::WorthQueryPrimaryGraphCommittedApplication,
> {
    let mut parts = value.split(':');
    if parts.next()? != "primary-application-commit" {
        return None;
    }
    let runtime: u64 = parts.next()?.parse().ok()?;
    let commit: u64 = parts.next()?.parse().ok()?;
    let changed: usize = parts.next()?.parse().ok()?;
    let emitted: usize = parts.next()?.parse().ok()?;
    let outcome_identity: u64 = parts.next()?.parse().ok()?;
    if parts.next().is_some() {
        return None;
    }
    let commit = worth_relational::facade::history::CommitId(commit);
    let commit = provider.committed_branch_head(branch, commit)?;
    let committed = provider.observe_completed_application(&commit)?;
    let expected_outcome = crate::domain_computation::primary_graph::application_attempt::WorthQueryApplicationCommitOutcomeIdentity::restore(outcome_identity)?;
    let exact = committed.runtime_instance_id() == runtime
        && committed.changed_record_count() == changed
        && committed.emitted_effect_count() == emitted
        && committed.application_outcome_identity() == Some(expected_outcome);
    exact.then_some(committed)
}

pub(super) fn denied(
    stage: WorthQueryApplicationCommitDenialStage,
) -> WorthQueryApplicationCommitOutcome {
    WorthQueryApplicationCommitOutcome::Denied(
        WorthQueryApplicationCommitDenial::provider_rejected(stage),
    )
}

pub(super) fn unknown_commit_recovery_evidence(
    detail: &'static str,
) -> super::super::WorthQueryApplicationUnresolvedCommitEvidence {
    let failure = crate::domain_computation::provider_session::WorthQueryProviderSessionFailure::new(
        crate::domain_computation::provider_session::WorthQueryProviderSessionDenialKind::ProviderRejected,
        crate::domain_computation::provider_session::WorthQueryProviderSessionProtocolStage::Commit,
        detail,
        crate::domain_computation::provider_session::WorthQueryProviderSessionProtocolCounters::default(),
    );
    super::super::WorthQueryApplicationUnresolvedCommitEvidence::from_provider_session_failure(
        super::super::WorthQueryApplicationCommitRecoveryKind::CommitRecoveryRequired,
        &failure,
    )
}
