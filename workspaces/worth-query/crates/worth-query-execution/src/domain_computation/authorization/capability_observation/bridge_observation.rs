//! Runtime Bridge observation lowering from exact Relational evidence.

use worth_runtime_bridge::facade::{
    BridgeAuthorizationClauseObservation, BridgeAuthorizationDependencyCardinality,
    BridgeAuthorizationObservation, BridgeAuthorizationRequirementObservation,
    BridgeAuthorizationRuleObservation,
};

use super::super::capability_registry::{
    WorthQueryCapabilityRequestGuard, WorthQueryCapabilityRequestValueAxis,
    WorthQueryInstalledCapabilityPlan,
};
use super::super::retained_capability_request::WorthQueryRetainedCapabilityRequest;
use super::super::WorthQueryOperationAuthorizationDenial;

pub(super) fn lower_bridge_observation(
    installed: &WorthQueryInstalledCapabilityPlan,
    projection: &WorthQueryRetainedCapabilityRequest,
    evidence: &worth_relational::facade::authorization::RelationalAuthorizationObservationEvidence,
    dependency_identity: [u8; 32],
) -> Result<BridgeAuthorizationObservation, WorthQueryOperationAuthorizationDenial> {
    lower_observation(
        installed,
        projection,
        evidence,
        dependency_identity,
        installed.correspondence,
        &installed.bridge_rules,
        &installed.rule_path_indices,
        installed.paths.len(),
    )
}

pub(super) fn lower_upper_bound_observation(
    installed: &WorthQueryInstalledCapabilityPlan,
    projection: &WorthQueryRetainedCapabilityRequest,
    evidence: &worth_relational::facade::authorization::RelationalAuthorizationObservationEvidence,
    dependency_identity: [u8; 32],
) -> Result<BridgeAuthorizationObservation, WorthQueryOperationAuthorizationDenial> {
    let upper_bound = installed
        .upper_bound
        .as_ref()
        .ok_or_else(|| super::invalid_policy(installed.contract.name()))?;
    lower_observation(
        installed,
        projection,
        evidence,
        dependency_identity,
        upper_bound.correspondence,
        &upper_bound.bridge_rules,
        &upper_bound.rule_path_indices,
        upper_bound.path_count,
    )
}

#[allow(clippy::too_many_arguments)]
fn lower_observation(
    installed: &WorthQueryInstalledCapabilityPlan,
    projection: &WorthQueryRetainedCapabilityRequest,
    evidence: &worth_relational::facade::authorization::RelationalAuthorizationObservationEvidence,
    dependency_identity: [u8; 32],
    correspondence: worth_runtime_bridge::facade::BridgeAuthorizationCorrespondenceIdentity,
    bridge_rules: &[worth_runtime_bridge::facade::BridgeAuthorizationRuleContract],
    rule_path_indices: &[Vec<Vec<usize>>],
    path_count: usize,
) -> Result<BridgeAuthorizationObservation, WorthQueryOperationAuthorizationDenial> {
    if bridge_rules.len() != rule_path_indices.len() || evidence.paths().len() != path_count {
        return Err(super::invalid_policy(installed.contract.name()));
    }
    let mut rules = Vec::with_capacity(bridge_rules.len());
    for (rule, requirements) in bridge_rules.iter().zip(rule_path_indices) {
        let observed_requirements = requirements
            .iter()
            .map(|indices| {
                let clauses = indices
                    .iter()
                    .map(|index| observe_clause(installed, projection, evidence, *index))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(BridgeAuthorizationRequirementObservation::any(clauses))
            })
            .collect::<Result<Vec<_>, WorthQueryOperationAuthorizationDenial>>()?;
        rules.push(BridgeAuthorizationRuleObservation::all(
            rule.effect(),
            observed_requirements,
        ));
    }
    Ok(BridgeAuthorizationObservation::new(
        correspondence,
        dependency_identity,
        rules,
    ))
}

fn observe_clause(
    installed: &WorthQueryInstalledCapabilityPlan,
    projection: &WorthQueryRetainedCapabilityRequest,
    evidence: &worth_relational::facade::authorization::RelationalAuthorizationObservationEvidence,
    index: usize,
) -> Result<BridgeAuthorizationClauseObservation, WorthQueryOperationAuthorizationDenial> {
    let template = installed
        .paths
        .get(index)
        .ok_or_else(|| super::invalid_policy(installed.contract.name()))?;
    let path = evidence
        .paths()
        .get(index)
        .ok_or_else(|| super::invalid_policy(installed.contract.name()))?;
    let guard = guard_matches(&template.guard, projection);
    Ok(BridgeAuthorizationClauseObservation::new(
        template.identity,
        path.matched() && guard,
        path.exhaustive(),
        BridgeAuthorizationDependencyCardinality {
            entities: path.entities().len(),
            relations: path.relations().len(),
            adjacency_lists: path.adjacency_lists().len(),
            fields: path.fields().len(),
        },
    ))
}

fn guard_matches(
    guard: &WorthQueryCapabilityRequestGuard,
    projection: &WorthQueryRetainedCapabilityRequest,
) -> bool {
    let WorthQueryCapabilityRequestGuard::Accepted { axis, values } = guard else {
        return true;
    };
    let actual = match axis {
        WorthQueryCapabilityRequestValueAxis::Action => Some(&projection.action),
        WorthQueryCapabilityRequestValueAxis::Purpose => Some(&projection.purpose),
        WorthQueryCapabilityRequestValueAxis::Field => projection.field.as_ref(),
        WorthQueryCapabilityRequestValueAxis::Amount => projection.amount.as_ref(),
    };
    actual.is_some_and(|actual| values.contains(actual))
}
