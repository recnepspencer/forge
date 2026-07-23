use super::super::*;

pub(super) fn validate_conditional_graph_closure(
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
    validate_conditional_outputs(node, semantics, stage_semantics)
}

fn validate_conditional_outputs(
    node: &WorthQueryPortableConditionalNodeDeclaration,
    semantics: &WorthQueryDomainOperationSemanticClosure,
    stage_semantics: Option<&WorthQueryWorkflowStageSemantics>,
) -> Result<(), &'static str> {
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
        validate_conditional_consequences(output, semantics)?;
    }
    Ok(())
}

fn validate_conditional_consequences(
    output: &WorthQueryConditionalNodeOutput,
    semantics: &WorthQueryDomainOperationSemanticClosure,
) -> Result<(), &'static str> {
    let WorthQueryConditionalNodeOutput::DerivedAspect { consequences, .. } = output else {
        return Ok(());
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
    Ok(())
}

fn projection_admits_dependency(
    read: &WorthQueryOperationNativeProjectionContract,
    dependency: &WorthQuerySemanticTruthDependency,
) -> bool {
    read.contract() == dependency.contract()
        && projection_mask_contains(read.mask(), dependency.projection_mask())
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
