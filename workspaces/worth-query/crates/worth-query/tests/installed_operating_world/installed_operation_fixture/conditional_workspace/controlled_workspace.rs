use worth_query::facade::{consumer_kit, domain};

use super::installation::conditional_installation_pair_in_partitions;
use super::providers::DirectConditionalCompute;
use super::{
    conditional_installation, conditional_workspace_builder, conditional_workspace_with,
    GeometryDomain, ReadFamily, ReadVertex, ReadVertexExecutor,
};

pub(crate) fn conditional_controlled_workspace(
    name: &str,
    node: domain::WorthQueryPortableConditionalNodeDeclaration,
) -> Result<consumer_kit::WorthQueryControlledTestWorkspace, consumer_kit::WorthQueryTestBackendError>
{
    let installation = conditional_installation(&node);
    conditional_workspace_builder(vec![node])
        .conditional_runtime(installation.bridge, installation.graph)
        .conditional_node(
            GeometryDomain,
            ReadVertex,
            ReadFamily,
            super::ConditionalModelGraph,
            domain::WorthQueryConditionalNodeLocation::operation(installation.node_identity)
                .unwrap(),
            vec![installation.dependency],
            installation.providers,
            DirectConditionalCompute,
        )
        .domain_operation_executor(GeometryDomain, ReadVertex, ReadFamily, ReadVertexExecutor)
        .controlled_workspace(name)
}

pub(crate) struct ConditionalWorkspacePlacement<'a> {
    pub(crate) name: &'a str,
    pub(crate) partition: &'a str,
}

pub(crate) struct ConditionalDonorWorkspaceScenario<'a, P> {
    pub(crate) owner: ConditionalWorkspacePlacement<'a>,
    pub(crate) donor: ConditionalWorkspacePlacement<'a>,
    pub(crate) node: domain::WorthQueryPortableConditionalNodeDeclaration,
    pub(crate) donor_compute: P,
}

pub(crate) fn conditional_controlled_workspace_with_donor<P>(
    scenario: ConditionalDonorWorkspaceScenario<'_, P>,
) -> Result<
    (
        consumer_kit::WorthQueryControlledTestWorkspace,
        worth_query::facade::runtime::WorthQueryWorkspace,
    ),
    consumer_kit::WorthQueryTestBackendError,
>
where
    P: domain::WorthQueryConditionalNodeComputeProvider<GeometryDomain, ReadVertex, ReadFamily>,
{
    let ConditionalDonorWorkspaceScenario {
        owner: owner_placement,
        donor: donor_placement,
        node,
        donor_compute,
    } = scenario;
    let (owner_installation, donor_installation) = conditional_installation_pair_in_partitions(
        &node,
        owner_placement.partition,
        donor_placement.partition,
    );
    let owner = conditional_workspace_builder(vec![node.clone()])
        .conditional_runtime(owner_installation.bridge, owner_installation.graph)
        .conditional_node(
            GeometryDomain,
            ReadVertex,
            ReadFamily,
            super::ConditionalModelGraph,
            domain::WorthQueryConditionalNodeLocation::operation(owner_installation.node_identity)
                .unwrap(),
            vec![owner_installation.dependency],
            owner_installation.providers,
            DirectConditionalCompute,
        )
        .domain_operation_executor(GeometryDomain, ReadVertex, ReadFamily, ReadVertexExecutor)
        .controlled_workspace(owner_placement.name)?;
    let donor = conditional_workspace_with(
        donor_placement.name,
        node,
        donor_installation,
        donor_compute,
    )?;
    Ok((owner, donor))
}
