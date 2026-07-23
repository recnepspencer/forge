use worth_query::facade::{domain, runtime};

use super::{
    conditional_model_graph_definition, conditional_package_with_access, ConditionalInstallation,
    ConditionalModelGraph, ConditionalModelGraphProvider, GeometryDomain, ReadFamily, ReadVertex,
    ReadVertexExecutor,
};

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
    conditional_public_runtime_with(node, installation, compute, harness)?.workspace(name)
}

pub(crate) fn conditional_public_observe_workspace_with_invalidation<P>(
    name: &str,
    node: domain::WorthQueryPortableConditionalNodeDeclaration,
    installation: ConditionalInstallation,
    compute: P,
    harness: &crate::support::public_bridge_runtime::PublicBridgeRuntimeHarness,
    invalidation: domain::WorthQueryConsumerSupportPosture,
) -> Result<runtime::WorthQueryWorkspace, runtime::WorthQueryRuntimeError>
where
    P: domain::WorthQueryConditionalNodeComputeProvider<GeometryDomain, ReadVertex, ReadFamily>,
{
    conditional_public_runtime_with_access(
        node,
        installation,
        compute,
        harness,
        domain::WorthQueryOperationGraphAccess::Observe,
        invalidation,
    )?
    .workspace(name)
}

pub(crate) fn conditional_public_controlled_workspace_with<P>(
    name: &str,
    node: domain::WorthQueryPortableConditionalNodeDeclaration,
    installation: ConditionalInstallation,
    compute: P,
    harness: &crate::support::public_bridge_runtime::PublicBridgeRuntimeHarness,
) -> Result<
    worth_query::facade::consumer_kit::WorthQueryControlledTestWorkspace,
    runtime::WorthQueryRuntimeError,
>
where
    P: domain::WorthQueryConditionalNodeComputeProvider<GeometryDomain, ReadVertex, ReadFamily>,
{
    let runtime = conditional_public_runtime_with(node, installation, compute, harness)?;
    worth_query::facade::consumer_kit::WorthQueryControlledTestWorkspace::from_runtime(
        name, runtime,
    )
}

fn conditional_public_runtime_with<P>(
    node: domain::WorthQueryPortableConditionalNodeDeclaration,
    installation: ConditionalInstallation,
    compute: P,
    harness: &crate::support::public_bridge_runtime::PublicBridgeRuntimeHarness,
) -> Result<runtime::WorthQueryRuntime, runtime::WorthQueryRuntimeError>
where
    P: domain::WorthQueryConditionalNodeComputeProvider<GeometryDomain, ReadVertex, ReadFamily>,
{
    conditional_public_runtime_with_access(
        node,
        installation,
        compute,
        harness,
        domain::WorthQueryOperationGraphAccess::Project,
        domain::WorthQueryConsumerSupportPosture::Supported,
    )
}

fn conditional_public_runtime_with_access<P>(
    node: domain::WorthQueryPortableConditionalNodeDeclaration,
    installation: ConditionalInstallation,
    compute: P,
    harness: &crate::support::public_bridge_runtime::PublicBridgeRuntimeHarness,
    graph_access: domain::WorthQueryOperationGraphAccess,
    invalidation: domain::WorthQueryConsumerSupportPosture,
) -> Result<runtime::WorthQueryRuntime, runtime::WorthQueryRuntimeError>
where
    P: domain::WorthQueryConditionalNodeComputeProvider<GeometryDomain, ReadVertex, ReadFamily>,
{
    let dependency_contract = node.dependencies()[0].contract().clone();
    let builder = runtime::WorthQueryRuntime::builder()
        .domain_package(conditional_package_with_access(vec![node], graph_access))
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
        )
        .consumer_support_posture(
            domain::WorthQueryConsumerSupportDimension::ConditionalTemporalOrOnDemand,
            domain::WorthQueryConsumerSupportPosture::Supported,
        )
        .consumer_support_posture(
            domain::WorthQueryConsumerSupportDimension::Live,
            domain::WorthQueryConsumerSupportPosture::Supported,
        )
        .consumer_support_posture(
            domain::WorthQueryConsumerSupportDimension::Sharing,
            domain::WorthQueryConsumerSupportPosture::Supported,
        )
        .consumer_support_posture(
            domain::WorthQueryConsumerSupportDimension::DependencyImpact,
            domain::WorthQueryConsumerSupportPosture::Supported,
        )
        .consumer_support_posture(
            domain::WorthQueryConsumerSupportDimension::Invalidation,
            invalidation,
        );
    harness
        .configure_runtime_builder(
            builder,
            installation.bridge,
            [super::super::identity_contract(), dependency_contract],
            crate::support::public_bridge_runtime::public_graph_support_profile(),
        )
        .build_backend_from_parts()
        .build()
}
