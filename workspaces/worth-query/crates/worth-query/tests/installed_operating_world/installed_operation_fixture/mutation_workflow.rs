use worth_query::facade::{domain, runtime};
use worth_relational::facade::identity::KindId;

use super::{
    canonical_bundle, configured_runtime_without_executors, semantic_closure, GeometryDomain,
};

#[derive(Clone, Copy, Debug)]
pub struct WorkflowMutation;

#[derive(Clone, Copy, Debug)]
pub struct MutationFamily;

impl domain::WorthQueryExecutableDomainOperation<GeometryDomain, MutationFamily>
    for WorkflowMutation
{
    type Input = ();
    type Output = ();
    type Publication = domain::WorthQueryTerminalOperation;
    type Execution = domain::WorthQueryWorkflowOperation;
}

#[derive(Clone, Copy)]
struct MutationWorkflowExecutor;

#[derive(Clone, Copy)]
struct MixedMutationWorkflowExecutor;

impl domain::WorthQueryDomainWorkflowStageExecutor<GeometryDomain, WorkflowMutation, MutationFamily>
    for MixedMutationWorkflowExecutor
{
    const LOWERING_FAMILY: &'static str = "workflow-mutation-v1";
    const DETERMINISTIC: bool = true;
    const EXECUTION_COST: domain::WorthQueryOperationCostClass =
        domain::WorthQueryOperationCostClass::ExternalBoundary;
    const RESULT_WIDTH_COST: domain::WorthQueryOperationCostClass =
        domain::WorthQueryOperationCostClass::DeclaredWidth;

    fn execute_stage(
        &self,
        input: domain::WorthQueryWorkflowValue,
        context: &domain::WorthQueryWorkflowStageExecutionContext<'_>,
        workspace: &mut domain::WorthQueryWorkflowStageWorkspace<'_>,
    ) -> Result<
        domain::WorthQueryWorkflowStageMaterial,
        domain::WorthQueryWorkflowStageExecutorFailure,
    > {
        domain::WorthQueryDomainWorkflowStageExecutor::execute_stage(
            &MutationWorkflowExecutor,
            input,
            context,
            workspace,
        )
    }
}

impl domain::WorthQueryDomainWorkflowStageExecutor<GeometryDomain, WorkflowMutation, MutationFamily>
    for MutationWorkflowExecutor
{
    const LOWERING_FAMILY: &'static str = "workflow-mutation-v1";
    const DETERMINISTIC: bool = true;
    const EXECUTION_COST: domain::WorthQueryOperationCostClass =
        domain::WorthQueryOperationCostClass::DeclaredWidth;
    const RESULT_WIDTH_COST: domain::WorthQueryOperationCostClass =
        domain::WorthQueryOperationCostClass::DeclaredWidth;

    fn execute_stage(
        &self,
        input: domain::WorthQueryWorkflowValue,
        context: &domain::WorthQueryWorkflowStageExecutionContext<'_>,
        workspace: &mut domain::WorthQueryWorkflowStageWorkspace<'_>,
    ) -> Result<
        domain::WorthQueryWorkflowStageMaterial,
        domain::WorthQueryWorkflowStageExecutorFailure,
    > {
        let command = runtime::WorthQueryAspectMutationBuilder::new()
            .aspect("identity.id", "workflow-mutation-entity")
            .build_insert("Vertex")
            .map_err(|detail| {
                domain::WorthQueryWorkflowStageExecutorFailure::new(
                    domain::WorthQueryOperationFailureClass::InvalidInput,
                    format!("{detail:?}"),
                )
            })?;
        context
            .execute_mutation(command, workspace)
            .map_err(|denial| {
                domain::WorthQueryWorkflowStageExecutorFailure::new(
                    domain::WorthQueryOperationFailureClass::Dependency,
                    format!("{denial:?}"),
                )
            })?;
        if matches!(input, domain::WorthQueryWorkflowValue::Text(value) if value == "fail-after-effect")
        {
            return Err(domain::WorthQueryWorkflowStageExecutorFailure::new(
                domain::WorthQueryOperationFailureClass::Dependency,
                "declared failure after mutation",
            ));
        }
        Ok(
            domain::WorthQueryWorkflowStageMaterial::new(domain::WorthQueryWorkflowValue::Text(
                "mutation-committed".into(),
            ))
            .with_result_state(domain::WorthQueryOperationResultState::Ready),
        )
    }
}

pub fn mutation_workflow_workspace(
    name: &str,
) -> Result<
    runtime::WorthQueryWorkspace,
    worth_query::facade::consumer_kit::WorthQueryTestBackendError,
> {
    let mut semantics = semantic_closure(
        canonical_bundle("Vertex"),
        domain::WorthQuerySupportRequirement::NotRequired,
        false,
    );
    semantics.graph_reads = domain::WorthQueryOperationGraphReadContract::NotRequired;
    semantics.effects = domain::WorthQueryOperationEffectContract::Declared {
        effect_families: vec![domain::WorthQueryOperationEffectFamily::Mutation],
    };
    semantics.invariants = domain::WorthQueryOperationInvariantContract::Declared {
        invariant_slots: vec!["workflow-invariant:1".into()],
    };
    semantics.lowering.family = "workflow-mutation-v1".into();
    semantics.terminal.failure_classes = vec![
        domain::WorthQueryOperationFailureClass::InvalidInput,
        domain::WorthQueryOperationFailureClass::Dependency,
    ];
    semantics.workflow = domain::WorthQueryOperationWorkflowContract::Declared(
        domain::WorthQueryPortableWorkflowDefinition::new(
            "mutate",
            [domain::WorthQueryPortableWorkflowStage::new(
                "mutate",
                std::iter::empty::<&str>(),
                true,
                false,
                std::iter::empty::<domain::WorthQueryOperationCapabilityRequirement>(),
            )
            .with_semantics(domain::WorthQueryWorkflowStageSemantics {
                input: domain::WorthQueryWorkflowValueContract::Text,
                output: domain::WorthQueryWorkflowValueContract::Text,
                effect_roles: vec![domain::WorthQueryOperationEffectFamily::Mutation],
                invariant_roles: vec!["workflow-invariant:1".into()],
                cost_roles: vec![
                    domain::WorthQueryWorkflowCostRole::Admission,
                    domain::WorthQueryWorkflowCostRole::Effect,
                    domain::WorthQueryWorkflowCostRole::Invariant,
                    domain::WorthQueryWorkflowCostRole::Execution,
                    domain::WorthQueryWorkflowCostRole::ResultValidation,
                ],
                terminal_result_states: vec![domain::WorthQueryOperationResultState::Ready],
                failure_classes: semantics.terminal.failure_classes.clone(),
                ..Default::default()
            })],
        ),
    );
    let operation = domain::WorthQueryDomainOperationDefinition::<
        GeometryDomain,
        WorkflowMutation,
        MutationFamily,
    >::new(
        domain::WorthQueryDomainOperationIdentity::new("workflow-mutation", 1),
        semantics,
    );
    let package = domain::WorthQueryDomainPackage::declare(
        GeometryDomain,
        domain::WorthQueryDomainIdentityDeclaration::new(
            domain::WorthQueryDomainIdentityNamespace::new("WORTH.tests").unwrap(),
            domain::WorthQueryDomainIdentityName::new("geometry").unwrap(),
            domain::WorthQueryDomainSemanticVersion::new(1, 0),
        ),
    )
    .invariant(domain::WorthQueryDomainInvariantDefinition::new(
        domain::WorthQueryDomainIdentityName::new("workflow-invariant").unwrap(),
        domain::WorthQueryDomainSemanticVersion::new(1, 0),
        domain::WorthQueryDomainInvariantPredicate::requires_outgoing_relations(
            vec![KindId::new(0xff00_0001)],
            vec![KindId::new(0xff00_0002)],
            1,
        ),
    ))
    .operation(operation);
    configured_runtime_without_executors(package)
        .workflow_stage_executor(
            GeometryDomain,
            WorkflowMutation,
            MutationFamily,
            MutationWorkflowExecutor,
        )
        .workspace(name)
}

pub fn mixed_mutation_workflow_runtime<G: 'static>(
    compensated: bool,
) -> worth_query::facade::consumer_kit::WorthQueryInMemoryTestRuntimeBuilder {
    let mut semantics = semantic_closure(
        canonical_bundle("Vertex"),
        domain::WorthQuerySupportRequirement::NotRequired,
        false,
    );
    semantics.graph_reads = domain::WorthQueryOperationGraphReadContract::Declared {
        roles: vec![domain::WorthQueryOperationGraphReadRole {
            role: "remote-a".into(),
            participation: domain::WorthQueryOperationGraphParticipation::SeparateAuthority {
                role: "remote-a".into(),
            },
            access: domain::WorthQueryOperationGraphAccess::Observe,
            semantic_reads: Vec::new(),
        }],
    };
    semantics.touches = domain::WorthQueryOperationTouchContract::Declared {
        graph_roles: vec!["remote-a".into()],
        scopes: vec!["vertex".into()],
    };
    semantics.effects = domain::WorthQueryOperationEffectContract::Declared {
        effect_families: vec![domain::WorthQueryOperationEffectFamily::Mutation],
    };
    semantics.invariants = domain::WorthQueryOperationInvariantContract::NotRequired;
    semantics.reversal = if compensated {
        domain::WorthQueryOperationReversalContract::Compensation {
            operation: domain::WorthQueryDomainOperationIdentity::new(
                "compensate-mixed-workflow-mutation",
                1,
            ),
        }
    } else {
        domain::WorthQueryOperationReversalContract::Irreversible
    };
    semantics.lowering.family = "workflow-mutation-v1".into();
    semantics.cost.execution = domain::WorthQueryOperationCostClass::ExternalBoundary;
    semantics.workflow = domain::WorthQueryOperationWorkflowContract::Declared(
        domain::WorthQueryPortableWorkflowDefinition::new(
            "mutate",
            [domain::WorthQueryPortableWorkflowStage::new(
                "mutate",
                std::iter::empty::<&str>(),
                true,
                false,
                std::iter::empty::<domain::WorthQueryOperationCapabilityRequirement>(),
            )
            .with_semantics(domain::WorthQueryWorkflowStageSemantics {
                input: domain::WorthQueryWorkflowValueContract::Text,
                output: domain::WorthQueryWorkflowValueContract::Text,
                graph_read_roles: vec!["remote-a".into()],
                touch_roles: vec!["remote-a".into()],
                effect_roles: vec![domain::WorthQueryOperationEffectFamily::Mutation],
                cost_roles: vec![
                    domain::WorthQueryWorkflowCostRole::Admission,
                    domain::WorthQueryWorkflowCostRole::GraphRead,
                    domain::WorthQueryWorkflowCostRole::TouchEffect,
                    domain::WorthQueryWorkflowCostRole::Effect,
                    domain::WorthQueryWorkflowCostRole::Execution,
                    domain::WorthQueryWorkflowCostRole::ResultValidation,
                ],
                terminal_result_states: vec![domain::WorthQueryOperationResultState::Ready],
                failure_classes: semantics.terminal.failure_classes.clone(),
                ..Default::default()
            })],
        ),
    );
    let operation = domain::WorthQueryDomainOperationDefinition::<
        GeometryDomain,
        WorkflowMutation,
        MutationFamily,
    >::new(
        domain::WorthQueryDomainOperationIdentity::new("mixed-workflow-mutation", 1),
        semantics,
    );
    let package = domain::WorthQueryDomainPackage::declare(
        GeometryDomain,
        domain::WorthQueryDomainIdentityDeclaration::new(
            domain::WorthQueryDomainIdentityNamespace::new("WORTH.tests").unwrap(),
            domain::WorthQueryDomainIdentityName::new("geometry").unwrap(),
            domain::WorthQueryDomainSemanticVersion::new(1, 0),
        ),
    )
    .operation(operation)
    .operation_graph_participation::<WorkflowMutation, MutationFamily, G>("remote-a");
    configured_runtime_without_executors(package).workflow_stage_executor(
        GeometryDomain,
        WorkflowMutation,
        MutationFamily,
        MixedMutationWorkflowExecutor,
    )
}
