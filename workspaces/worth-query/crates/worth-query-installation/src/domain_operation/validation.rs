use std::collections::BTreeSet;

use super::*;

mod workflow;

use super::conditional_node::validate_conditional_nodes;
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
    validate_collection(&semantics.collection)?;
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
    validate_conditional_nodes(&semantics.conditional_nodes)?;
    validate_conditional_graph_closure(semantics)?;
    validate_workflow_closure(semantics)?;
    validate_graph_reads(&semantics.graph_reads)?;
    validate_touches(&semantics.touches)?;
    validate_touch_graph_roles(&semantics.touches, &semantics.graph_reads)?;
    validate_effects(&semantics.effects)?;
    validate_invariants(&semantics.invariants)?;
    validate_reversal(&semantics.reversal)?;
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

fn validate_conditional_graph_closure(
    semantics: &WorthQueryDomainOperationSemanticClosure,
) -> Result<(), &'static str> {
    for node in &semantics.conditional_nodes {
        if node.role() == WorthQueryConditionalNodeRole::WorkflowStage {
            return Err("workflow-stage-conditional-attached-at-operation");
        }
        validate_node_graph_closure(node, semantics, None)?;
    }
    if let WorthQueryOperationWorkflowContract::Declared(workflow) = &semantics.workflow {
        for stage in workflow.stages() {
            for node in &stage.semantics().conditional_nodes {
                if node.role() != WorthQueryConditionalNodeRole::WorkflowStage {
                    return Err("operation-conditional-attached-at-workflow-stage");
                }
                validate_node_graph_closure(node, semantics, Some(stage.semantics()))?;
            }
        }
    }
    Ok(())
}

fn validate_node_graph_closure(
    node: &WorthQueryPortableConditionalNodeDeclaration,
    semantics: &WorthQueryDomainOperationSemanticClosure,
    stage_semantics: Option<&WorthQueryWorkflowStageSemantics>,
) -> Result<(), &'static str> {
    for dependency in node.dependencies() {
        let role_name = dependency.graph_read_role().as_str();
        if stage_semantics
            .is_some_and(|stage| !stage.graph_read_roles.iter().any(|role| role == role_name))
        {
            return Err("conditional-dependency-outside-stage-graph-read-role");
        }
        let Some(role) = semantics
            .graph_reads
            .roles()
            .iter()
            .find(|role| role.role == role_name)
        else {
            return Err("conditional-dependency-uses-undeclared-graph-read-role");
        };
        if !role
            .semantic_reads
            .iter()
            .any(|read| projection_admits_dependency(read, dependency))
        {
            return Err("conditional-dependency-exceeds-graph-read-scope");
        }
    }
    for output in node.outputs() {
        match output {
            WorthQueryConditionalNodeOutput::OperationOutput { projection_role } => {
                if !matches!(
                    &semantics.publication,
                    WorthQueryOperationPublicationContract::DerivedProjection { projection_role: installed }
                        if installed == projection_role
                ) {
                    return Err("conditional-output-uses-undeclared-operation-output-role");
                }
            }
            WorthQueryConditionalNodeOutput::WorkflowStageOutput { contract } => {
                if *contract == WorthQueryWorkflowValueContract::NotRequired
                    || stage_semantics.is_none_or(|stage| stage.output != *contract)
                {
                    return Err("conditional-output-uses-undeclared-workflow-output-role");
                }
            }
            WorthQueryConditionalNodeOutput::DerivedAspect { .. } => {}
        }
        let WorthQueryConditionalNodeOutput::DerivedAspect { consequences, .. } = output else {
            continue;
        };
        for consequence in consequences {
            match consequence {
                WorthQueryConditionalConsequenceRole::DerivedOnly => {}
                WorthQueryConditionalConsequenceRole::Touch(touch) => {
                    let WorthQueryOperationTouchContract::Declared {
                        graph_roles,
                        scopes,
                    } = &semantics.touches
                    else {
                        return Err("conditional-output-uses-undeclared-touch-role");
                    };
                    if !graph_roles.iter().any(|role| role == touch.graph_role())
                        || !scopes.iter().any(|scope| scope == touch.scope())
                    {
                        return Err("conditional-output-uses-undeclared-touch-role");
                    }
                }
                WorthQueryConditionalConsequenceRole::Effect(family) => {
                    let WorthQueryOperationEffectContract::Declared { effect_families } =
                        &semantics.effects
                    else {
                        return Err("conditional-output-uses-undeclared-effect-role");
                    };
                    if !effect_families.contains(family) {
                        return Err("conditional-output-uses-undeclared-effect-role");
                    }
                }
            }
        }
    }
    Ok(())
}

fn projection_admits_dependency(
    read: &WorthQueryOperationNativeProjectionContract,
    dependency: &WorthQuerySemanticTruthDependency,
) -> bool {
    read.aspect_key == *dependency.contract().key()
        && read.aspect_identity == dependency.contract().identity()
        && read.contract_revision == dependency.contract().revision()
        && projection_mask_contains(&read.mask, dependency.projection_mask())
}

fn projection_mask_contains(
    admitted: &worth_foundational::facade::AspectMask<worth_foundational::facade::ProjectionMask>,
    requested: &worth_foundational::facade::AspectMask<worth_foundational::facade::ProjectionMask>,
) -> bool {
    admitted.is_whole_aspect()
        || (!requested.is_whole_aspect()
            && requested
                .paths()
                .iter()
                .all(|path| admitted.paths().contains(path)))
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

fn validate_collection(
    contract: &WorthQueryOperationCollectionContract,
) -> Result<(), &'static str> {
    let WorthQueryOperationCollectionContract::Collection {
        row_identity_field,
        ordering_fields,
        ..
    } = contract
    else {
        return Ok(());
    };
    if row_identity_field.trim().is_empty() {
        return Err("empty-row-identity-field");
    }
    if ordering_fields.is_empty() {
        return Err("empty-collection-ordering");
    }
    validate_text_sequence(ordering_fields, "empty-ordering-field")
}

fn validate_graph_reads(
    contract: &WorthQueryOperationGraphReadContract,
) -> Result<(), &'static str> {
    let WorthQueryOperationGraphReadContract::Declared { roles } = contract else {
        return Ok(());
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
        .any(|role| !reads.roles().iter().any(|read| &read.role == role))
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
        validate_text_sequence(scopes, "empty-touch-scope")?;
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
    }
    Ok(())
}

fn validate_reversal(contract: &WorthQueryOperationReversalContract) -> Result<(), &'static str> {
    let subject = match contract {
        WorthQueryOperationReversalContract::ExactInverse { lowering_family } => lowering_family,
        WorthQueryOperationReversalContract::Compensation { operation } => operation,
        WorthQueryOperationReversalContract::RebuildRequired { recovery_family } => recovery_family,
        WorthQueryOperationReversalContract::Irreversible
        | WorthQueryOperationReversalContract::ProvisionalDiscard => return Ok(()),
    };
    if subject.trim().is_empty() {
        return Err("empty-reversal-subject");
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
