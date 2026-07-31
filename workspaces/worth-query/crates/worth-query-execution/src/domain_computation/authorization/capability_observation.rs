//! Capability observation across Relational and Runtime Bridge authority.

use std::collections::BTreeSet;

use worth_query_declaration::facade::application_capability::{
    ApplicationCapabilityCardinalityDimension, ApplicationCapabilityRelationDimension,
};
use worth_relational::facade::authorization::{
    RelationalAuthorizationEntityAnchor, RelationalAuthorizationFieldComparison,
    RelationalAuthorizationObservationPlan, RelationalAuthorizationPredicate,
    RelationalAuthorizationRelatedEntityConstraint,
};
use worth_runtime_bridge::facade::{
    BridgeAuthorizationClauseObservation, BridgeAuthorizationDependencyCardinality,
    BridgeAuthorizationObservation, BridgeAuthorizationRequirementObservation,
    BridgeAuthorizationRuleObservation, BridgeAuthorizationRuntime,
};

use super::capability_registry::{
    WorthQueryCapabilityRequestGuard, WorthQueryCapabilityRequestValueAxis,
    WorthQueryInstalledCapabilityPlan,
};
use super::capability_request_resolution::WorthQueryCapabilityContextKey;
use super::retained_capability_request::WorthQueryRetainedCapabilityRequest;
use super::{
    WorthQueryAuthorizationDecisionFact, WorthQueryOperationAuthorizationDenial,
    WorthQueryOperationAuthorizationDenialKind,
};
use crate::domain_computation::authorization::WorthQueryAuthorizationTimeSample;

pub(super) fn observe_capability(
    relational: &worth_relational::facade::runtime::RelationalRuntime,
    snapshot: worth_relational::facade::snapshots::SnapshotHandle,
    bridge: &BridgeAuthorizationRuntime,
    installed: &WorthQueryInstalledCapabilityPlan,
    request: &WorthQueryRetainedCapabilityRequest,
    sample: &WorthQueryAuthorizationTimeSample,
) -> Result<WorthQueryAuthorizationDecisionFact, WorthQueryOperationAuthorizationDenial> {
    validate_projection_shape(installed, request)?;
    let paths = installed
        .paths
        .iter()
        .enumerate()
        .map(|(index, template)| {
            let mut plan = template.plan.clone();
            let mut predicates = plan.predicates().to_vec();
            if index == 0 {
                append_grant_predicates(installed, request, sample, &mut predicates);
                if let (Some(traversal), Some(entity)) =
                    (&installed.request.related_relation, request.related)
                {
                    plan = plan.with_related_entities([
                        RelationalAuthorizationRelatedEntityConstraint::new(
                            1,
                            traversal.clone(),
                            entity,
                        ),
                    ]);
                }
            }
            let anchors = template
                .context_anchors
                .iter()
                .map(|anchor| {
                    let key = context_key(anchor);
                    request
                        .context
                        .get(&key)
                        .copied()
                        .map(|entity| {
                            RelationalAuthorizationEntityAnchor::new(
                                anchor.ordinal,
                                anchor.kind,
                                entity,
                            )
                        })
                        .ok_or_else(|| projection_denial(&anchor.slot))
                })
                .collect::<Result<Vec<_>, _>>()?;
            Ok(plan
                .with_predicates(predicates)
                .with_entity_anchors(anchors))
        })
        .collect::<Result<Vec<_>, WorthQueryOperationAuthorizationDenial>>()?;
    let observation_plan = RelationalAuthorizationObservationPlan::try_new(
        snapshot,
        request.principal,
        request.resource,
        installed.principal_kind,
        installed.scope_kind,
        paths,
        [],
    )
    .map_err(|_| invalid_policy(installed.contract.name()))?;
    let evidence = relational
        .observe_authorization(observation_plan)
        .map_err(|_| {
            WorthQueryOperationAuthorizationDenial::new(
                WorthQueryOperationAuthorizationDenialKind::RelationalObservationRejected,
                installed.contract.name(),
            )
        })?;
    let dependency_identity = *evidence.observation_identity().bytes();
    let bridge_observation =
        lower_bridge_observation(installed, request, &evidence, dependency_identity)?;
    let bridge_evidence = bridge.evaluate(bridge_observation).map_err(|_| {
        WorthQueryOperationAuthorizationDenial::new(
            WorthQueryOperationAuthorizationDenialKind::BridgeEvaluationRejected,
            installed.contract.name(),
        )
    })?;
    if bridge_evidence.dependency_identity() != &dependency_identity
        || !bridge.retains(&bridge_evidence)
    {
        return Err(WorthQueryOperationAuthorizationDenial::new(
            WorthQueryOperationAuthorizationDenialKind::InconsistentDecision,
            installed.contract.name(),
        ));
    }
    if !bridge_evidence.is_allowed() {
        return Err(WorthQueryOperationAuthorizationDenial::new(
            WorthQueryOperationAuthorizationDenialKind::PermissionDenied,
            installed.contract.name(),
        ));
    }
    Ok(WorthQueryAuthorizationDecisionFact {
        relational: evidence,
        bridge: bridge_evidence,
    })
}

fn append_grant_predicates(
    installed: &WorthQueryInstalledCapabilityPlan,
    projection: &WorthQueryRetainedCapabilityRequest,
    sample: &WorthQueryAuthorizationTimeSample,
    predicates: &mut Vec<RelationalAuthorizationPredicate>,
) {
    predicates.push(RelationalAuthorizationPredicate::compare(
        1,
        installed.grant_kind,
        installed.request.not_before.clone(),
        RelationalAuthorizationFieldComparison::AtMost,
        sample.value().clone(),
    ));
    predicates.push(RelationalAuthorizationPredicate::compare(
        1,
        installed.grant_kind,
        installed.request.not_after.clone(),
        RelationalAuthorizationFieldComparison::AtLeast,
        sample.value().clone(),
    ));
    if let (Some(field), Some(value)) = (&installed.request.field, projection.field.as_ref()) {
        predicates.push(RelationalAuthorizationPredicate::new(
            1,
            installed.grant_kind,
            field.clone(),
            value.clone(),
        ));
    }
    if let (Some(field), Some(value)) = (&installed.request.amount, projection.amount.as_ref()) {
        predicates.push(RelationalAuthorizationPredicate::compare(
            1,
            installed.grant_kind,
            field.clone(),
            RelationalAuthorizationFieldComparison::AtLeast,
            value.clone(),
        ));
    }
}

fn validate_projection_shape(
    installed: &WorthQueryInstalledCapabilityPlan,
    projection: &WorthQueryRetainedCapabilityRequest,
) -> Result<(), WorthQueryOperationAuthorizationDenial> {
    let request = &installed.request;
    if projection.action != request.action
        || projection.purpose != request.purpose
        || projection.resource_entity.as_ref() != request.resource_entity
        || projection.context_name.as_ref() != request.context
        || projection.context_type.as_ref() != request.context_type
        || !cardinality_admitted(request.cardinality, projection.cardinality)
        || projection.field.is_some() != request.field.is_some()
        || projection.amount.is_some() != request.amount.is_some()
    {
        return Err(projection_denial(installed.contract.name()));
    }
    let relation_matches = match (
        installed.contract.target().relation(),
        projection.related_relation.as_ref(),
    ) {
        (ApplicationCapabilityRelationDimension::NotApplicable, None) => true,
        (ApplicationCapabilityRelationDimension::Bound(expected), Some(actual)) => {
            expected == actual
        }
        _ => false,
    };
    if !relation_matches {
        return Err(projection_denial(installed.contract.name()));
    }
    let expected_context = installed
        .paths
        .iter()
        .flat_map(|path| path.context_anchors.iter().map(context_key))
        .collect::<BTreeSet<_>>();
    if expected_context.len() != projection.context.len()
        || !expected_context
            .iter()
            .all(|key| projection.context.contains_key(key))
    {
        return Err(projection_denial(installed.contract.name()));
    }
    Ok(())
}

const fn cardinality_admitted(
    installed: ApplicationCapabilityCardinalityDimension,
    requested: u32,
) -> bool {
    match installed {
        ApplicationCapabilityCardinalityDimension::One => requested == 1,
        ApplicationCapabilityCardinalityDimension::Many => requested > 0,
        ApplicationCapabilityCardinalityDimension::Bounded(maximum) => {
            requested > 0 && requested <= maximum
        }
    }
}

fn context_key(
    anchor: &super::capability_registry::WorthQueryCapabilityContextAnchor,
) -> WorthQueryCapabilityContextKey {
    WorthQueryCapabilityContextKey {
        context: anchor.context.clone(),
        context_type: anchor.context_type.clone(),
        slot: anchor.slot.clone(),
        slot_type: anchor.slot_type.clone(),
        entity: anchor.entity.clone(),
    }
}

fn lower_bridge_observation(
    installed: &WorthQueryInstalledCapabilityPlan,
    projection: &WorthQueryRetainedCapabilityRequest,
    evidence: &worth_relational::facade::authorization::RelationalAuthorizationObservationEvidence,
    dependency_identity: [u8; 32],
) -> Result<BridgeAuthorizationObservation, WorthQueryOperationAuthorizationDenial> {
    if installed.bridge_rules.len() != installed.rule_path_indices.len()
        || evidence.paths().len() != installed.paths.len()
    {
        return Err(invalid_policy(installed.contract.name()));
    }
    let mut rules = Vec::with_capacity(installed.bridge_rules.len());
    for (rule, requirements) in installed
        .bridge_rules
        .iter()
        .zip(&installed.rule_path_indices)
    {
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
        installed.correspondence,
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
        .ok_or_else(|| invalid_policy(installed.contract.name()))?;
    let path = evidence
        .paths()
        .get(index)
        .ok_or_else(|| invalid_policy(installed.contract.name()))?;
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

fn projection_denial(subject: impl Into<String>) -> WorthQueryOperationAuthorizationDenial {
    WorthQueryOperationAuthorizationDenial::new(
        WorthQueryOperationAuthorizationDenialKind::CapabilityProjectionRejected,
        subject,
    )
}

fn invalid_policy(subject: &str) -> WorthQueryOperationAuthorizationDenial {
    WorthQueryOperationAuthorizationDenial::new(
        WorthQueryOperationAuthorizationDenialKind::InvalidInstalledPolicy,
        subject,
    )
}
