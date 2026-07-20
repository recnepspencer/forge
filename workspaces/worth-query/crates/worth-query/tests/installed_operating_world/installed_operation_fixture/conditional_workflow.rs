use worth_query::facade::{domain, runtime};

use super::conditional_workspace::{
    conditional_installation, conditional_model_graph_definition, ConditionalModelGraphProvider,
};
use super::executors::WorkflowStageExecutor;
use super::workflow::{valid_stages, workflow_package};
use super::workflow_parallel_providers::WorkflowParallelProvider;
use super::{
    configured_runtime_without_executors, ConditionalModelGraph, GeometryDomain, ReadFamily,
    WorkflowRead,
};

pub fn conditional_workflow_workspace(
    name: &str,
    node: domain::WorthQueryPortableConditionalNodeDeclaration,
) -> Result<
    runtime::WorthQueryWorkspace,
    worth_query::facade::consumer_kit::WorthQueryTestBackendError,
> {
    let installation = conditional_installation(&node);
    let mut stages = valid_stages();
    let publish = stages
        .pop()
        .expect("the standard workflow retains its publication stage");
    let mut semantics = publish.semantics().clone();
    semantics.conditional_nodes = vec![node];
    stages.push(publish.with_semantics(semantics));
    let package = workflow_package(
        domain::WorthQueryPortableWorkflowDefinition::new("start", stages),
        true,
    )
    .operation_graph_participation::<WorkflowRead, ReadFamily, ConditionalModelGraph>("model");
    configured_runtime_without_executors(package)
        .graph_participation(conditional_model_graph_definition())
        .graph_participation_provider(ConditionalModelGraph, ConditionalModelGraphProvider)
        .consumer_support_posture(
            domain::WorthQueryConsumerSupportDimension::ConditionalEvaluation,
            domain::WorthQueryConsumerSupportPosture::Supported,
        )
        .consumer_support_posture(
            domain::WorthQueryConsumerSupportDimension::ConditionalComparator,
            domain::WorthQueryConsumerSupportPosture::Supported,
        )
        .consumer_support_posture(
            domain::WorthQueryConsumerSupportDimension::ConditionalTrigger,
            domain::WorthQueryConsumerSupportPosture::Supported,
        )
        .consumer_support_posture(
            domain::WorthQueryConsumerSupportDimension::ConditionalTemporalOrOnDemand,
            domain::WorthQueryConsumerSupportPosture::Supported,
        )
        .conditional_runtime(installation.bridge, installation.graph)
        .conditional_node(
            GeometryDomain,
            WorkflowRead,
            ReadFamily,
            ConditionalModelGraph,
            domain::WorthQueryConditionalNodeLocation::workflow_stage(
                "publish",
                installation.node_identity,
            )
            .unwrap(),
            vec![installation.dependency],
            installation.providers,
            WorkflowConditionalCompute,
        )
        .workflow_stage_executor(
            GeometryDomain,
            WorkflowRead,
            ReadFamily,
            WorkflowStageExecutor,
        )
        .workflow_parallel_admission_provider(
            GeometryDomain,
            WorkflowRead,
            ReadFamily,
            WorkflowParallelProvider,
        )
        .workspace(name)
}

struct WorkflowConditionalCompute;
impl domain::WorthQueryConditionalNodeComputeProvider<GeometryDomain, WorkflowRead, ReadFamily>
    for WorkflowConditionalCompute
{
    fn compute(
        &self,
        context: &domain::WorthQueryConditionalComputeContext,
    ) -> Result<worth_signal::facade::NodeEvaluationResult, String> {
        if context.workflow_run_identity().is_none() {
            return Err("workflow conditional compute lost its run identity".into());
        }
        Ok(worth_signal::facade::NodeEvaluationResult::from_version(
            worth_signal::facade::AspectVersion::from_updates([(
                worth_signal::facade::Aspect::new(0),
                1,
            )]),
        ))
    }
}
