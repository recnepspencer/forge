//! Installed active-elevation composition.

use worth_query_declaration::facade::application_capability::{
    ApplicationCapabilityElevationDefinition, ErasedApplicationCapabilityContract,
};
use worth_relational::facade::authorization::{
    RelationalAuthorizationPathPlan, RelationalAuthorizationTraversal,
    RelationalAuthorizationTraversalDirection,
};
use worth_relational::facade::identity::KindId;
use worth_runtime_bridge::facade::{
    BridgeAuthorizationRuleContract, BridgeAuthorizationRuleEffect,
};

use super::{bridge_rule, clause_identity};
use crate::domain_computation::authorization::capability_binding_lowering::{
    kind, predicate, relation,
};
use crate::domain_computation::authorization::capability_registry::{
    WorthQueryCapabilityElevationBindings, WorthQueryCapabilityPathTemplate,
    WorthQueryCapabilityRequestGuard,
};
use crate::domain_computation::authorization::{
    authorization_denial, WorthQueryOperationAuthorizationDenial,
};
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphLayout;

mod approver_conflict;

#[allow(clippy::too_many_arguments)]
pub(super) fn compile_elevation_rules(
    contract: &ErasedApplicationCapabilityContract,
    layout: &WorthQueryPrimaryGraphLayout,
    capability: &[u8; 32],
    principal_kind: KindId,
    grant_kind: KindId,
    scope_kind: KindId,
    paths: &mut Vec<WorthQueryCapabilityPathTemplate>,
    rules: &mut Vec<BridgeAuthorizationRuleContract>,
    rule_path_indices: &mut Vec<Vec<Vec<usize>>>,
) -> Result<Option<WorthQueryCapabilityElevationBindings>, WorthQueryOperationAuthorizationDenial> {
    let Some(elevation) = contract.elevation().definition() else {
        return Ok(None);
    };
    let elevation_kind = kind(layout, elevation.identity().entity())?;
    let relations = lower_relations(layout, contract, elevation)?;
    relations.validate_endpoints(
        contract,
        principal_kind,
        elevation_kind,
        grant_kind,
        scope_kind,
    )?;

    let required_path_index = paths.len();
    paths.push(required_path(
        layout,
        capability,
        elevation,
        elevation_kind,
        required_path_index,
        &relations,
    )?);
    push_rule(
        BridgeAuthorizationRuleEffect::Required,
        required_path_index,
        paths,
        rules,
        rule_path_indices,
    );

    let self_approval_path_index = paths.len();
    paths.push(self_approval_path(
        capability,
        self_approval_path_index,
        &relations,
    ));
    push_rule(
        BridgeAuthorizationRuleEffect::Prohibited,
        self_approval_path_index,
        paths,
        rules,
        rule_path_indices,
    );
    let approver_conflict_requirements = approver_conflict::compile(
        contract,
        layout,
        capability,
        &relations,
        paths,
        rules,
        rule_path_indices,
    )?;
    Ok(Some(WorthQueryCapabilityElevationBindings::new(
        elevation_kind,
        required_path_index,
        self_approval_path_index,
        approver_conflict_requirements,
    )))
}

struct ElevationRelations {
    requester: RelationalAuthorizationTraversal,
    approver_reverse: RelationalAuthorizationTraversal,
    approver_forward: RelationalAuthorizationTraversal,
    grant: RelationalAuthorizationTraversal,
    resource: RelationalAuthorizationTraversal,
}

impl ElevationRelations {
    fn validate_endpoints(
        &self,
        contract: &ErasedApplicationCapabilityContract,
        principal_kind: KindId,
        elevation_kind: KindId,
        grant_kind: KindId,
        scope_kind: KindId,
    ) -> Result<(), WorthQueryOperationAuthorizationDenial> {
        if self.requester.from_kind() != principal_kind
            || self.requester.to_kind() != elevation_kind
            || self.approver_forward.from_kind() != principal_kind
            || self.approver_forward.to_kind() != elevation_kind
            || self.approver_reverse.from_kind() != principal_kind
            || self.approver_reverse.to_kind() != elevation_kind
            || self.grant.from_kind() != elevation_kind
            || self.grant.to_kind() != grant_kind
            || self.resource.from_kind() != grant_kind
            || self.resource.to_kind() != scope_kind
        {
            return Err(authorization_denial(
                contract.name(),
                "elevation relation endpoints changed",
            ));
        }
        Ok(())
    }
}

fn lower_relations(
    layout: &WorthQueryPrimaryGraphLayout,
    contract: &ErasedApplicationCapabilityContract,
    elevation: &ApplicationCapabilityElevationDefinition,
) -> Result<ElevationRelations, WorthQueryOperationAuthorizationDenial> {
    let lower = |binding, direction| relation(layout, binding, direction);
    Ok(ElevationRelations {
        requester: lower(
            elevation.requester(),
            RelationalAuthorizationTraversalDirection::Forward,
        )?,
        approver_reverse: lower(
            elevation.approver(),
            RelationalAuthorizationTraversalDirection::Reverse,
        )?,
        approver_forward: lower(
            elevation.approver(),
            RelationalAuthorizationTraversalDirection::Forward,
        )?,
        grant: lower(
            elevation.grant(),
            RelationalAuthorizationTraversalDirection::Forward,
        )?,
        resource: lower(
            contract.target().resource(),
            RelationalAuthorizationTraversalDirection::Forward,
        )?,
    })
}

fn required_path(
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

fn push_rule(
    effect: BridgeAuthorizationRuleEffect,
    path_index: usize,
    paths: &[WorthQueryCapabilityPathTemplate],
    rules: &mut Vec<BridgeAuthorizationRuleContract>,
    rule_path_indices: &mut Vec<Vec<Vec<usize>>>,
) {
    let requirements = vec![vec![path_index]];
    rules.push(bridge_rule(effect, requirements.clone(), paths));
    rule_path_indices.push(requirements);
}
