use std::collections::BTreeSet;

use super::*;

mod collection;
mod conditional_graph_closure;
#[cfg(test)]
mod tests;
mod workflow;

use super::conditional_node::validate_conditional_nodes;
use collection::validate_collection;
use conditional_graph_closure::validate_conditional_graph_closure;
use workflow::{validate_workflow, validate_workflow_closure};

pub(super) fn validate_domain_operation_meaning(
    operation: &WorthQueryPortableDomainOperationDefinition,
) -> Result<(), &'static str> {
    if operation.identity().name().trim().is_empty() {
        return Err("empty-operation-name");
    }
    if operation.identity().version() == 0 {
        return Err("zero-operation-version");
    }
    let semantics = operation.semantics();
    semantics
        .canonical_query
        .check_invariants()
        .map_err(|_| "invalid-canonical-query-bundle")?;
    validate_parameters(&semantics.parameters)?;
    validate_collection(&semantics.collection, &semantics.canonical_query)?;
    if semantics
        .required_domains
        .iter()
        .enumerate()
        .any(|(index, role)| {
            semantics.required_domains[..index].contains(role) || role.as_str().trim().is_empty()
        })
    {
        return Err("duplicate-or-empty-required-domain-role");
    }
    validate_workflow(&semantics.workflow)?;
    semantics.resources.validate()?;
    validate_conditional_nodes(&semantics.conditional_nodes)?;
    validate_conditional_graph_closure(semantics)?;
    validate_workflow_closure(semantics)?;
    validate_graph_reads(&semantics.graph_reads)?;
    validate_touches(&semantics.touches)?;
    validate_touch_graph_roles(&semantics.touches, &semantics.graph_reads)?;
    validate_effects(&semantics.effects)?;
    validate_invariants(&semantics.invariants)?;
    validate_invariant_execution(
        &semantics.invariants,
        &semantics.invariant_execution,
        &semantics.touches,
    )?;
    validate_aftermath(semantics.aftermath.as_ref())?;
    validate_publication(&semantics.publication)?;
    validate_projection_consumption(&semantics.publication, semantics.projection_consumption)?;
    if semantics.terminal.result_states.is_empty() {
        return Err("empty-terminal-result-state-set");
    }
    if semantics.lowering.family.trim().is_empty() {
        return Err("empty-lowering-family");
    }
    Ok(())
}

fn validate_parameters(
    contract: &WorthQueryOperationParameterContract,
) -> Result<(), &'static str> {
    let WorthQueryOperationParameterContract::Declared { fields } = contract else {
        return Ok(());
    };
    let mut names = BTreeSet::new();
    for field in fields {
        if field.name.trim().is_empty() {
            return Err("empty-parameter-name");
        }
        if !names.insert(field.name.as_str()) {
            return Err("duplicate-parameter-name");
        }
    }
    Ok(())
}

fn validate_graph_reads(
    contract: &WorthQueryOperationGraphReadContract,
) -> Result<(), &'static str> {
    let roles = match contract {
        WorthQueryOperationGraphReadContract::NotRequired => return Ok(()),
        WorthQueryOperationGraphReadContract::Declared { .. } => {
            return Err("application-graph-read-contract-in-domain-operation")
        }
        WorthQueryOperationGraphReadContract::DeclaredDomain { roles } => roles,
    };
    if roles.is_empty() {
        return Err("empty-graph-read-role-set");
    }
    for (index, role) in roles.iter().enumerate() {
        if role.role.trim().is_empty() {
            return Err("empty-graph-read-role");
        }
        if let WorthQueryOperationGraphParticipation::SeparateAuthority {
            role: participation_role,
        } = &role.participation
        {
            if participation_role.trim().is_empty() {
                return Err("empty-separate-graph-role");
            }
            if participation_role != &role.role {
                return Err("separate-graph-role-mismatch");
            }
        }
        if roles[..index].iter().any(|prior| prior.role == role.role) {
            return Err("duplicate-graph-read-role");
        }
    }
    Ok(())
}

fn validate_touch_graph_roles(
    touches: &WorthQueryOperationTouchContract,
    reads: &WorthQueryOperationGraphReadContract,
) -> Result<(), &'static str> {
    let WorthQueryOperationTouchContract::Declared { graph_roles, .. } = touches else {
        return Ok(());
    };
    if graph_roles
        .iter()
        .any(|role| !reads.domain_roles().iter().any(|read| &read.role == role))
    {
        return Err("touch-references-undeclared-graph-role");
    }
    Ok(())
}

fn validate_touches(contract: &WorthQueryOperationTouchContract) -> Result<(), &'static str> {
    if let WorthQueryOperationTouchContract::Declared {
        graph_roles,
        scopes,
    } = contract
    {
        if graph_roles.is_empty() || scopes.is_empty() {
            return Err("empty-touch-contract");
        }
        validate_text_sequence(graph_roles, "empty-touch-graph-role")?;
        if scopes
            .iter()
            .any(|scope| !matches!(scope, WorthQueryOperationTouchScope::DeclaredDomain(_)))
        {
            return Err("application-touch-scope-in-domain-operation");
        }
    }
    Ok(())
}

fn validate_effects(contract: &WorthQueryOperationEffectContract) -> Result<(), &'static str> {
    if let WorthQueryOperationEffectContract::Declared { effect_families } = contract {
        if effect_families.is_empty() {
            return Err("empty-effect-family-set");
        }
        if effect_families.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err("duplicate-effect-family");
        }
    }
    Ok(())
}

fn validate_invariants(
    contract: &WorthQueryOperationInvariantContract,
) -> Result<(), &'static str> {
    if let WorthQueryOperationInvariantContract::Declared { invariant_slots } = contract {
        if invariant_slots.is_empty() {
            return Err("empty-invariant-slot-set");
        }
        validate_text_sequence(invariant_slots, "empty-invariant-slot")?;
        if invariant_slots.iter().collect::<BTreeSet<_>>().len() != invariant_slots.len() {
            return Err("duplicate-invariant-slot");
        }
    }
    Ok(())
}

fn validate_invariant_execution(
    slots: &WorthQueryOperationInvariantContract,
    execution: &WorthQueryInvariantExecutionContract,
    touches: &WorthQueryOperationTouchContract,
) -> Result<(), &'static str> {
    let (invariant_slots, requirements) = match (slots, execution) {
        (
            WorthQueryOperationInvariantContract::NotRequired,
            WorthQueryInvariantExecutionContract::NotRequired,
        ) => return Ok(()),
        (
            WorthQueryOperationInvariantContract::NotRequired,
            WorthQueryInvariantExecutionContract::Declared { .. },
        ) => return Err("invariant-execution-without-declared-slots"),
        (
            WorthQueryOperationInvariantContract::Declared { .. },
            WorthQueryInvariantExecutionContract::NotRequired,
        ) => return Err("declared-invariant-without-execution-requirement"),
        (
            WorthQueryOperationInvariantContract::Declared { invariant_slots },
            WorthQueryInvariantExecutionContract::Declared { requirements },
        ) => (invariant_slots, requirements),
    };
    let declared_slots = invariant_slots
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let executable_slots = requirements
        .iter()
        .map(WorthQueryInstalledInvariantExecutionRequirement::slot)
        .collect::<BTreeSet<_>>();
    if declared_slots != executable_slots {
        return Err("invariant-execution-slot-set-mismatch");
    }
    let touch_roles = match touches {
        WorthQueryOperationTouchContract::NotRequired => BTreeSet::new(),
        WorthQueryOperationTouchContract::Declared { graph_roles, .. } => {
            graph_roles.iter().map(String::as_str).collect()
        }
    };
    if requirements
        .iter()
        .any(|requirement| !touch_roles.contains(requirement.executor_role()))
    {
        return Err("invariant-executor-role-does-not-own-provisional-state");
    }
    Ok(())
}

fn validate_aftermath(
    contract: Option<&crate::application_aftermath::WorthQueryInstalledAftermathContract>,
) -> Result<(), &'static str> {
    let Some(contract) = contract else {
        return Ok(());
    };
    if contract.operation_slot().trim().is_empty() {
        return Err("empty-aftermath-operation");
    }
    Ok(())
}

fn validate_publication(
    contract: &WorthQueryOperationPublicationContract,
) -> Result<(), &'static str> {
    if matches!(
        contract,
        WorthQueryOperationPublicationContract::DerivedProjection { projection_role }
            if projection_role.as_str().trim().is_empty()
    ) {
        return Err("empty-publication-role");
    }
    Ok(())
}

fn validate_projection_consumption(
    publication: &WorthQueryOperationPublicationContract,
    consumption: WorthQueryOperationProjectionConsumptionContract,
) -> Result<(), &'static str> {
    match (publication, consumption) {
        (
            WorthQueryOperationPublicationContract::NotRequired,
            WorthQueryOperationProjectionConsumptionContract::NotRequired,
        )
        | (
            WorthQueryOperationPublicationContract::DerivedProjection { .. },
            WorthQueryOperationProjectionConsumptionContract::QueryReadAuthority,
        ) => Ok(()),
        _ => Err("publication-projection-consumption-contract-mismatch"),
    }
}

fn validate_text_sequence(values: &[String], denial: &'static str) -> Result<(), &'static str> {
    if values.iter().any(|value| value.trim().is_empty()) {
        return Err(denial);
    }
    Ok(())
}
