//! Installed paths for exact active elevation and temporal posture.

use worth_query_declaration::facade::application_capability::{
    ApplicationCapabilityElevationDefinition, ApplicationCapabilityValueBinding,
};
use worth_relational::facade::authorization::RelationalAuthorizationPathPlan;
use worth_relational::facade::identity::KindId;
use worth_runtime_bridge::facade::{
    BridgeAuthorizationRuleContract, BridgeAuthorizationRuleEffect,
};

use super::ElevationRelations;
use crate::domain_computation::authorization::capability_binding_lowering::predicate;
use crate::domain_computation::authorization::capability_lowering::{bridge_rule, clause_identity};
use crate::domain_computation::authorization::capability_registry::{
    WorthQueryCapabilityPathTemplate, WorthQueryCapabilityRequestGuard,
};
use crate::domain_computation::authorization::WorthQueryOperationAuthorizationDenial;
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphLayout;

pub(super) struct ActiveElevationPathIndices {
    pub(super) active: usize,
    pub(super) not_before: usize,
    pub(super) not_after: usize,
    pub(super) expired: usize,
    pub(super) self_approval: usize,
}

pub(super) struct ElevationRuleSinks<'a> {
    pub(super) paths: &'a mut Vec<WorthQueryCapabilityPathTemplate>,
    pub(super) rules: &'a mut Vec<BridgeAuthorizationRuleContract>,
    pub(super) rule_path_indices: &'a mut Vec<Vec<Vec<usize>>>,
}

pub(super) fn compile(
    layout: &WorthQueryPrimaryGraphLayout,
    capability: &[u8; 32],
    elevation: &ApplicationCapabilityElevationDefinition,
    elevation_kind: KindId,
    relations: &ElevationRelations,
    mut sinks: ElevationRuleSinks<'_>,
) -> Result<ActiveElevationPathIndices, WorthQueryOperationAuthorizationDenial> {
    let active = push_path(
        active_path(
            layout,
            capability,
            elevation,
            elevation_kind,
            sinks.paths.len(),
            relations,
        )?,
        BridgeAuthorizationRuleEffect::Required,
        &mut sinks,
    );
    let not_before = push_temporal_path(capability, relations, &mut sinks)?;
    let not_after = push_temporal_path(capability, relations, &mut sinks)?;
    let expired = push_path(
        expired_path(
            layout,
            capability,
            elevation_kind,
            sinks.paths.len(),
            relations,
            elevation.states().expired(),
        )?,
        BridgeAuthorizationRuleEffect::Prohibited,
        &mut sinks,
    );
    let self_approval = push_path(
        self_approval_path(capability, sinks.paths.len(), relations),
        BridgeAuthorizationRuleEffect::Prohibited,
        &mut sinks,
    );
    Ok(ActiveElevationPathIndices {
        active,
        not_before,
        not_after,
        expired,
        self_approval,
    })
}

fn push_temporal_path(
    capability: &[u8; 32],
    relations: &ElevationRelations,
    sinks: &mut ElevationRuleSinks<'_>,
) -> Result<usize, WorthQueryOperationAuthorizationDenial> {
    Ok(push_path(
        temporal_path(capability, sinks.paths.len(), relations),
        BridgeAuthorizationRuleEffect::Required,
        sinks,
    ))
}

fn active_path(
    layout: &WorthQueryPrimaryGraphLayout,
    capability: &[u8; 32],
    elevation: &ApplicationCapabilityElevationDefinition,
    elevation_kind: KindId,
    path_index: usize,
    relations: &ElevationRelations,
) -> Result<WorthQueryCapabilityPathTemplate, WorthQueryOperationAuthorizationDenial> {
    Ok(WorthQueryCapabilityPathTemplate {
        plan: RelationalAuthorizationPathPlan::new(
            [
                relations.requester.clone(),
                relations.approver_reverse.clone(),
                relations.approver_forward.clone(),
                relations.grant.clone(),
                relations.resource.clone(),
            ],
            [predicate(
                layout,
                1,
                elevation_kind,
                elevation.states().active(),
            )?],
        ),
        identity: clause_identity(capability, path_index),
        guard: WorthQueryCapabilityRequestGuard::Unconditional,
        grant_ordinal: Some(4),
        elevation_ordinals: vec![1, 3],
        context_anchors: Vec::new(),
    })
}

fn temporal_path(
    capability: &[u8; 32],
    path_index: usize,
    relations: &ElevationRelations,
) -> WorthQueryCapabilityPathTemplate {
    WorthQueryCapabilityPathTemplate {
        plan: RelationalAuthorizationPathPlan::new(
            [
                relations.requester.clone(),
                relations.grant.clone(),
                relations.resource.clone(),
            ],
            [],
        ),
        identity: clause_identity(capability, path_index),
        guard: WorthQueryCapabilityRequestGuard::Unconditional,
        grant_ordinal: Some(2),
        elevation_ordinals: vec![1],
        context_anchors: Vec::new(),
    }
}

fn expired_path(
    layout: &WorthQueryPrimaryGraphLayout,
    capability: &[u8; 32],
    elevation_kind: KindId,
    path_index: usize,
    relations: &ElevationRelations,
    expired: &ApplicationCapabilityValueBinding,
) -> Result<WorthQueryCapabilityPathTemplate, WorthQueryOperationAuthorizationDenial> {
    let mut path = temporal_path(capability, path_index, relations);
    path.plan = path
        .plan
        .with_predicates([predicate(layout, 1, elevation_kind, expired)?]);
    Ok(path)
}

fn self_approval_path(
    capability: &[u8; 32],
    path_index: usize,
    relations: &ElevationRelations,
) -> WorthQueryCapabilityPathTemplate {
    WorthQueryCapabilityPathTemplate {
        plan: RelationalAuthorizationPathPlan::new(
            [
                relations.approver_forward.clone(),
                relations.grant.clone(),
                relations.resource.clone(),
            ],
            [],
        ),
        identity: clause_identity(capability, path_index),
        guard: WorthQueryCapabilityRequestGuard::Unconditional,
        grant_ordinal: Some(2),
        elevation_ordinals: vec![1],
        context_anchors: Vec::new(),
    }
}

fn push_path(
    path: WorthQueryCapabilityPathTemplate,
    effect: BridgeAuthorizationRuleEffect,
    sinks: &mut ElevationRuleSinks<'_>,
) -> usize {
    let path_index = sinks.paths.len();
    sinks.paths.push(path);
    let requirements = vec![vec![path_index]];
    sinks
        .rules
        .push(bridge_rule(effect, requirements.clone(), sinks.paths));
    sinks.rule_path_indices.push(requirements);
    path_index
}
