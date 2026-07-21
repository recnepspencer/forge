use worth_foundational::facade::AspectContract;
use worth_query::facade::consumer_kit::{in_memory_test_runtime, WorthQueryTestBackendSchema};
use worth_query::facade::{domain, runtime};
mod aftermath;
mod conditional_workflow;
pub(crate) mod conditional_workspace;
mod correspondence_bridge;
mod count_vertices;
mod executors;
mod federated_package;
mod foreign_material;
mod lineage_workflow;
mod mutation_workflow;
mod operation_semantics;
mod required_domains;
mod semantic_drift;
mod touch_package;
mod workflow;
mod workflow_parallel_providers;
pub use aftermath::{
    aftermath_workspace, provisional_workflow_workspace, AftermathCandidate, AftermathContract,
    AftermathFamily, AftermathOriginal, ProvisionalWorkflow,
};
pub use conditional_workflow::{
    conditional_workflow_workspace, reverted_conditional_lineage_workflow_workspace,
};
pub(crate) use conditional_workspace::{
    conditional_installation, conditional_installation_with_change,
    conditional_public_workspace_with, conditional_workspace_with,
};
pub use conditional_workspace::{conditional_workspace, ConditionalModelGraph};
pub(super) use correspondence_bridge::{
    conditional_runtime_bridge, conditional_runtime_bridge_with_change, correspondence_bridge,
};
pub use count_vertices::{CountVertices, CountVerticesInput};
pub use executors::graph_projection_material;
use executors::{
    CountVerticesExecutor, FederatedReadExecutor, ReadVertexExecutor,
    UnderstatedFederatedReadExecutor,
};
pub use federated_package::{
    federated_operation_contract_drift_package, federated_package, FederatedOperationContractDrift,
};
pub use foreign_material::{
    foreign_material_workspace, mismatched_cost_workspace, mismatched_determinism_workspace,
    mismatched_read_plan_workspace, missing_read_execution_workspace,
};
pub use lineage_workflow::{lineage_workflow_workspace, LineageEvidenceScenario};
pub use mutation_workflow::{
    mixed_mutation_workflow_runtime, mutation_workflow_workspace, MutationFamily, WorkflowMutation,
};
pub(super) use operation_semantics::{
    canonical_bundle, canonical_collection_bundle, semantic_closure,
};
pub use required_domains::required_domain_runtime;
pub use semantic_drift::semantic_drift_workspace;
pub use touch_package::federated_touch_package;
pub use workflow::{
    divergent_frontier_workspace, invalid_workflow_workspace,
    mismatched_workflow_determinism_workspace, mismatched_workflow_lowering_workspace,
    missing_parallel_provider_workspace, missing_replay_comparator_workspace,
    nondeterministic_workflow_workspace, reversed_workflow_workspace,
    serial_parallel_provider_workspace, workflow_workspace, InvalidWorkflow,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GeometryDomain;

impl domain::WorthQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str {
        "WORTH.tests.geometry"
    }

    fn display_name(&self) -> &'static str {
        "Geometry"
    }

    fn required_capability_families(&self) -> &'static [domain::WorthQueryCapabilityFamily] {
        &[]
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ReadVertex;

pub struct ReadExecutionInput {
    pub state: domain::WorthQueryOperationResultState,
    pub warning: Option<domain::WorthQueryOperationExecutionWarning>,
    pub failure: Option<domain::WorthQueryOperationFailureClass>,
}

impl Default for ReadExecutionInput {
    fn default() -> Self {
        Self {
            state: domain::WorthQueryOperationResultState::Ready,
            warning: None,
            failure: None,
        }
    }
}

impl domain::WorthQueryOperationInput for ReadExecutionInput {
    fn parameters(&self) -> Vec<domain::WorthQueryOperationParameter<'_>> {
        Vec::new()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ReadVertexLookalike;

#[derive(Clone, Copy, Debug)]
pub struct ReadFamily;

#[derive(Clone, Copy, Debug)]
pub struct FederatedRead;

#[derive(Clone, Copy, Debug)]
pub struct WorkflowRead;

impl domain::WorthQueryExecutableDomainOperation<GeometryDomain, ReadFamily> for ReadVertex {
    type Input = ReadExecutionInput;
    type Output = worth_query::facade::read::WorthQueryReadCompletion;
    type Publication = domain::WorthQueryPublishingOperation;
    type Execution = domain::WorthQueryDirectOperation;
}

impl domain::WorthQueryExecutableDomainOperation<GeometryDomain, ReadFamily> for FederatedRead {
    type Input = ();
    type Output = worth_query::facade::read::WorthQueryReadCompletion;
    type Publication = domain::WorthQueryPublishingOperation;
    type Execution = domain::WorthQueryDirectOperation;
}

impl domain::WorthQueryExecutableDomainOperation<GeometryDomain, ReadFamily> for WorkflowRead {
    type Input = ();
    type Output = worth_query::facade::read::WorthQueryReadCompletion;
    type Publication = domain::WorthQueryPublishingOperation;
    type Execution = domain::WorthQueryWorkflowOperation;
}

pub fn workspace(
    name: &str,
    reversed: bool,
) -> Result<
    runtime::WorthQueryWorkspace,
    worth_query::facade::consumer_kit::WorthQueryTestBackendError,
> {
    workspace_for_package(name, package(reversed, false))
}

pub fn configured_runtime(
) -> worth_query::facade::consumer_kit::WorthQueryInMemoryTestRuntimeBuilder {
    configured_base_runtime_for_package(package(false, false))
}

pub fn conflicting_workspace(
    name: &str,
) -> Result<
    runtime::WorthQueryWorkspace,
    worth_query::facade::consumer_kit::WorthQueryTestBackendError,
> {
    workspace_for_package(name, package(false, true))
}

pub fn lowering_mismatch_workspace(
    name: &str,
) -> Result<
    runtime::WorthQueryWorkspace,
    worth_query::facade::consumer_kit::WorthQueryTestBackendError,
> {
    let base = read_vertex_definition(domain::WorthQuerySupportRequirement::Required);
    let mut semantics = base.semantics().clone();
    semantics.lowering.family = "foreign-lowering-family".into();
    let operation = domain::WorthQueryDomainOperationDefinition::<
        GeometryDomain,
        ReadVertex,
        ReadFamily,
    >::new(base.identity().clone(), semantics);
    let package = domain::WorthQueryDomainPackage::declare(
        GeometryDomain,
        domain::WorthQueryDomainIdentityDeclaration::new(
            domain::WorthQueryDomainIdentityNamespace::new("WORTH.tests").unwrap(),
            domain::WorthQueryDomainIdentityName::new("geometry").unwrap(),
            domain::WorthQueryDomainSemanticVersion::new(1, 0),
        ),
    )
    .operation(operation);
    configured_runtime_without_executors(package)
        .domain_operation_executor(GeometryDomain, ReadVertex, ReadFamily, ReadVertexExecutor)
        .workspace(name)
}

pub fn unsupported_direct_effect_workspace(
    name: &str,
) -> Result<
    runtime::WorthQueryWorkspace,
    worth_query::facade::consumer_kit::WorthQueryTestBackendError,
> {
    let base = read_vertex_definition(domain::WorthQuerySupportRequirement::Required);
    let mut semantics = base.semantics().clone();
    semantics.effects = domain::WorthQueryOperationEffectContract::Declared {
        effect_families: vec![domain::WorthQueryOperationEffectFamily::Mutation],
    };
    let operation = domain::WorthQueryDomainOperationDefinition::<
        GeometryDomain,
        ReadVertex,
        ReadFamily,
    >::new(base.identity().clone(), semantics);
    let package = domain::WorthQueryDomainPackage::declare(
        GeometryDomain,
        domain::WorthQueryDomainIdentityDeclaration::new(
            domain::WorthQueryDomainIdentityNamespace::new("WORTH.tests").unwrap(),
            domain::WorthQueryDomainIdentityName::new("geometry").unwrap(),
            domain::WorthQueryDomainSemanticVersion::new(1, 0),
        ),
    )
    .operation(operation);
    configured_runtime_without_executors(package)
        .domain_operation_executor(GeometryDomain, ReadVertex, ReadFamily, ReadVertexExecutor)
        .workspace(name)
}

pub fn support_dimension_workspace(
    name: &str,
    dimension: domain::WorthQueryConsumerSupportDimension,
    posture: domain::WorthQueryConsumerSupportPosture,
) -> Result<
    runtime::WorthQueryWorkspace,
    worth_query::facade::consumer_kit::WorthQueryTestBackendError,
> {
    let base = read_vertex_definition(domain::WorthQuerySupportRequirement::Required);
    let mut semantics = base.semantics().clone();
    require_support_dimension(&mut semantics.support, dimension);
    let operation = domain::WorthQueryDomainOperationDefinition::<
        GeometryDomain,
        ReadVertex,
        ReadFamily,
    >::new(base.identity().clone(), semantics);
    let package = domain::WorthQueryDomainPackage::declare(
        GeometryDomain,
        domain::WorthQueryDomainIdentityDeclaration::new(
            domain::WorthQueryDomainIdentityNamespace::new("WORTH.tests").unwrap(),
            domain::WorthQueryDomainIdentityName::new("geometry").unwrap(),
            domain::WorthQueryDomainSemanticVersion::new(1, 0),
        ),
    )
    .operation(operation);
    configured_runtime_without_executors(package)
        .consumer_support_posture(dimension, posture)
        .domain_operation_executor(GeometryDomain, ReadVertex, ReadFamily, ReadVertexExecutor)
        .workspace(name)
}

fn require_support_dimension(
    requirements: &mut domain::WorthQueryOperationSupportRequirements,
    dimension: domain::WorthQueryConsumerSupportDimension,
) {
    use domain::WorthQueryConsumerSupportDimension as Dimension;
    let required = domain::WorthQuerySupportRequirement::Required;
    match dimension {
        Dimension::Basis => {}
        Dimension::Live => requirements.live = required,
        Dimension::Continuation => requirements.continuation = required,
        Dimension::AsyncResultState => requirements.async_result_state = required,
        Dimension::Recovery => requirements.recovery = required,
        Dimension::Inspection => requirements.inspection = required,
        Dimension::ProjectionConsumption => requirements.projection_consumption = required,
        Dimension::DependencyImpact => requirements.dependency_impact = required,
        Dimension::Sharing => requirements.sharing = required,
        Dimension::Invalidation => requirements.invalidation = required,
        Dimension::CollectionDelivery => requirements.collection_delivery = required,
        Dimension::ConditionalEvaluation => requirements.conditional_evaluation = required,
        Dimension::ConditionalComparator => requirements.conditional_comparator = required,
        Dimension::ConditionalTrigger => requirements.conditional_trigger = required,
        Dimension::ConditionalTemporalOrOnDemand => {
            requirements.conditional_temporal_or_on_demand = required
        }
    }
}

fn workspace_for_package(
    name: &str,
    package: domain::WorthQueryDomainPackage<GeometryDomain>,
) -> Result<
    runtime::WorthQueryWorkspace,
    worth_query::facade::consumer_kit::WorthQueryTestBackendError,
> {
    configured_base_runtime_for_package(package).workspace(name)
}

pub fn configured_runtime_for_package(
    package: domain::WorthQueryDomainPackage<GeometryDomain>,
) -> worth_query::facade::consumer_kit::WorthQueryInMemoryTestRuntimeBuilder {
    configured_runtime_without_executors(package).domain_operation_executor(
        GeometryDomain,
        FederatedRead,
        ReadFamily,
        FederatedReadExecutor,
    )
}

pub fn configured_runtime_for_understated_cost_package(
    package: domain::WorthQueryDomainPackage<GeometryDomain>,
) -> worth_query::facade::consumer_kit::WorthQueryInMemoryTestRuntimeBuilder {
    configured_runtime_without_executors(package).domain_operation_executor(
        GeometryDomain,
        FederatedRead,
        ReadFamily,
        UnderstatedFederatedReadExecutor,
    )
}

fn configured_base_runtime_for_package(
    package: domain::WorthQueryDomainPackage<GeometryDomain>,
) -> worth_query::facade::consumer_kit::WorthQueryInMemoryTestRuntimeBuilder {
    configured_runtime_without_executors(package)
        .domain_operation_executor(GeometryDomain, ReadVertex, ReadFamily, ReadVertexExecutor)
        .domain_operation_executor(
            GeometryDomain,
            CountVertices,
            ReadFamily,
            CountVerticesExecutor,
        )
}

fn configured_runtime_without_executors(
    package: domain::WorthQueryDomainPackage<GeometryDomain>,
) -> worth_query::facade::consumer_kit::WorthQueryInMemoryTestRuntimeBuilder {
    let schema = WorthQueryTestBackendSchema::single_collection("Vertex")
        .aspect_contract(identity_contract())
        .unwrap()
        .aspect("identity.id", "identity.id")
        .unwrap();
    in_memory_test_runtime()
        .with_schema(schema)
        .domain_package(package)
}

fn package(
    reversed: bool,
    conflicting_duplicate: bool,
) -> domain::WorthQueryDomainPackage<GeometryDomain> {
    let read = read_vertex_definition(domain::WorthQuerySupportRequirement::Required);
    let count = count_vertices_definition();
    let mut package = domain::WorthQueryDomainPackage::declare(
        GeometryDomain,
        domain::WorthQueryDomainIdentityDeclaration::new(
            domain::WorthQueryDomainIdentityNamespace::new("WORTH.tests").unwrap(),
            domain::WorthQueryDomainIdentityName::new("geometry").unwrap(),
            domain::WorthQueryDomainSemanticVersion::new(1, 0),
        ),
    );
    package = if reversed {
        package.operation(count).operation(read)
    } else {
        package.operation(read).operation(count)
    };
    if conflicting_duplicate {
        package = package.operation(read_vertex_definition(
            domain::WorthQuerySupportRequirement::NotRequired,
        ));
    }
    package
}

pub fn read_vertex_definition(
    projection_consumption: domain::WorthQuerySupportRequirement,
) -> domain::WorthQueryDomainOperationDefinition<GeometryDomain, ReadVertex, ReadFamily> {
    let bundle = canonical_bundle("Vertex");
    domain::WorthQueryDomainOperationDefinition::new(
        domain::WorthQueryDomainOperationIdentity::new("read-vertex", 1),
        semantic_closure(bundle, projection_consumption, true),
    )
}

fn count_vertices_definition(
) -> domain::WorthQueryDomainOperationDefinition<GeometryDomain, CountVertices, ReadFamily> {
    let bundle = canonical_bundle("Vertex");
    let mut semantics = semantic_closure(
        bundle,
        domain::WorthQuerySupportRequirement::NotRequired,
        false,
    );
    semantics.parameters = domain::WorthQueryOperationParameterContract::Declared {
        fields: vec![domain::WorthQueryOperationParameterField {
            name: "minimum".into(),
            value_family: domain::WorthQueryOperationValueFamily::U64,
            required: true,
        }],
    };
    domain::WorthQueryDomainOperationDefinition::new(
        domain::WorthQueryDomainOperationIdentity::new("count-vertices", 1),
        semantics,
    )
}

fn identity_contract() -> AspectContract {
    super::conditional_node_contract::identity_contract()
}
