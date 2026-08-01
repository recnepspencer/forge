//! Capability observation across Relational and Runtime Bridge authority.

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

use super::capability_projection_validation::{context_key, projection_denial};
use super::capability_registry::{
    WorthQueryCapabilityRequestGuard, WorthQueryCapabilityRequestValueAxis,
    WorthQueryInstalledCapabilityPlan,
};
use super::retained_capability_request::WorthQueryRetainedCapabilityRequest;
use super::{
    WorthQueryAuthorizationDecisionFact, WorthQueryOperationAuthorizationDenial,
    WorthQueryOperationAuthorizationDenialKind,
};
use crate::domain_computation::authorization::WorthQueryAuthorizationTimeSample;

pub(in crate::domain_computation) struct WorthQueryObservedCapabilityDecision {
    decision: WorthQueryAuthorizationDecisionFact,
    grant: worth_relational::facade::identity::EntityId,
}

impl WorthQueryObservedCapabilityDecision {
    pub(in crate::domain_computation) fn into_parts(
        self,
    ) -> (
        WorthQueryAuthorizationDecisionFact,
        worth_relational::facade::identity::EntityId,
    ) {
        (self.decision, self.grant)
    }
}

pub(in crate::domain_computation) fn observe_capability(
    session_identity: worth_foundational::facade::CanonicalDigestId,
    relational: &worth_relational::facade::runtime::RelationalRuntime,
    snapshot: worth_relational::facade::snapshots::SnapshotHandle,
    bridge: &BridgeAuthorizationRuntime,
    installed: &WorthQueryInstalledCapabilityPlan,
    request: &WorthQueryRetainedCapabilityRequest,
    sample: &WorthQueryAuthorizationTimeSample,
    exact_grant: Option<worth_relational::facade::identity::EntityId>,
) -> Result<WorthQueryObservedCapabilityDecision, WorthQueryOperationAuthorizationDenial> {
    super::capability_projection_validation::validate_retained_capability_shape(
        installed, request,
    )?;
    let grant_path_index = installed.grant_witness.path_index();
    let paths = installed
        .paths
        .iter()
        .enumerate()
        .map(|(index, template)| {
            let mut plan = template.plan.clone();
            let mut predicates = plan.predicates().to_vec();
            if index == grant_path_index {
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
            let mut anchors = template
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
            if index == grant_path_index {
                if let Some(grant) = exact_grant {
                    anchors.push(RelationalAuthorizationEntityAnchor::new(
                        1,
                        installed.grant_kind,
                        grant,
                    ));
                }
            }
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
    let grant = evidence
        .paths()
        .get(installed.grant_witness.path_index())
        .and_then(|path| path.witness())
        .and_then(|witness| witness.entity_at(installed.grant_witness.entity_ordinal()))
        .ok_or_else(|| invalid_policy(installed.contract.name()))?;
    if exact_grant.is_some_and(|expected| expected != grant) {
        return Err(WorthQueryOperationAuthorizationDenial::new(
            WorthQueryOperationAuthorizationDenialKind::InconsistentDecision,
            installed.contract.name(),
        ));
    }
    Ok(WorthQueryObservedCapabilityDecision {
        decision: WorthQueryAuthorizationDecisionFact::new(
            session_identity,
            evidence,
            bridge_evidence,
        ),
        grant,
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

fn invalid_policy(subject: &str) -> WorthQueryOperationAuthorizationDenial {
    WorthQueryOperationAuthorizationDenial::new(
        WorthQueryOperationAuthorizationDenialKind::InvalidInstalledPolicy,
        subject,
    )
}
