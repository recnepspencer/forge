use worth_query::facade::{domain, runtime};

use super::executors::{
    MismatchedWorkflowDeterminismExecutor, MismatchedWorkflowStageExecutor, WorkflowStageExecutor,
};
use super::workflow_parallel_providers::{SerialParallelProvider, WorkflowParallelProvider};
use super::{
    canonical_bundle, configured_runtime_without_executors, semantic_closure, GeometryDomain,
    ReadFamily, WorkflowRead,
};

#[derive(Clone, Copy, Debug)]
pub enum InvalidWorkflow {
    Cycle,
    MissingPredecessor,
    DuplicateStage,
    ExtraRoot,
    IncompleteTerminalPath,
    UndeclaredRequiredDomain,
    UnusedOperationGraphRead,
}

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
        .workflow_stage_executor(
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
        .workflow_stage_executor(
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

pub fn missing_parallel_provider_workspace(
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
        .workspace(name)
}

pub fn serial_parallel_provider_workspace(
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
        .workflow_stage_executor(
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

fn workflow_definition(
    invalid: Option<InvalidWorkflow>,
) -> domain::WorthQueryPortableWorkflowDefinition {
    let stages = match invalid {
        None => valid_stages(),
        Some(InvalidWorkflow::Cycle) => vec![
            stage("start", [], false, false, no_value(), text()),
            stage("left", ["right"], false, false, text(), text()),
            stage("right", ["left"], false, false, text(), text()),
            stage("publish", ["left"], true, true, text(), projection()),
        ],
        Some(InvalidWorkflow::MissingPredecessor) => vec![
            stage("start", [], false, false, no_value(), text()),
            stage("publish", ["missing"], true, true, text(), projection()),
        ],
        Some(InvalidWorkflow::DuplicateStage) => vec![
            stage("start", [], false, false, no_value(), text()),
            stage("publish", ["start"], true, true, text(), projection()),
            stage("publish", ["start"], true, true, text(), projection()),
        ],
        Some(InvalidWorkflow::ExtraRoot) => vec![
            stage("start", [], false, false, no_value(), text()),
            stage("orphan", [], false, false, no_value(), text()),
            stage("publish", ["start"], true, true, text(), projection()),
        ],
        Some(InvalidWorkflow::IncompleteTerminalPath) => vec![
            stage("start", [], false, false, no_value(), text()),
            stage("dead-end", ["start"], false, false, text(), text()),
            stage("publish", ["start"], true, true, text(), projection()),
        ],
        Some(InvalidWorkflow::UndeclaredRequiredDomain) => {
            let mut stages = valid_stages();
            stages[1] =
                stages[1]
                    .clone()
                    .with_semantics(domain::WorthQueryWorkflowStageSemantics {
                        input: text(),
                        output: text(),
                        required_domain_roles: vec![
                            domain::WorthQueryOperationRequiredDomainRole::new("auxiliary")
                                .unwrap(),
                        ],
                        graph_read_roles: vec!["model".into()],
                        cost_roles: standard_cost_roles(true),
                        failure_classes: vec![domain::WorthQueryOperationFailureClass::Dependency],
                        ..Default::default()
                    });
            stages
        }
        Some(InvalidWorkflow::UnusedOperationGraphRead) => {
            let mut stages = valid_stages();
            let publish = stages.pop().expect("valid workflow retains publication");
            stages.push(
                publish.with_semantics(domain::WorthQueryWorkflowStageSemantics {
                    input: text(),
                    output: projection(),
                    cost_roles: standard_cost_roles(false),
                    terminal_result_states: vec![domain::WorthQueryOperationResultState::Ready],
                    failure_classes: vec![domain::WorthQueryOperationFailureClass::Dependency],
                    ..Default::default()
                }),
            );
            stages
        }
    };
    domain::WorthQueryPortableWorkflowDefinition::new("start", stages)
}

pub(super) fn valid_stages() -> Vec<domain::WorthQueryPortableWorkflowStage> {
    vec![
        stage("start", [], false, false, no_value(), text()),
        stage("left", ["start"], false, false, text(), text()),
        stage("right", ["start"], false, false, text(), text()),
        stage(
            "publish",
            ["left", "right"],
            true,
            true,
            text(),
            projection(),
        ),
    ]
}

fn stage(
    identity: &str,
    predecessors: impl IntoIterator<Item = &'static str>,
    terminal: bool,
    publishable: bool,
    input: domain::WorthQueryWorkflowValueContract,
    output: domain::WorthQueryWorkflowValueContract,
) -> domain::WorthQueryPortableWorkflowStage {
    domain::WorthQueryPortableWorkflowStage::new(
        identity,
        predecessors,
        terminal,
        publishable,
        std::iter::empty::<domain::WorthQueryOperationCapabilityRequirement>(),
    )
    .with_semantics(domain::WorthQueryWorkflowStageSemantics {
        input,
        output,
        required_domain_roles: Vec::new(),
        graph_read_roles: matches!(output, domain::WorthQueryWorkflowValueContract::Projection)
            .then_some("model".into())
            .into_iter()
            .collect(),
        touch_roles: Vec::new(),
        effect_roles: Vec::new(),
        invariant_roles: Vec::new(),
        cost_roles: standard_cost_roles(matches!(
            output,
            domain::WorthQueryWorkflowValueContract::Projection
        )),
        terminal_result_states: terminal
            .then_some(domain::WorthQueryOperationResultState::Ready)
            .into_iter()
            .collect(),
        failure_classes: vec![domain::WorthQueryOperationFailureClass::Dependency],
        conditional_nodes: Vec::new(),
    })
}

fn standard_cost_roles(graph_read: bool) -> Vec<domain::WorthQueryWorkflowCostRole> {
    use domain::WorthQueryWorkflowCostRole as Role;
    let mut roles = vec![Role::Admission, Role::Execution, Role::ResultValidation];
    if graph_read {
        roles.push(Role::GraphRead);
    }
    roles
}

fn no_value() -> domain::WorthQueryWorkflowValueContract {
    domain::WorthQueryWorkflowValueContract::NotRequired
}
fn text() -> domain::WorthQueryWorkflowValueContract {
    domain::WorthQueryWorkflowValueContract::Text
}
fn projection() -> domain::WorthQueryWorkflowValueContract {
    domain::WorthQueryWorkflowValueContract::Projection
}
