use worth_query::facade::{domain, runtime};

use super::executors::ReadVertexExecutor;
use super::{
    conditional_runtime_bridge, conditional_runtime_bridge_with_change,
    configured_runtime_without_executors, read_vertex_definition, GeometryDomain, ReadFamily,
    ReadVertex,
};

mod providers;

use providers::{providers_for, DirectConditionalCompute};

#[derive(Clone, Copy, Debug)]
pub struct ConditionalModelGraph;

pub(super) struct ConditionalModelGraphProvider;

pub(super) fn conditional_model_graph_definition(
) -> domain::WorthQueryGraphParticipationDefinition<ConditionalModelGraph> {
    domain::WorthQueryGraphParticipationDefinition::new(
        "model",
        domain::WorthQueryGraphParticipationContract {
            observation: domain::WorthQueryGraphObservationPosture::Snapshot,
            projection: domain::WorthQueryGraphProjectionPosture::NativeProjection,
            mutation: domain::WorthQueryGraphMutationPosture::NotRequired,
            identity: domain::WorthQueryGraphIdentityPosture::Opaque,
            locality: domain::WorthQueryGraphLocalityPosture::InProcess,
            budget: domain::WorthQueryGraphBudgetPosture::ConstantAdmission,
            commit: domain::WorthQueryGraphCommitPosture::ReadOnly,
            failure: domain::WorthQueryGraphFailureTopology::Local,
        },
    )
}

impl domain::WorthQueryGraphParticipationProvider<ConditionalModelGraph>
    for ConditionalModelGraphProvider
{
    fn observe(
        &self,
        call: &domain::WorthQueryGraphProviderCall,
    ) -> Result<domain::WorthQueryGraphProviderReceipt, domain::WorthQueryGraphProviderFailure>
    {
        Ok(call.completed("conditional-model-observe"))
    }

    fn project(
        &self,
        call: &domain::WorthQueryGraphProviderCall,
    ) -> Result<domain::WorthQueryGraphProviderReceipt, domain::WorthQueryGraphProviderFailure>
    {
        Ok(call.completed("conditional-model-project"))
    }

    fn touch_effect(
        &self,
        call: &domain::WorthQueryGraphProviderCall,
    ) -> Result<domain::WorthQueryGraphProviderReceipt, domain::WorthQueryGraphProviderFailure>
    {
        Ok(call.completed("conditional-model-touch"))
    }
}

pub fn conditional_workspace(
    name: &str,
    node: domain::WorthQueryPortableConditionalNodeDeclaration,
) -> Result<
    runtime::WorthQueryWorkspace,
    worth_query::facade::consumer_kit::WorthQueryTestBackendError,
> {
    let installation = conditional_installation(&node);
    conditional_workspace_with(name, node, installation, DirectConditionalCompute)
}

pub(crate) fn conditional_workspace_with<P>(
    name: &str,
    node: domain::WorthQueryPortableConditionalNodeDeclaration,
    installation: ConditionalInstallation,
    compute: P,
) -> Result<
    runtime::WorthQueryWorkspace,
    worth_query::facade::consumer_kit::WorthQueryTestBackendError,
>
where
    P: domain::WorthQueryConditionalNodeComputeProvider<GeometryDomain, ReadVertex, ReadFamily>,
{
    conditional_workspace_builder(vec![node])
        .conditional_runtime(installation.bridge, installation.graph)
        .conditional_node(
            GeometryDomain,
            ReadVertex,
            ReadFamily,
            ConditionalModelGraph,
            domain::WorthQueryConditionalNodeLocation::operation(installation.node_identity)
                .unwrap(),
            vec![installation.dependency],
            installation.providers,
            compute,
        )
        .domain_operation_executor(GeometryDomain, ReadVertex, ReadFamily, ReadVertexExecutor)
        .workspace(name)
}

pub(crate) fn conditional_workspace_without_lowering(
    name: &str,
    node: domain::WorthQueryPortableConditionalNodeDeclaration,
) -> Result<
    runtime::WorthQueryWorkspace,
    worth_query::facade::consumer_kit::WorthQueryTestBackendError,
> {
    conditional_workspace_builder(vec![node])
        .domain_operation_executor(GeometryDomain, ReadVertex, ReadFamily, ReadVertexExecutor)
        .workspace(name)
}

pub(crate) fn conditional_public_workspace_with<P>(
    name: &str,
    node: domain::WorthQueryPortableConditionalNodeDeclaration,
    installation: ConditionalInstallation,
    compute: P,
    harness: &crate::support::public_bridge_runtime::PublicBridgeRuntimeHarness,
) -> Result<runtime::WorthQueryWorkspace, runtime::WorthQueryRuntimeError>
where
    P: domain::WorthQueryConditionalNodeComputeProvider<GeometryDomain, ReadVertex, ReadFamily>,
{
    let dependency_contract = node.dependencies()[0].contract().clone();
    let builder = runtime::WorthQueryRuntime::builder()
        .domain_package(conditional_package(vec![node]))
        .expect("conditional public package should admit")
        .graph_participation(conditional_model_graph_definition())
        .graph_participation_provider(ConditionalModelGraph, ConditionalModelGraphProvider)
        .conditional_signal_graph(installation.graph)
        .conditional_node(
            GeometryDomain,
            ReadVertex,
            ReadFamily,
            ConditionalModelGraph,
            domain::WorthQueryConditionalNodeLocation::operation(installation.node_identity)
                .unwrap(),
            vec![installation.dependency],
            installation.providers,
            compute,
        )
        .domain_operation_executor(GeometryDomain, ReadVertex, ReadFamily, ReadVertexExecutor)
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
        );
    harness
        .configure_runtime_builder(
            builder,
            installation.bridge,
            [super::identity_contract(), dependency_contract],
            crate::support::public_bridge_runtime::public_graph_support_profile(),
        )
        .build_backend_from_parts()
        .build()?
        .workspace(name)
}

fn conditional_workspace_builder(
    nodes: Vec<domain::WorthQueryPortableConditionalNodeDeclaration>,
) -> worth_query::facade::consumer_kit::WorthQueryInMemoryTestRuntimeBuilder {
    configured_runtime_without_executors(conditional_package(nodes))
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
}

fn conditional_package(
    nodes: Vec<domain::WorthQueryPortableConditionalNodeDeclaration>,
) -> domain::WorthQueryDomainPackage<GeometryDomain> {
    let base = read_vertex_definition(domain::WorthQuerySupportRequirement::Required);
    let mut semantics = base.semantics().clone();
    if let domain::WorthQueryOperationGraphReadContract::Declared { roles } =
        &mut semantics.graph_reads
    {
        let model = roles
            .iter_mut()
            .find(|role| role.role == "model")
            .expect("conditional fixture declares the model graph read");
        for dependency in nodes.iter().flat_map(|node| node.dependencies()) {
            let projection = domain::WorthQueryOperationNativeProjectionContract {
                aspect_key: dependency.contract().key().clone(),
                aspect_identity: dependency.contract().identity(),
                contract_revision: dependency.contract().revision(),
                mask: dependency.projection_mask().clone(),
            };
            if !model.semantic_reads.contains(&projection) {
                model.semantic_reads.push(projection);
            }
        }
    }
    semantics.conditional_nodes = nodes;
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
    .operation(operation)
    .operation_graph_participation::<ReadVertex, ReadFamily, ConditionalModelGraph>("model");
    package
}

pub(crate) struct ConditionalInstallation {
    pub(crate) bridge: worth_runtime_bridge::facade::RuntimeBridge,
    pub(crate) graph: worth_signal::facade::SignalGraph,
    pub(crate) node_identity: String,
    pub(crate) signal_node: worth_signal::facade::NodeId,
    pub(crate) dependency: domain::WorthQueryConditionalDependencyInstallation,
    pub(crate) providers: worth_runtime_bridge::facade::BridgeConditionalProviderSet,
}

pub(crate) fn conditional_installation(
    node: &domain::WorthQueryPortableConditionalNodeDeclaration,
) -> ConditionalInstallation {
    let dependency = &node.dependencies()[0];
    let bridge = conditional_runtime_bridge(dependency);
    let mut graph = worth_signal::facade::SignalGraph::new();
    let signal_node_id = graph.node().build();
    let worth_proof::TransitionOutcome::Success(signal_node) =
        graph.admit_installed_node(signal_node_id)
    else {
        panic!("fresh Signal node should admit")
    };
    let target = worth_runtime_bridge::facade::BridgeSignalAspectTargetDeclaration::allocate(
        worth_runtime_bridge::facade::BridgeAspectRegistrationId::from_stable_name(
            "conditional-identity",
        ),
        worth_signal::facade::PartitionToken::new("geometry-signal"),
        signal_node,
    );
    let source = match dependency.locality() {
        domain::WorthQuerySemanticLocality::SourceRecord => {
            Some(worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts::entity(0, 0, 1))
        }
        domain::WorthQuerySemanticLocality::SourcePartition(_)
        | domain::WorthQuerySemanticLocality::WholeLogicalGraph => None,
    };
    ConditionalInstallation {
        bridge,
        graph,
        node_identity: node.identity().to_string(),
        signal_node: signal_node_id,
        dependency: domain::WorthQueryConditionalDependencyInstallation::new(source, vec![target]),
        providers: providers_for(node),
    }
}

pub(crate) fn conditional_installation_with_change(
    node: &domain::WorthQueryPortableConditionalNodeDeclaration,
) -> (
    ConditionalInstallation,
    worth_runtime_bridge::facade::RelationalCommittedPatchRequest,
    [worth_runtime_bridge::facade::RelationalBridgeSnapshotIdentityParts; 2],
) {
    let dependency = &node.dependencies()[0];
    let (bridge, request, record, snapshots) = conditional_runtime_bridge_with_change(dependency);
    let mut graph = worth_signal::facade::SignalGraph::new();
    let signal_node_id = graph.node().build();
    let worth_proof::TransitionOutcome::Success(signal_node) =
        graph.admit_installed_node(signal_node_id)
    else {
        panic!("fresh Signal node should admit")
    };
    let target = worth_runtime_bridge::facade::BridgeSignalAspectTargetDeclaration::allocate(
        worth_runtime_bridge::facade::BridgeAspectRegistrationId::from_stable_name(
            "conditional-identity",
        ),
        worth_signal::facade::PartitionToken::new("geometry-signal"),
        signal_node,
    );
    (
        ConditionalInstallation {
            bridge,
            graph,
            node_identity: node.identity().to_string(),
            signal_node: signal_node_id,
            dependency: domain::WorthQueryConditionalDependencyInstallation::new(
                Some(record),
                vec![target],
            ),
            providers: providers_for(node),
        },
        request,
        snapshots,
    )
}

pub(crate) fn shared_signal_node_workspace(
    name: &str,
    first: domain::WorthQueryPortableConditionalNodeDeclaration,
    second: domain::WorthQueryPortableConditionalNodeDeclaration,
) -> Result<
    runtime::WorthQueryWorkspace,
    worth_query::facade::consumer_kit::WorthQueryTestBackendError,
> {
    let second_providers = providers_for(&second);
    let first_identity = first.identity().to_string();
    let second_identity = second.identity().to_string();
    let installation = conditional_installation(&first);
    let worth_proof::TransitionOutcome::Success(second_node) = installation
        .graph
        .admit_installed_node(installation.signal_node)
    else {
        panic!("the installed Signal node should remain current during fixture construction")
    };
    let second_target = worth_runtime_bridge::facade::BridgeSignalAspectTargetDeclaration::allocate(
        worth_runtime_bridge::facade::BridgeAspectRegistrationId::from_stable_name(
            "conditional-identity",
        ),
        worth_signal::facade::PartitionToken::new("geometry-signal"),
        second_node,
    );
    let second_dependency = domain::WorthQueryConditionalDependencyInstallation::new(
        Some(worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts::entity(0, 0, 1)),
        vec![second_target],
    );
    let ConditionalInstallation {
        bridge,
        graph,
        dependency: first_dependency,
        providers: first_providers,
        ..
    } = installation;
    conditional_workspace_builder(vec![first, second])
        .conditional_runtime(bridge, graph)
        .conditional_node(
            GeometryDomain,
            ReadVertex,
            ReadFamily,
            ConditionalModelGraph,
            domain::WorthQueryConditionalNodeLocation::operation(first_identity).unwrap(),
            vec![first_dependency],
            first_providers,
            DirectConditionalCompute,
        )
        .conditional_node(
            GeometryDomain,
            ReadVertex,
            ReadFamily,
            ConditionalModelGraph,
            domain::WorthQueryConditionalNodeLocation::operation(second_identity).unwrap(),
            vec![second_dependency],
            second_providers,
            DirectConditionalCompute,
        )
        .domain_operation_executor(GeometryDomain, ReadVertex, ReadFamily, ReadVertexExecutor)
        .workspace(name)
}
