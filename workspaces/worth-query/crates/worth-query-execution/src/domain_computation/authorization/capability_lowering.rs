use std::sync::Arc;

use worth_query_declaration::facade::application_capability::{
    ApplicationCapabilityFieldBinding, ApplicationCapabilityGraphRule,
    ApplicationCapabilityScopeGuard, ErasedApplicationCapabilityContract,
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
use super::capability_registry::WorthQueryCapabilityDelegationBindings;
use super::capability_registry::{
    field_binding, WorthQueryCapabilityGrantWitnessBinding, WorthQueryCapabilityPathTemplate,
    WorthQueryCapabilityRequestGuard, WorthQueryCapabilityRequestValueAxis,
    WorthQueryInstalledCapabilityPlan,
};
use super::lowering::lower_authorization_path;
use super::{authorization_denial, WorthQueryOperationAuthorizationDenial};
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphLayout;

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
    compile_composition_rules(
        contract,
        layout,
        source.identity().bytes(),
        &mut paths,
        &mut rules,
        &mut rule_path_indices,
    )?;
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
        grant_witness,
        request: request_bindings(contract, layout)?,
        delegation: WorthQueryCapabilityDelegationBindings::compile(contract, layout)?,
        paths,
        bridge_rules: rules,
        rule_path_indices,
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
) -> Result<(), WorthQueryOperationAuthorizationDenial> {
    let composition = contract.composition();
    compile_graph_rule(
        contract,
        layout,
        capability,
        BridgeAuthorizationRuleEffect::Required,
        composition.decision().allow().graph(),
        paths,
        rules,
        rule_path_indices,
    )?;
    for graph in [
        composition.decision().deny().graph(),
        composition.decision().conflict().graph(),
        composition.actors().separation_of_duty().graph(),
        composition.actors().distinct_actor().graph(),
    ]
    .into_iter()
    .flatten()
    {
        compile_graph_rule(
            contract,
            layout,
            capability,
            BridgeAuthorizationRuleEffect::Prohibited,
            graph,
            paths,
            rules,
            rule_path_indices,
        )?;
    }
    Ok(())
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
) -> Result<(), WorthQueryOperationAuthorizationDenial> {
    let mut requirements = Vec::with_capacity(graph.requirements().len());
    for requirement in graph.requirements() {
        let mut indices = Vec::with_capacity(requirement.clauses().len());
        for clause in requirement.clauses() {
            let path_index = paths.len();
            let plan = lower_authorization_path(layout, clause.path())?;
            let guard = lower_guard(contract, clause.guard())?;
            let anchors = lower_context_anchors(contract, layout, clause)?;
            paths.push(WorthQueryCapabilityPathTemplate {
                plan,
                identity: clause_identity(capability, path_index),
                guard,
                context_anchors: anchors,
            });
            indices.push(path_index);
        }
        requirements.push(indices);
    }
    rules.push(bridge_rule(effect, requirements.clone(), paths));
    rule_path_indices.push(requirements);
    Ok(())
}

fn bridge_rule(
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
    if field_binding(contract.constraints().amount()) == Some(field) {
        return Ok(WorthQueryCapabilityRequestValueAxis::Amount);
    }
    Err(authorization_denial(
        field.field(),
        "capability guard is not bound to a request axis",
    ))
}
