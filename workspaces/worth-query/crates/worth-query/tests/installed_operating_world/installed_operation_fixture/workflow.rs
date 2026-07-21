use worth_query::facade::{domain, runtime};

use super::executors::{
    MismatchedWorkflowDeterminismExecutor, MismatchedWorkflowStageExecutor, WorkflowStageExecutor,
};
use super::workflow_parallel_providers::{SerialParallelProvider, WorkflowParallelProvider};
use super::{
    canonical_bundle, configured_runtime_without_executors, semantic_closure, GeometryDomain,
    ReadFamily, WorkflowRead,
};

#[path = "workflow/definitions.rs"]
mod definitions;
pub(super) use definitions::valid_stages;
pub use definitions::InvalidWorkflow;
use definitions::{no_value, projection, stage, text, workflow_definition};

pub fn workflow_workspace(
    name: &str,
) -> Result<
    runtime::WorthQueryWorkspace,
    worth_query::facade::consumer_kit::WorthQueryTestBackendError,
> {
    build_workspace(name, workflow_definition(None))
}

pub fn reversed_workflow_workspace(
    name: &str,
) -> Result<
    runtime::WorthQueryWorkspace,
    worth_query::facade::consumer_kit::WorthQueryTestBackendError,
> {
    let mut stages = valid_stages();
    stages.reverse();
    build_workspace(
        name,
        domain::WorthQueryPortableWorkflowDefinition::new("start", stages),
    )
}

pub fn divergent_frontier_workspace(
    name: &str,
) -> Result<
    runtime::WorthQueryWorkspace,
    worth_query::facade::consumer_kit::WorthQueryTestBackendError,
> {
    build_workspace(
        name,
        domain::WorthQueryPortableWorkflowDefinition::new(
            "start",
            [
                stage("start", [], false, false, no_value(), text()),
                stage("left", ["start"], false, false, text(), text()),
                stage("bridge", ["start"], false, false, text(), text()),
                stage("right", ["bridge"], false, false, text(), text()),
                stage(
                    "publish",
                    ["left", "right"],
                    true,
                    true,
                    text(),
                    projection(),
                ),
            ],
        ),
    )
}

pub fn invalid_workflow_workspace(
    name: &str,
    invalid: InvalidWorkflow,
) -> Result<
    runtime::WorthQueryWorkspace,
    worth_query::facade::consumer_kit::WorthQueryTestBackendError,
> {
    build_workspace(name, workflow_definition(Some(invalid)))
}

pub fn mismatched_workflow_lowering_workspace(
    name: &str,
) -> Result<
    runtime::WorthQueryWorkspace,
    worth_query::facade::consumer_kit::WorthQueryTestBackendError,
> {
    configured_runtime_without_executors(workflow_package(workflow_definition(None), true))
        .replayable_workflow_stage_executor(
            GeometryDomain,
            WorkflowRead,
            ReadFamily,
            MismatchedWorkflowStageExecutor,
        )
        .workflow_parallel_admission_provider(
            GeometryDomain,
            WorkflowRead,
            ReadFamily,
            WorkflowParallelProvider,
        )
        .workspace(name)
}

pub fn mismatched_workflow_determinism_workspace(
    name: &str,
) -> Result<
    runtime::WorthQueryWorkspace,
    worth_query::facade::consumer_kit::WorthQueryTestBackendError,
> {
    configured_runtime_without_executors(workflow_package(workflow_definition(None), true))
        .replayable_workflow_stage_executor(
            GeometryDomain,
            WorkflowRead,
            ReadFamily,
            MismatchedWorkflowDeterminismExecutor,
        )
        .workflow_parallel_admission_provider(
            GeometryDomain,
            WorkflowRead,
            ReadFamily,
            WorkflowParallelProvider,
        )
        .workspace(name)
}

pub fn missing_replay_comparator_workspace(
    name: &str,
) -> Result<
    runtime::WorthQueryWorkspace,
    worth_query::facade::consumer_kit::WorthQueryTestBackendError,
> {
    configured_runtime_without_executors(workflow_package(workflow_definition(None), true))
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

pub fn missing_parallel_provider_workspace(
    name: &str,
) -> Result<
    runtime::WorthQueryWorkspace,
    worth_query::facade::consumer_kit::WorthQueryTestBackendError,
> {
    configured_runtime_without_executors(workflow_package(workflow_definition(None), true))
        .replayable_workflow_stage_executor(
            GeometryDomain,
            WorkflowRead,
            ReadFamily,
            WorkflowStageExecutor,
        )
        .workspace(name)
}

pub fn serial_parallel_provider_workspace(
    name: &str,
) -> Result<
    runtime::WorthQueryWorkspace,
    worth_query::facade::consumer_kit::WorthQueryTestBackendError,
> {
    configured_runtime_without_executors(workflow_package(workflow_definition(None), true))
        .replayable_workflow_stage_executor(
            GeometryDomain,
            WorkflowRead,
            ReadFamily,
            WorkflowStageExecutor,
        )
        .workflow_parallel_admission_provider(
            GeometryDomain,
            WorkflowRead,
            ReadFamily,
            SerialParallelProvider,
        )
        .workspace(name)
}

pub fn nondeterministic_workflow_workspace(
    name: &str,
) -> Result<
    runtime::WorthQueryWorkspace,
    worth_query::facade::consumer_kit::WorthQueryTestBackendError,
> {
    configured_runtime_without_executors(workflow_package(workflow_definition(None), false))
        .replayable_workflow_stage_executor(
            GeometryDomain,
            WorkflowRead,
            ReadFamily,
            MismatchedWorkflowDeterminismExecutor,
        )
        .workflow_parallel_admission_provider(
            GeometryDomain,
            WorkflowRead,
            ReadFamily,
            WorkflowParallelProvider,
        )
        .workspace(name)
}

pub(super) fn build_workspace(
    name: &str,
    workflow: domain::WorthQueryPortableWorkflowDefinition,
) -> Result<
    runtime::WorthQueryWorkspace,
    worth_query::facade::consumer_kit::WorthQueryTestBackendError,
> {
    configured_runtime_without_executors(workflow_package(workflow, true))
        .replayable_workflow_stage_executor(
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

pub(super) fn workflow_package(
    workflow: domain::WorthQueryPortableWorkflowDefinition,
    deterministic: bool,
) -> domain::WorthQueryDomainPackage<GeometryDomain> {
    let mut semantics = semantic_closure(
        canonical_bundle("Vertex"),
        domain::WorthQuerySupportRequirement::Required,
        true,
    );
    semantics.lowering.deterministic = deterministic;
    semantics.replay = domain::WorthQueryOperationReplayContract::CertReplayable {
        comparator: domain::WorthQueryOperationReplayComparatorContract {
            family: "installed-workflow-exact-v1",
        },
    };
    semantics.workflow = domain::WorthQueryOperationWorkflowContract::Declared(workflow);
    let operation = domain::WorthQueryDomainOperationDefinition::<
        GeometryDomain,
        WorkflowRead,
        ReadFamily,
    >::new(
        domain::WorthQueryDomainOperationIdentity::new("workflow-read", 1),
        semantics,
    );
    domain::WorthQueryDomainPackage::declare(
        GeometryDomain,
        domain::WorthQueryDomainIdentityDeclaration::new(
            domain::WorthQueryDomainIdentityNamespace::new("WORTH.tests").unwrap(),
            domain::WorthQueryDomainIdentityName::new("geometry").unwrap(),
            domain::WorthQueryDomainSemanticVersion::new(1, 0),
        ),
    )
    .operation(operation)
}
