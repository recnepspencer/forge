use std::collections::BTreeSet;

use worth_query_declaration::facade::application_capability::{
    ApplicationCapabilityCardinalityDimension, ApplicationCapabilityRelationDimension,
    ApplicationCapabilityRequestProjection,
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

use super::admitted_operation::WorthQueryAuthorizationCommitDependency;
use super::capability_registry::{
    WorthQueryCapabilityRequestGuard, WorthQueryCapabilityRequestValueAxis,
    WorthQueryInstalledCapabilityPlan,
};
use super::capability_request_resolution::{
    WorthQueryCapabilityContextKey, WorthQueryResolvedCapabilityRequest,
};
use super::{WorthQueryOperationAuthorizationDenial, WorthQueryOperationAuthorizationDenialKind};
use crate::domain_computation::authorization::WorthQueryAuthorizationTimeSample;

pub(super) fn observe_capability<Schema, Scope, Context>(
    relational: &worth_relational::facade::runtime::RelationalRuntime,
    snapshot: worth_relational::facade::snapshots::SnapshotHandle,
    bridge: &BridgeAuthorizationRuntime,
    installed: &WorthQueryInstalledCapabilityPlan,
    principal: worth_relational::facade::identity::EntityId,
    projection: &ApplicationCapabilityRequestProjection<Schema, Scope, Context>,
    resolved: &WorthQueryResolvedCapabilityRequest<Schema, Scope>,
    sample: &WorthQueryAuthorizationTimeSample,
) -> Result<WorthQueryAuthorizationCommitDependency, WorthQueryOperationAuthorizationDenial> {
    validate_projection_shape(installed, projection, resolved)?;
    let paths = installed
        .paths
        .iter()
        .enumerate()
        .map(|(index, template)| {
            let mut plan = template.plan.clone();
            let mut predicates = plan.predicates().to_vec();
            if index == 0 {
                append_grant_predicates(
                    installed,
                    projection,
                    sample,
                    &mut predicates,
                );
                if let (Some(traversal), Some(entity)) =
                    (&installed.request.related_relation, resolved.related)
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
                    resolved
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
            Ok(plan.with_predicates(predicates).with_entity_anchors(anchors))
        })
        .collect::<Result<Vec<_>, WorthQueryOperationAuthorizationDenial>>()?;
    let observation_plan = RelationalAuthorizationObservationPlan::try_new(
        snapshot,
        principal,
        resolved.resource.entity_id(),
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
        lower_bridge_observation(installed, projection, &evidence, dependency_identity)?;
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
    Ok(WorthQueryAuthorizationCommitDependency {
        relational: evidence,
        bridge: bridge_evidence,
    })
}

fn append_grant_predicates<Schema, Scope, Context>(
    installed: &WorthQueryInstalledCapabilityPlan,
    projection: &ApplicationCapabilityRequestProjection<Schema, Scope, Context>,
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
    if let (Some(field), Some(value)) = (&installed.request.field, projection.field_value()) {
        predicates.push(RelationalAuthorizationPredicate::new(
            1,
            installed.grant_kind,
            field.clone(),
            value.clone(),
        ));
    }
    if let (Some(field), Some(value)) = (&installed.request.amount, projection.amount_value()) {
        predicates.push(RelationalAuthorizationPredicate::compare(
            1,
            installed.grant_kind,
            field.clone(),
            RelationalAuthorizationFieldComparison::AtLeast,
            value.clone(),
        ));
    }
}

fn validate_projection_shape<Schema, Scope, Context>(
    installed: &WorthQueryInstalledCapabilityPlan,
    projection: &ApplicationCapabilityRequestProjection<Schema, Scope, Context>,
    resolved: &WorthQueryResolvedCapabilityRequest<Schema, Scope>,
) -> Result<(), WorthQueryOperationAuthorizationDenial> {
    let request = &installed.request;
    if projection.action() != &request.action
        || projection.purpose() != &request.purpose
        || projection.resource().entity() != request.resource_entity
        || projection.context_value().context() != request.context
        || projection.context_value().context_type() != request.context_type
        || !cardinality_admitted(request.cardinality, projection.cardinality_value())
        || projection.field_value().is_some() != request.field.is_some()
        || projection.amount_value().is_some() != request.amount.is_some()
    {
        return Err(projection_denial(installed.contract.name()));
    }
    let relation_matches = match (
        installed.contract.target().relation(),
        projection.related(),
    ) {
        (ApplicationCapabilityRelationDimension::NotApplicable, None) => true,
        (ApplicationCapabilityRelationDimension::Bound(expected), Some(actual)) => {
            expected == actual.relation()
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
    if expected_context.len() != resolved.context.len()
        || !expected_context
            .iter()
            .all(|key| resolved.context.contains_key(key))
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

fn lower_bridge_observation<Schema, Scope, Context>(
    installed: &WorthQueryInstalledCapabilityPlan,
    projection: &ApplicationCapabilityRequestProjection<Schema, Scope, Context>,
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

fn observe_clause<Schema, Scope, Context>(
    installed: &WorthQueryInstalledCapabilityPlan,
    projection: &ApplicationCapabilityRequestProjection<Schema, Scope, Context>,
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

fn guard_matches<Schema, Scope, Context>(
    guard: &WorthQueryCapabilityRequestGuard,
    projection: &ApplicationCapabilityRequestProjection<Schema, Scope, Context>,
) -> bool {
    let WorthQueryCapabilityRequestGuard::Accepted { axis, values } = guard else {
        return true;
    };
    let actual = match axis {
        WorthQueryCapabilityRequestValueAxis::Action => Some(projection.action()),
        WorthQueryCapabilityRequestValueAxis::Purpose => Some(projection.purpose()),
        WorthQueryCapabilityRequestValueAxis::Field => projection.field_value(),
        WorthQueryCapabilityRequestValueAxis::Amount => projection.amount_value(),
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
