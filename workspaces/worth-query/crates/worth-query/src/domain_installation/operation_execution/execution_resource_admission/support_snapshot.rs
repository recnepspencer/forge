use std::collections::BTreeSet;
use std::sync::Arc;

use super::WorthQueryExecutionResourceSupport;

pub(crate) fn operation_conditional_supports<
    D,
    O,
    F,
    L: crate::basis_lifecycle::BasisOperationLane,
>(
    bound: &crate::domain_installation::WorthQueryBoundDomainOperation<D, O, F, L>,
) -> Vec<(String, WorthQueryExecutionResourceSupport)> {
    bound
        .conditional_nodes()
        .iter()
        .filter_map(|node| {
            match node.lowering.location() {
            worth_query_installation::facade::WorthQueryConditionalNodeLocation::Operation {
                node_identity,
            } => Some((
                format!("operation:{node_identity}"),
                node.resource_support.clone(),
            )),
            worth_query_installation::facade::WorthQueryConditionalNodeLocation::WorkflowStage {
                ..
            } => None,
        }
        })
        .collect()
}

pub(crate) fn stage_conditional_supports<D, O, F, L: crate::basis_lifecycle::BasisOperationLane>(
    bound: &crate::domain_installation::WorthQueryBoundDomainOperation<D, O, F, L>,
    expected_stage: &str,
) -> Vec<(String, WorthQueryExecutionResourceSupport)> {
    bound
        .conditional_nodes()
        .iter()
        .filter_map(|node| {
            match node.lowering.location() {
            worth_query_installation::facade::WorthQueryConditionalNodeLocation::WorkflowStage {
                stage_identity,
                node_identity,
            } if stage_identity == expected_stage => Some((
                format!("stage:{stage_identity}:{node_identity}"),
                node.resource_support.clone(),
            )),
            _ => None,
        }
        })
        .collect()
}

pub(crate) fn commit_supports_for_roles<D, O, F, L: crate::basis_lifecycle::BasisOperationLane>(
    bound: &crate::domain_installation::WorthQueryBoundDomainOperation<D, O, F, L>,
    roles: &BTreeSet<&str>,
) -> Vec<(String, WorthQueryExecutionResourceSupport)> {
    if bound.commit_posture() != crate::domain_installation::WorthQueryBoundCommitPosture::Atomic {
        return Vec::new();
    }
    let mut groups = Vec::<(
        Arc<crate::domain_installation::graph_participation::WorthQueryInstalledGraphCommitAuthority>,
        Vec<String>,
    )>::new();
    for participation in bound
        .graph_participations()
        .iter()
        .filter(|participation| roles.contains(participation.role.as_str()))
    {
        let Some(authority) = &participation.record.commit_authority else {
            continue;
        };
        match groups.iter_mut().find(|(candidate, _)| {
            Arc::ptr_eq(candidate, authority) && candidate.identity() == authority.identity()
        }) {
            Some((_, group_roles)) => group_roles.push(participation.role.clone()),
            None => groups.push((Arc::clone(authority), vec![participation.role.clone()])),
        }
    }
    groups
        .into_iter()
        .map(|(authority, mut group_roles)| {
            group_roles.sort();
            (group_roles.join(","), authority.resource_support.clone())
        })
        .collect()
}
