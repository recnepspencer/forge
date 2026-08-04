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

pub(super) fn parse_provider_receipt(
    value: &str,
    branch: &worth_relational::facade::history::BranchId,
) -> Option<
    crate::domain_computation::primary_graph::provider::WorthQueryPrimaryGraphCommittedApplication,
> {
    let mut parts = value.split(':');
    if parts.next()? != "primary-application-commit" {
        return None;
    }
    let runtime = parts.next()?.parse().ok()?;
    let commit = parts.next()?.parse().ok()?;
    let changed = parts.next()?.parse().ok()?;
    let emitted = parts.next()?.parse().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Some(
        crate::domain_computation::primary_graph::provider::WorthQueryPrimaryGraphCommittedApplication::new(
        runtime,
        branch.clone(),
        worth_relational::facade::history::CommitId(commit),
        changed,
        emitted,
    ))
}

pub(super) fn denied(
    stage: WorthQueryApplicationCommitDenialStage,
) -> WorthQueryApplicationCommitOutcome {
    WorthQueryApplicationCommitOutcome::Denied(
        WorthQueryApplicationCommitDenial::provider_rejected(stage),
    )
}
