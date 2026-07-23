use super::super::{
    WorthQueryOperationWorkflowContract, WorthQueryPortableWorkflowStage,
    WorthQueryWorkflowStageSemantics,
};
use super::evidence::{MismatchEvidence, WorthQueryPortableOperationComparisonWork};
use super::WorthQueryPortableOperationDimension as Dimension;

/// Compares workflow meaning that is not owned by conditional comparison.
pub(super) fn compare_workflow_structure(
    left: &WorthQueryOperationWorkflowContract,
    right: &WorthQueryOperationWorkflowContract,
    work: &mut WorthQueryPortableOperationComparisonWork,
) -> Result<(), MismatchEvidence> {
    work.inspect_owner_dimension();
    match (left, right) {
        (
            WorthQueryOperationWorkflowContract::NotRequired,
            WorthQueryOperationWorkflowContract::NotRequired,
        ) => Ok(()),
        (
            WorthQueryOperationWorkflowContract::Declared(left),
            WorthQueryOperationWorkflowContract::Declared(right),
        ) => {
            require_equal(left.entry_stage(), right.entry_stage(), work)?;
            work.submit_variable_items(left.stages().len() + right.stages().len());
            require_equal(&left.stages().len(), &right.stages().len(), work)?;
            for (left, right) in left.stages().iter().zip(right.stages()) {
                compare_stage(left, right, work)?;
            }
            Ok(())
        }
        _ => Err(MismatchEvidence::installation_owner(Dimension::Workflow)),
    }
}

fn compare_stage(
    left: &WorthQueryPortableWorkflowStage,
    right: &WorthQueryPortableWorkflowStage,
    work: &mut WorthQueryPortableOperationComparisonWork,
) -> Result<(), MismatchEvidence> {
    require_equal(left.identity(), right.identity(), work)?;
    work.submit_variable_items(left.predecessors().len() + right.predecessors().len());
    require_equal(left.predecessors(), right.predecessors(), work)?;
    require_equal(&left.is_terminal(), &right.is_terminal(), work)?;
    require_equal(&left.is_publishable(), &right.is_publishable(), work)?;
    work.submit_variable_items(
        left.required_capabilities().len() + right.required_capabilities().len(),
    );
    require_equal(
        left.required_capabilities(),
        right.required_capabilities(),
        work,
    )?;
    compare_stage_semantics(left.semantics(), right.semantics(), work)
}

fn compare_stage_semantics(
    left: &WorthQueryWorkflowStageSemantics,
    right: &WorthQueryWorkflowStageSemantics,
    work: &mut WorthQueryPortableOperationComparisonWork,
) -> Result<(), MismatchEvidence> {
    require_equal(&left.input, &right.input, work)?;
    require_equal(&left.output, &right.output, work)?;
    work.submit_variable_items(
        left.required_domain_roles.len() + right.required_domain_roles.len(),
    );
    require_equal(
        &left.required_domain_roles,
        &right.required_domain_roles,
        work,
    )?;
    work.submit_variable_items(left.graph_read_roles.len() + right.graph_read_roles.len());
    require_equal(&left.graph_read_roles, &right.graph_read_roles, work)?;
    work.submit_variable_items(left.touch_roles.len() + right.touch_roles.len());
    require_equal(&left.touch_roles, &right.touch_roles, work)?;
    work.submit_variable_items(left.effect_roles.len() + right.effect_roles.len());
    require_equal(&left.effect_roles, &right.effect_roles, work)?;
    work.submit_variable_items(left.invariant_roles.len() + right.invariant_roles.len());
    require_equal(&left.invariant_roles, &right.invariant_roles, work)?;
    work.submit_variable_items(left.cost_roles.len() + right.cost_roles.len());
    require_equal(&left.cost_roles, &right.cost_roles, work)?;
    work.submit_variable_items(
        left.terminal_result_states.len() + right.terminal_result_states.len(),
    );
    require_equal(
        &left.terminal_result_states,
        &right.terminal_result_states,
        work,
    )?;
    work.submit_variable_items(left.failure_classes.len() + right.failure_classes.len());
    require_equal(&left.failure_classes, &right.failure_classes, work)
}

fn require_equal<T: PartialEq + ?Sized>(
    left: &T,
    right: &T,
    work: &mut WorthQueryPortableOperationComparisonWork,
) -> Result<(), MismatchEvidence> {
    work.inspect_owner_dimension();
    if left == right {
        Ok(())
    } else {
        Err(MismatchEvidence::installation_owner(Dimension::Workflow))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain_operation::{
        WorthQueryArtifactPosture, WorthQueryArtifactReuseEquivalence,
        WorthQueryConditionalEvaluationCondition, WorthQueryConditionalNodeContext,
        WorthQueryConditionalNodeOutput, WorthQueryConditionalNodeRole,
        WorthQueryConditionalTrigger, WorthQueryMaintenancePosture,
        WorthQueryOnDemandTriggerFamily, WorthQueryOutputEquivalenceRequirement,
        WorthQueryOutputRelationship, WorthQueryPortableConditionalNodeDeclaration,
        WorthQueryPortableWorkflowDefinition, WorthQueryWorkflowValueContract,
    };

    struct Trigger;

    impl WorthQueryOnDemandTriggerFamily for Trigger {
        const PORTABLE_IDENTITY: &'static str = "test.workflow-structure.trigger";
    }

    #[test]
    fn canonical_conditional_order_is_not_rejudged_by_workflow_structure() {
        let alpha = workflow_node("alpha");
        let zeta = workflow_node("zeta");
        let mut canonical_left = vec![zeta.clone(), alpha.clone()];
        let mut canonical_right = vec![alpha.clone(), zeta.clone()];
        crate::domain_operation::conditional_node::canonicalize_conditional_nodes(
            &mut canonical_left,
        );
        crate::domain_operation::conditional_node::canonicalize_conditional_nodes(
            &mut canonical_right,
        );
        assert_eq!(canonical_left, canonical_right);

        let left = workflow(vec![zeta, alpha]);
        let right = workflow(canonical_right);
        assert_ne!(left, right);
        let mut work = WorthQueryPortableOperationComparisonWork::default();
        assert!(compare_workflow_structure(&left, &right, &mut work).is_ok());
        assert_eq!(work.owner_dimensions_inspected(), 18);
    }

    fn workflow(
        nodes: Vec<WorthQueryPortableConditionalNodeDeclaration>,
    ) -> WorthQueryOperationWorkflowContract {
        let stage = WorthQueryPortableWorkflowStage::new(
            "stage",
            std::iter::empty::<&str>(),
            true,
            true,
            [],
        )
        .with_semantics(WorthQueryWorkflowStageSemantics {
            output: WorthQueryWorkflowValueContract::Bool,
            conditional_nodes: nodes,
            ..WorthQueryWorkflowStageSemantics::default()
        });
        WorthQueryOperationWorkflowContract::Declared(WorthQueryPortableWorkflowDefinition::new(
            "stage",
            [stage],
        ))
    }

    fn workflow_node(identity: &str) -> WorthQueryPortableConditionalNodeDeclaration {
        WorthQueryPortableConditionalNodeDeclaration::declare(
            identity,
            WorthQueryConditionalNodeRole::WorkflowStage,
        )
        .dependencies([])
        .outputs([WorthQueryConditionalNodeOutput::WorkflowStageOutput {
            contract: WorthQueryWorkflowValueContract::Bool,
        }])
        .required_context([WorthQueryConditionalNodeContext::WorkflowRun])
        .evaluation(
            WorthQueryConditionalEvaluationCondition::on_demand(),
            WorthQueryConditionalTrigger::on_demand::<Trigger>(),
        )
        .comparison(
            crate::domain_operation::WorthQueryComparatorRequirement::ExactCanonicalValue,
            WorthQueryOutputEquivalenceRequirement::ExactCanonicalValue,
        )
        .artifact_policy(
            WorthQueryArtifactReuseEquivalence::NotReusable,
            WorthQueryMaintenancePosture::OnDemandOnly,
            WorthQueryArtifactPosture::Ephemeral,
        )
        .output_relationship(WorthQueryOutputRelationship::IsWorkflowStageOutput)
        .finish()
        .unwrap()
    }
}
