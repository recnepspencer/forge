use worth_query::facade::{domain, runtime};

use super::super::{executors::ReadVertexExecutor, GeometryDomain, ReadFamily, ReadVertex};
use super::{
    conditional_model_graph_definition, conditional_package,
    installation::conditional_installation_with_change, providers::providers_for,
    ConditionalModelGraph,
};

pub(crate) fn conditional_public_sibling_workspace_with_change<P, Q>(
    name: &str,
    first: domain::WorthQueryPortableConditionalNodeDeclaration,
    second: domain::WorthQueryPortableConditionalNodeDeclaration,
    first_compute: P,
    second_compute: Q,
    harness: &crate::support::public_bridge_runtime::PublicBridgeRuntimeHarness,
) -> Result<
    (
        runtime::WorthQueryWorkspace,
        worth_runtime_bridge::facade::RelationalCommittedPatchRequest,
        [worth_runtime_bridge::facade::RelationalBridgeSnapshotIdentityParts; 2],
    ),
    runtime::WorthQueryRuntimeError,
>
where
    P: domain::WorthQueryConditionalNodeComputeProvider<GeometryDomain, ReadVertex, ReadFamily>,
    Q: domain::WorthQueryConditionalNodeComputeProvider<GeometryDomain, ReadVertex, ReadFamily>,
{
    let dependency_contract = first.dependencies()[0].contract().clone();
    assert_eq!(
        second.dependencies()[0].contract(),
        &dependency_contract,
        "sibling fixture requires one owner contract"
    );
    let first_identity = first.identity().to_string();
    let second_identity = second.identity().to_string();
    let (mut installation, request, snapshots) = conditional_installation_with_change(&first);
    let second_signal_node = installation.graph.node().build();
    let worth_proof::TransitionOutcome::Success(second_node) =
        installation.graph.admit_installed_node(second_signal_node)
    else {
        panic!("shared installed Signal node should remain current")
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
    let second_providers = providers_for(&second);
    let builder = runtime::WorthQueryRuntime::builder()
        .domain_package(conditional_package(vec![first, second]))
        .expect("sibling conditional package should admit")
        .graph_participation(conditional_model_graph_definition())
        .graph_participation_provider(ConditionalModelGraph, super::ConditionalModelGraphProvider)
        .conditional_signal_graph(installation.graph)
        .conditional_node(
            GeometryDomain,
            ReadVertex,
            ReadFamily,
            ConditionalModelGraph,
            domain::WorthQueryConditionalNodeLocation::operation(first_identity).unwrap(),
            vec![installation.dependency],
            installation.providers,
            first_compute,
        )
        .conditional_node(
            GeometryDomain,
            ReadVertex,
            ReadFamily,
            ConditionalModelGraph,
            domain::WorthQueryConditionalNodeLocation::operation(second_identity).unwrap(),
            vec![second_dependency],
            second_providers,
            second_compute,
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
        );
    let workspace = harness
        .configure_runtime_builder(
            builder,
            installation.bridge,
            [super::super::identity_contract(), dependency_contract],
            crate::support::public_bridge_runtime::public_graph_support_profile(),
        )
        .build_backend_from_parts()
        .build()?
        .workspace(name)?;
    Ok((workspace, request, snapshots))
}
