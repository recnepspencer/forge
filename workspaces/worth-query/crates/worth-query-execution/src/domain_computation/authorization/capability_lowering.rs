use std::sync::Arc;

use worth_query_declaration::facade::application_capability::{
    ApplicationCapabilityFieldBinding, ApplicationCapabilityGraphRule,
    ApplicationCapabilityScopeGuard, ErasedApplicationCapabilityContract,
};
use worth_query_declaration::facade::application_schema::{
    ApplicationAuthorizationPath, ApplicationAuthorizationTraversalDirection,
};
use worth_query_installation::facade::{
    ApplicationSchema, WorthQueryInstalledApplicationCapabilityPlanSource,
    WorthQueryInstalledApplicationSchema,
};
use worth_relational::facade::authorization::{
    RelationalAuthorizationFieldComparison, RelationalAuthorizationFieldConstraint,
    RelationalAuthorizationPathPlan, RelationalAuthorizationTraversalDirection,
};
use worth_runtime_bridge::facade::{
    BridgeAuthorizationClauseContract, BridgeAuthorizationInstallationRequest,
    BridgeAuthorizationRequirementContract, BridgeAuthorizationRuleContract,
    BridgeAuthorizationRuleEffect, BridgeAuthorizationRuntime,
};

use super::capability_binding_lowering::{
    capability_principal, clause_identity, kind, lower_context_anchors, operand, predicate,
    relation, request_bindings,
};
use super::capability_registry::{
    field_binding, WorthQueryCapabilityContextAnchor, WorthQueryCapabilityGrantWitnessBinding,
    WorthQueryCapabilityPathTemplate, WorthQueryCapabilityRequestGuard,
    WorthQueryCapabilityRequestValueAxis, WorthQueryCapabilityUpperBoundBindings,
    WorthQueryInstalledCapabilityPlan,
};
use super::capability_registry::{
    WorthQueryCapabilityDecisionRuleBindings, WorthQueryCapabilityDelegationBindings,
};
use super::lowering::lower_authorization_path;
use super::{authorization_denial, WorthQueryOperationAuthorizationDenial};
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphLayout;

mod elevation;

pub(super) fn compile_capability_plan<Schema>(
    schema: &WorthQueryInstalledApplicationSchema<Schema>,
    source: WorthQueryInstalledApplicationCapabilityPlanSource<'_>,
    layout: &WorthQueryPrimaryGraphLayout,
    bridge: &mut BridgeAuthorizationRuntime,
) -> Result<WorthQueryInstalledCapabilityPlan, WorthQueryOperationAuthorizationDenial>
where
    Schema: ApplicationSchema,
{
    let contract = source.contract();
    let principal = capability_principal(contract)?;
    let principal_kind = kind(layout, principal)?;
    let grant_kind = kind(layout, contract.grant_entity())?;
    let scope_entity = contract.target().resource().to();
    let scope_kind = kind(layout, scope_entity)?;
    let grant_join_index_id = layout
        .capability_grant_join_index_id(
            contract.delegation().grantee().relation(),
            contract.target().resource().relation(),
        )
        .ok_or_else(|| {
            authorization_denial(contract.name(), "capability grant join is not installed")
        })?;
    let mut paths = Vec::new();
    let grant_path_index = paths.len();
    paths.push(compile_grant_path(
        source.identity().bytes(),
        contract,
        layout,
        principal_kind,
        grant_kind,
        scope_kind,
    )?);
    let grant_witness = WorthQueryCapabilityGrantWitnessBinding::new(grant_path_index, 1);
    let mut rules = vec![bridge_rule(
        BridgeAuthorizationRuleEffect::Required,
        vec![vec![0]],
        &paths,
    )];
    let mut rule_path_indices = vec![vec![vec![0]]];
    let decision_rules = compile_composition_rules(
        contract,
        layout,
        source.identity().bytes(),
        &mut paths,
        &mut rules,
        &mut rule_path_indices,
    )?;
    let upper_bound_path_count = paths.len();
    let upper_bound_rules = rules.clone();
    let upper_bound_rule_path_indices = rule_path_indices.clone();
    let elevation = elevation::compile_elevation_rules(
        contract,
        layout,
        source.identity().bytes(),
        principal_kind,
        grant_kind,
        scope_kind,
        &mut paths,
        &mut rules,
        &mut rule_path_indices,
    )?;
    let upper_bound = if elevation.is_some() {
        let identity = source.elevation_upper_bound_identity();
        let correspondence = bridge
            .install(BridgeAuthorizationInstallationRequest::new(
                &identity,
                super::bridge_authorization_binding_identity(&schema.binding_identity()),
                format!("{}:elevation-upper-bound", contract.name()),
                scope_entity,
                format!("{}:elevation-upper-bound", contract.operation()),
                upper_bound_rules.iter().cloned(),
            ))
            .map_err(|denial| {
                authorization_denial(denial.subject(), "Bridge rejected elevation upper bound")
            })?;
        Some(WorthQueryCapabilityUpperBoundBindings {
            correspondence,
            path_count: upper_bound_path_count,
            bridge_rules: upper_bound_rules,
            rule_path_indices: upper_bound_rule_path_indices,
            decision_rules: decision_rules.clone(),
        })
    } else {
        None
    };
    let correspondence = bridge
        .install(BridgeAuthorizationInstallationRequest::new(
            source.identity().digest(),
            super::bridge_authorization_binding_identity(&schema.binding_identity()),
            contract.name(),
            scope_entity,
            contract.operation(),
            rules.iter().cloned(),
        ))
        .map_err(|denial| authorization_denial(denial.subject(), "Bridge rejected capability"))?;
    Ok(WorthQueryInstalledCapabilityPlan {
        correspondence,
        capability_authority_identity: Arc::from(source.authority_identity()),
        contract: contract.clone(),
        principal_kind,
        grant_kind,
        scope_kind,
        grant_join_index_id,
        grant_witness,
        request: request_bindings(contract, layout)?,
        delegation: WorthQueryCapabilityDelegationBindings::compile(contract, layout)?,
        elevation,
        upper_bound,
        paths,
        bridge_rules: rules,
        rule_path_indices,
        decision_rules,
    })
}

fn compile_grant_path(
    capability: &[u8; 32],
    contract: &ErasedApplicationCapabilityContract,
    layout: &WorthQueryPrimaryGraphLayout,
    principal_kind: worth_relational::facade::identity::KindId,
    grant_kind: worth_relational::facade::identity::KindId,
    scope_kind: worth_relational::facade::identity::KindId,
) -> Result<WorthQueryCapabilityPathTemplate, WorthQueryOperationAuthorizationDenial> {
    let grantee = relation(
        layout,
        contract.delegation().grantee(),
        RelationalAuthorizationTraversalDirection::Forward,
    )?;
    if grantee.from_kind() != principal_kind || grantee.to_kind() != grant_kind {
        return Err(authorization_denial(
            contract.name(),
            "capability grantee relation endpoints changed",
        ));
    }
    let resource = relation(
        layout,
        contract.target().resource(),
        RelationalAuthorizationTraversalDirection::Forward,
    )?;
    if resource.from_kind() != grant_kind || resource.to_kind() != scope_kind {
        return Err(authorization_denial(
            contract.name(),
            "capability resource relation endpoints changed",
        ));
    }
    let currentness = contract.constraints().currentness();
    let predicates = vec![
        predicate(layout, 1, grant_kind, contract.target().action())?,
        predicate(layout, 1, grant_kind, contract.target().purpose())?,
        predicate(layout, 1, grant_kind, currentness.active_status())?,
    ];
    let workflow = currentness.workflow();
    let workflow_constraint = RelationalAuthorizationFieldConstraint::new(
        operand(layout, 1, grant_kind, workflow.grant())?,
        RelationalAuthorizationFieldComparison::Equal,
        operand(layout, 2, scope_kind, workflow.resource())?,
    );
    Ok(WorthQueryCapabilityPathTemplate {
        plan: RelationalAuthorizationPathPlan::new([grantee, resource], predicates)
            .with_field_constraints([workflow_constraint]),
        identity: clause_identity(capability, 0),
        guard: WorthQueryCapabilityRequestGuard::Unconditional,
        grant_ordinal: Some(1),
        elevation_ordinals: Vec::new(),
        elevation_resource_ordinal: None,
        context_anchors: Vec::new(),
    })
}

fn compile_composition_rules(
    contract: &ErasedApplicationCapabilityContract,
    layout: &WorthQueryPrimaryGraphLayout,
    capability: &[u8; 32],
    paths: &mut Vec<WorthQueryCapabilityPathTemplate>,
    rules: &mut Vec<BridgeAuthorizationRuleContract>,
    rule_path_indices: &mut Vec<Vec<Vec<usize>>>,
) -> Result<WorthQueryCapabilityDecisionRuleBindings, WorthQueryOperationAuthorizationDenial> {
    let composition = contract.composition();
    let allow = compile_graph_rule(
        contract,
        layout,
        capability,
        BridgeAuthorizationRuleEffect::Required,
        composition.decision().allow().graph(),
        paths,
        rules,
        rule_path_indices,
    )?;
    let mut compile_optional = |graph: Option<&ApplicationCapabilityGraphRule>| {
        graph
            .map(|graph| {
                compile_graph_rule(
                    contract,
                    layout,
                    capability,
                    BridgeAuthorizationRuleEffect::Prohibited,
                    graph,
                    paths,
                    rules,
                    rule_path_indices,
                )
            })
            .transpose()
    };
    let deny = compile_optional(composition.decision().deny().graph())?;
    let conflict = compile_optional(composition.decision().conflict().graph())?;
    let separation_of_duty = compile_optional(composition.actors().separation_of_duty().graph())?;
    let distinct_actor = compile_optional(composition.actors().distinct_actor().graph())?;
    Ok(WorthQueryCapabilityDecisionRuleBindings {
        grant: 0,
        allow,
        deny,
        conflict,
        separation_of_duty,
        distinct_actor,
    })
}

#[allow(clippy::too_many_arguments)]
fn compile_graph_rule(
    contract: &ErasedApplicationCapabilityContract,
    layout: &WorthQueryPrimaryGraphLayout,
    capability: &[u8; 32],
    effect: BridgeAuthorizationRuleEffect,
    graph: &ApplicationCapabilityGraphRule,
    paths: &mut Vec<WorthQueryCapabilityPathTemplate>,
    rules: &mut Vec<BridgeAuthorizationRuleContract>,
    rule_path_indices: &mut Vec<Vec<Vec<usize>>>,
) -> Result<usize, WorthQueryOperationAuthorizationDenial> {
    let mut requirements = Vec::with_capacity(graph.requirements().len());
    for requirement in graph.requirements() {
        let mut indices = Vec::with_capacity(requirement.clauses().len());
        for clause in requirement.clauses() {
            let path_index = paths.len();
            let plan = lower_authorization_path(layout, clause.path())?;
            let guard = lower_guard(contract, clause.guard())?;
            let anchors = lower_context_anchors(contract, layout, clause)?;
            let grant_ordinal = command_grant_ordinal(contract, clause.path(), &anchors)?;
            paths.push(WorthQueryCapabilityPathTemplate {
                plan,
                identity: clause_identity(capability, path_index),
                guard,
                grant_ordinal,
                elevation_ordinals: Vec::new(),
                elevation_resource_ordinal: None,
                context_anchors: anchors,
            });
            indices.push(path_index);
        }
        requirements.push(indices);
    }
    let rule_index = rules.len();
    rules.push(bridge_rule(effect, requirements.clone(), paths));
    rule_path_indices.push(requirements);
    Ok(rule_index)
}

fn grant_ordinal(
    contract: &ErasedApplicationCapabilityContract,
    path: &ApplicationAuthorizationPath,
) -> Result<Option<usize>, WorthQueryOperationAuthorizationDenial> {
    let grant = contract.grant_entity();
    let mut found = (path.principal_entity() == grant).then_some(0);
    for (index, traversal) in path.traversals().iter().enumerate() {
        let entity = match traversal.direction() {
            ApplicationAuthorizationTraversalDirection::Forward => traversal.to(),
            ApplicationAuthorizationTraversalDirection::Reverse => traversal.from(),
        };
        if entity == grant && found.replace(index + 1).is_some() {
            return Err(authorization_denial(
                contract.name(),
                "capability policy path traverses the grant kind more than once",
            ));
        }
    }
    Ok(found)
}

fn command_grant_ordinal(
    contract: &ErasedApplicationCapabilityContract,
    path: &ApplicationAuthorizationPath,
    context: &[WorthQueryCapabilityContextAnchor],
) -> Result<Option<usize>, WorthQueryOperationAuthorizationDenial> {
    let grant = grant_ordinal(contract, path)?;
    Ok(grant.filter(|grant| !context.iter().any(|anchor| anchor.ordinal < *grant)))
}

pub(super) fn bridge_rule(
    effect: BridgeAuthorizationRuleEffect,
    requirements: Vec<Vec<usize>>,
    paths: &[WorthQueryCapabilityPathTemplate],
) -> BridgeAuthorizationRuleContract {
    BridgeAuthorizationRuleContract::all(
        effect,
        requirements.into_iter().map(|indices| {
            BridgeAuthorizationRequirementContract::any(
                indices
                    .into_iter()
                    .map(|index| BridgeAuthorizationClauseContract::new(paths[index].identity)),
            )
        }),
    )
}

fn lower_guard(
    contract: &ErasedApplicationCapabilityContract,
    guard: &ApplicationCapabilityScopeGuard,
) -> Result<WorthQueryCapabilityRequestGuard, WorthQueryOperationAuthorizationDenial> {
    let [] = guard.requirements() else {
        if guard.requirements().len() != 1 {
            return Err(authorization_denial(
                contract.name(),
                "capability clause guard must bind one request axis",
            ));
        }
        let requirement = &guard.requirements()[0];
        return Ok(WorthQueryCapabilityRequestGuard::Accepted {
            axis: guard_axis(contract, requirement.field())?,
            values: requirement.values().to_vec(),
        });
    };
    Ok(WorthQueryCapabilityRequestGuard::Unconditional)
}

fn guard_axis(
    contract: &ErasedApplicationCapabilityContract,
    field: &ApplicationCapabilityFieldBinding,
) -> Result<WorthQueryCapabilityRequestValueAxis, WorthQueryOperationAuthorizationDenial> {
    if field == contract.target().action().field() {
        return Ok(WorthQueryCapabilityRequestValueAxis::Action);
    }
    if field == contract.target().purpose().field() {
        return Ok(WorthQueryCapabilityRequestValueAxis::Purpose);
    }
    if field_binding(contract.target().field()) == Some(field) {
        return Ok(WorthQueryCapabilityRequestValueAxis::Field);
    }
    if field_binding(contract.constraints().magnitude()) == Some(field) {
        return Ok(WorthQueryCapabilityRequestValueAxis::Magnitude);
    }
    Err(authorization_denial(
        field.field(),
        "capability guard is not bound to a request axis",
    ))
}
