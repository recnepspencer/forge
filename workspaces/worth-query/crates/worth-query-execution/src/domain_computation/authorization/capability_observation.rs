//! Capability observation across Relational and Runtime Bridge authority.

use std::collections::BTreeSet;

use worth_query_declaration::facade::application_capability::{
    ApplicationCapabilityCardinalityDimension, ApplicationCapabilityRelationDimension,
};
use worth_relational::facade::authorization::{
    RelationalAuthorizationEntityAnchor, RelationalAuthorizationObservationPlan,
};
use worth_runtime_bridge::facade::BridgeAuthorizationRuntime;

use super::capability_registry::WorthQueryInstalledCapabilityPlan;
use super::capability_request_resolution::WorthQueryCapabilityContextKey;
use super::retained_capability_request::WorthQueryRetainedCapabilityRequest;
use super::{
    WorthQueryAuthorizationDecisionFact, WorthQueryOperationAuthorizationDenial,
    WorthQueryOperationAuthorizationDenialKind,
};
use crate::domain_computation::authorization::WorthQueryAuthorizationTimeSample;

mod bridge_observation;
mod grant_selection;

pub(super) struct WorthQueryObservedCapabilityDecision {
    decision: WorthQueryAuthorizationDecisionFact,
    grant: worth_relational::facade::identity::EntityId,
}

impl WorthQueryObservedCapabilityDecision {
    pub(super) const fn new(
        decision: WorthQueryAuthorizationDecisionFact,
        grant: worth_relational::facade::identity::EntityId,
    ) -> Self {
        Self { decision, grant }
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        WorthQueryAuthorizationDecisionFact,
        worth_relational::facade::identity::EntityId,
    ) {
        (self.decision, self.grant)
    }
}

pub(super) fn observe_capability_policy(
    session_identity: crate::domain_computation::provider_session::WorthQueryGraphWorkSessionIdentity,
    relational: &worth_relational::facade::runtime::RelationalRuntime,
    snapshot: worth_relational::facade::snapshots::SnapshotHandle,
    bridge: &BridgeAuthorizationRuntime,
    installed: &WorthQueryInstalledCapabilityPlan,
    request: &WorthQueryRetainedCapabilityRequest,
    sample: &WorthQueryAuthorizationTimeSample,
    exact_grant: Option<worth_relational::facade::identity::EntityId>,
) -> Result<WorthQueryObservedCapabilityDecision, WorthQueryOperationAuthorizationDenial> {
    validate_projection_shape(installed, request)?;
    let (exact_grant, preparatory_relational_work) = resolve_exact_grant(
        relational,
        snapshot.clone(),
        installed,
        request,
        sample,
        exact_grant,
    )?;
    let paths = prepare_exact_policy_paths(installed, request, sample, exact_grant)?;
    let evidence = observe_exact_policy(relational, snapshot, installed, request, paths)?;
    let bridge_evidence = evaluate_exact_policy(bridge, installed, request, &evidence)?;
    let grant = extract_exact_grant(installed, &evidence)?;
    if exact_grant != grant {
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
        )
        .with_preparatory_relational_work(preparatory_relational_work),
        grant,
    })
}

fn resolve_exact_grant(
    relational: &worth_relational::facade::runtime::RelationalRuntime,
    snapshot: worth_relational::facade::snapshots::SnapshotHandle,
    installed: &WorthQueryInstalledCapabilityPlan,
    request: &WorthQueryRetainedCapabilityRequest,
    sample: &WorthQueryAuthorizationTimeSample,
    exact_grant: Option<worth_relational::facade::identity::EntityId>,
) -> Result<
    (
        worth_relational::facade::identity::EntityId,
        worth_relational::facade::authorization::RelationalAuthorizationObservationCounters,
    ),
    WorthQueryOperationAuthorizationDenial,
> {
    match exact_grant {
        Some(grant) => Ok((grant, Default::default())),
        None => {
            grant_selection::select_exact_grant(relational, snapshot, installed, request, sample)
                .map(grant_selection::SelectedCapabilityGrant::into_parts)
        }
    }
}

fn prepare_exact_policy_paths(
    installed: &WorthQueryInstalledCapabilityPlan,
    request: &WorthQueryRetainedCapabilityRequest,
    sample: &WorthQueryAuthorizationTimeSample,
    exact_grant: worth_relational::facade::identity::EntityId,
) -> Result<
    Vec<worth_relational::facade::authorization::RelationalAuthorizationPathPlan>,
    WorthQueryOperationAuthorizationDenial,
> {
    let grant_path_index = installed.grant_witness.path_index();
    installed
        .paths
        .iter()
        .enumerate()
        .map(|(index, template)| {
            let plan = if index == grant_path_index {
                grant_selection::prepare_grant_path(installed, request, sample)?
            } else {
                template.plan.clone()
            };
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
            if let Some(ordinal) = template.grant_ordinal {
                anchors.push(RelationalAuthorizationEntityAnchor::new(
                    ordinal,
                    installed.grant_kind,
                    exact_grant,
                ));
            }
            if !template.elevation_ordinals.is_empty() {
                let elevation = request
                    .elevation
                    .ok_or_else(|| projection_denial(installed.contract.name()))?;
                let bindings = installed
                    .elevation
                    .as_ref()
                    .ok_or_else(|| invalid_policy(installed.contract.name()))?;
                anchors.extend(template.elevation_ordinals.iter().map(|ordinal| {
                    RelationalAuthorizationEntityAnchor::new(
                        *ordinal,
                        bindings.elevation_kind,
                        elevation,
                    )
                }));
            }
            Ok(plan.with_entity_anchors(anchors))
        })
        .collect()
}

fn observe_exact_policy(
    relational: &worth_relational::facade::runtime::RelationalRuntime,
    snapshot: worth_relational::facade::snapshots::SnapshotHandle,
    installed: &WorthQueryInstalledCapabilityPlan,
    request: &WorthQueryRetainedCapabilityRequest,
    paths: Vec<worth_relational::facade::authorization::RelationalAuthorizationPathPlan>,
) -> Result<
    worth_relational::facade::authorization::RelationalAuthorizationObservationEvidence,
    WorthQueryOperationAuthorizationDenial,
> {
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
    relational
        .observe_authorization(observation_plan)
        .map_err(|_| {
            WorthQueryOperationAuthorizationDenial::new(
                WorthQueryOperationAuthorizationDenialKind::RelationalObservationRejected,
                installed.contract.name(),
            )
        })
}

fn evaluate_exact_policy(
    bridge: &BridgeAuthorizationRuntime,
    installed: &WorthQueryInstalledCapabilityPlan,
    request: &WorthQueryRetainedCapabilityRequest,
    evidence: &worth_relational::facade::authorization::RelationalAuthorizationObservationEvidence,
) -> Result<
    worth_runtime_bridge::facade::BridgeAuthorizationDecisionEvidence,
    WorthQueryOperationAuthorizationDenial,
> {
    let dependency_identity = *evidence.observation_identity().bytes();
    let bridge_observation = bridge_observation::lower_bridge_observation(
        installed,
        request,
        evidence,
        dependency_identity,
    )?;
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
        let kind = elevation_denial_kind(installed, evidence)
            .unwrap_or(WorthQueryOperationAuthorizationDenialKind::PermissionDenied);
        return Err(WorthQueryOperationAuthorizationDenial::new(
            kind,
            installed.contract.name(),
        ));
    }
    Ok(bridge_evidence)
}

fn extract_exact_grant(
    installed: &WorthQueryInstalledCapabilityPlan,
    evidence: &worth_relational::facade::authorization::RelationalAuthorizationObservationEvidence,
) -> Result<worth_relational::facade::identity::EntityId, WorthQueryOperationAuthorizationDenial> {
    evidence
        .paths()
        .get(installed.grant_witness.path_index())
        .and_then(|path| path.witness())
        .and_then(|witness| witness.entity_at(installed.grant_witness.entity_ordinal()))
        .ok_or_else(|| invalid_policy(installed.contract.name()))
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
        || projection.elevation.is_some() != installed.elevation.is_some()
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

fn elevation_denial_kind(
    installed: &WorthQueryInstalledCapabilityPlan,
    evidence: &worth_relational::facade::authorization::RelationalAuthorizationObservationEvidence,
) -> Option<WorthQueryOperationAuthorizationDenialKind> {
    let elevation = installed.elevation.as_ref()?;
    let required = evidence.paths().get(elevation.required_path_index)?;
    let self_approval = evidence.paths().get(elevation.self_approval_path_index)?;
    if self_approval.matched() {
        Some(WorthQueryOperationAuthorizationDenialKind::ElevationSelfApproval)
    } else if !required.matched() {
        Some(WorthQueryOperationAuthorizationDenialKind::ElevationInactive)
    } else {
        None
    }
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
