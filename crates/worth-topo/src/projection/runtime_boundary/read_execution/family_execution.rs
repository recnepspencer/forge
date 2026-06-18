use forge_query::facade::{
    CollectionQueryBuilder, CollectionReadOperatorQueryBuilder, CollectionResultShapeBuilder,
    ForgeQueryReadFallbackClass, ForgeQueryReadFamily, ForgeQueryReadResult,
    ForgeQueryRuntimeError, ForgeQueryWorkspace, RelationName, TraversalSelector,
};

use super::basis_context::TopologyReadExecutionTarget;
use super::query_shape::{
    identity_anchor_predicate, identity_ordering, identity_result_field, identity_selector,
    topology_kind_result_field, topology_kind_selector, TOPOLOGY_ENTITY_ROOT,
};
use crate::projection::read_views::domain::error::TopologyReadError;
use crate::projection::read_views::domain::read_proof::report::TopologyReadRequestReport;
use crate::projection::read_views::domain::request::TopologyReadRequest;
use crate::projection::runtime_boundary::read_lowering::lower_topology_read;
use crate::projection::runtime_boundary::read_lowering::schema::{
    topology_read_schema_view, TopologyDomainTraversalRelation,
};

#[derive(Clone, Copy)]
pub(crate) enum SharedNeighborhoodReadKind {
    SharedEndpoint,
    SharedAttachment,
}

pub(crate) struct ExecutedTopologyReadFamily {
    pub(crate) result: ForgeQueryReadResult,
    pub(crate) report: TopologyReadRequestReport,
}

pub(crate) fn execute_shared_neighborhood_read(
    workspace: &mut ForgeQueryWorkspace,
    execution_target: &TopologyReadExecutionTarget,
    request: &TopologyReadRequest,
    family_name: String,
    relations: [RelationName; 2],
    read_kind: SharedNeighborhoodReadKind,
    anchor_identity: &str,
) -> Result<ExecutedTopologyReadFamily, TopologyReadError> {
    let lowering_artifact = lower_topology_read(request)?;
    let schema_view = topology_read_schema_view()
        .map_err(|error| TopologyReadError::canonical_lowering_resolution(error.to_string()))?;
    let family = workspace
        .define_read_family(family_name, |read| match read_kind {
            SharedNeighborhoodReadKind::SharedEndpoint => read.local_shared_endpoint_collection(
                TOPOLOGY_ENTITY_ROOT,
                schema_view,
                relations,
                |query| identity_topology_query(query, anchor_identity),
                identity_topology_result_shape,
            ),
            SharedNeighborhoodReadKind::SharedAttachment => read
                .local_shared_attachment_collection(
                    TOPOLOGY_ENTITY_ROOT,
                    schema_view,
                    relations,
                    |query| identity_topology_query(query, anchor_identity),
                    identity_topology_result_shape,
                ),
        })
        .map_err(map_read_family_execution_error)?;
    execute_debt_free_family(
        workspace,
        execution_target,
        &family,
        lowering_artifact,
        "shared neighborhood",
    )
}

pub(crate) fn execute_loop_cycle_read(
    workspace: &mut ForgeQueryWorkspace,
    execution_target: &TopologyReadExecutionTarget,
    request: &TopologyReadRequest,
    start_identity: &str,
    cycle_depth: usize,
) -> Result<ExecutedTopologyReadFamily, TopologyReadError> {
    let lowering_artifact = lower_topology_read(request)?;
    let schema_view = topology_read_schema_view()
        .map_err(|error| TopologyReadError::canonical_lowering_resolution(error.to_string()))?;
    let family = workspace
        .define_read_family(
            format!("topology.loop_cycle_neighborhood:{start_identity}:{cycle_depth}"),
            |read| {
                if cycle_depth == 1 {
                    read.local_direct_edge_collection(
                        TOPOLOGY_ENTITY_ROOT,
                        schema_view,
                        successor_relation_name(),
                        |query| identity_topology_query(query, start_identity),
                        identity_topology_result_shape,
                    )
                } else {
                    read.explicit_broad_search_frontier_collection(
                        TOPOLOGY_ENTITY_ROOT,
                        schema_view,
                        [successor_relation_name()],
                        u8::try_from(cycle_depth).expect("supported traversal depth fits in u8"),
                        |query| identity_topology_query(query, start_identity),
                        identity_topology_result_shape,
                    )
                }
            },
        )
        .map_err(map_read_family_execution_error)?;
    execute_debt_free_family(
        workspace,
        execution_target,
        &family,
        lowering_artifact,
        "loop cycle",
    )
}

pub(crate) fn execute_local_rewire_read(
    workspace: &mut ForgeQueryWorkspace,
    execution_target: &TopologyReadExecutionTarget,
    request: &TopologyReadRequest,
    moved_identity: &str,
    cycle_depth: usize,
) -> Result<ExecutedTopologyReadFamily, TopologyReadError> {
    let lowering_artifact = lower_topology_read(request)?;
    let schema_view = topology_read_schema_view()
        .map_err(|error| TopologyReadError::canonical_lowering_resolution(error.to_string()))?;
    let family = workspace
        .define_read_family(
            format!("topology.local_rewire_neighborhood:{moved_identity}:{cycle_depth}"),
            |read| {
                read.anchored_collection(
                    TOPOLOGY_ENTITY_ROOT,
                    schema_view,
                    |query| {
                        authored_identity_topology_query(query, moved_identity)
                            .traverse(
                                TraversalSelector::bounded_relation_name(
                                    successor_relation_name(),
                                    u8::try_from(cycle_depth)
                                        .expect("supported traversal depth fits in u8"),
                                )
                                .expect("successor traversal should build"),
                            )
                            .traverse(
                                TraversalSelector::bounded_relation_name(prev_relation_name(), 1)
                                    .expect("predecessor traversal should build"),
                            )
                    },
                    identity_topology_result_shape,
                )
            },
        )
        .map_err(map_read_family_execution_error)?;
    execute_debt_free_family(
        workspace,
        execution_target,
        &family,
        lowering_artifact,
        "local rewire neighborhood",
    )
}

fn identity_topology_query(
    query: CollectionReadOperatorQueryBuilder,
    anchor_identity: &str,
) -> CollectionReadOperatorQueryBuilder {
    query
        .project(identity_selector().expect("identity selector should build"))
        .project(topology_kind_selector().expect("topology selector should build"))
        .where_equal(
            identity_anchor_predicate(anchor_identity).expect("identity anchor should build"),
        )
        .order_by(identity_ordering().expect("identity ordering should build"))
}

fn authored_identity_topology_query(
    query: CollectionQueryBuilder,
    anchor_identity: &str,
) -> CollectionQueryBuilder {
    query
        .project(identity_selector().expect("identity selector should build"))
        .project(topology_kind_selector().expect("topology selector should build"))
        .where_equal(
            identity_anchor_predicate(anchor_identity).expect("identity anchor should build"),
        )
        .order_by(identity_ordering().expect("identity ordering should build"))
}

fn identity_topology_result_shape(
    shape: CollectionResultShapeBuilder,
) -> CollectionResultShapeBuilder {
    shape
        .field(identity_result_field().expect("identity result field should build"))
        .field(topology_kind_result_field().expect("topology kind result field should build"))
}

fn execute_debt_free_family(
    workspace: &mut ForgeQueryWorkspace,
    execution_target: &TopologyReadExecutionTarget,
    family: &ForgeQueryReadFamily,
    lowering_artifact: crate::projection::runtime_boundary::read_lowering::TopologyReadLoweringArtifact,
    read_surface: &str,
) -> Result<ExecutedTopologyReadFamily, TopologyReadError> {
    let lowering_artifact =
        lowering_artifact.with_query_read_graph_relationship_proof(family.read_graph());
    let result = execution_target.execute_family(workspace, family)?;
    let receipt = result.receipt();
    require_no_query_fallback(receipt.fallback_class(), read_surface)?;
    Ok(ExecutedTopologyReadFamily {
        report: TopologyReadRequestReport::query_execution_without_fallback_debt(
            lowering_artifact,
            receipt,
        ),
        result,
    })
}

pub(crate) fn starts_at_vertex_relation_name() -> RelationName {
    TopologyDomainTraversalRelation::HalfEdgeStartsAtVertex.relation_name()
}

pub(crate) fn ends_at_vertex_relation_name() -> RelationName {
    TopologyDomainTraversalRelation::HalfEdgeEndsAtVertex.relation_name()
}

pub(crate) fn radial_next_relation_name() -> RelationName {
    TopologyDomainTraversalRelation::HalfEdgeRadialNext.relation_name()
}

pub(crate) fn uses_edge_relation_name() -> RelationName {
    TopologyDomainTraversalRelation::HalfEdgeUsesEdge.relation_name()
}

pub(crate) fn successor_relation_name() -> RelationName {
    TopologyDomainTraversalRelation::HalfEdgeNext.relation_name()
}

pub(crate) fn prev_relation_name() -> RelationName {
    TopologyDomainTraversalRelation::HalfEdgePrev.relation_name()
}

fn require_no_query_fallback(
    fallback_class: &ForgeQueryReadFallbackClass,
    read_surface: &str,
) -> Result<(), TopologyReadError> {
    if fallback_class != &ForgeQueryReadFallbackClass::None {
        return Err(TopologyReadError::read_family_execution_denied(format!(
            "{read_surface} read family unexpectedly executed with fallback `{:?}`",
            fallback_class
        )));
    }
    Ok(())
}

fn map_read_family_execution_error(error: ForgeQueryRuntimeError) -> TopologyReadError {
    TopologyReadError::read_family_execution_denied(format!("{error:?}"))
}
