use forge_query::facade::{
    CollectionReadOperatorQueryBuilder, CollectionResultShapeBuilder, ForgeQueryReadResult,
    ForgeQueryRuntimeError, ForgeQueryWorkspace, RelationName,
};

use super::access_planned_execution::execute_access_planned_topology_read_family;
use super::basis_context::TopologyReadExecutionTarget;
use super::query_shape::{
    identity_anchor_predicate, identity_ordering, identity_result_field, identity_selector,
    topology_kind_result_field, topology_kind_selector, TOPOLOGY_ENTITY_ROOT,
};
use crate::projection::read_views::domain::error::TopologyReadError;
use crate::projection::read_views::domain::read_proof::report::TopologyReadRequestFamily;
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
    let query_parts = identity_topology_query_parts(anchor_identity)?;
    let result_shape = identity_topology_result_shape_parts()?;
    let family = workspace
        .define_read_family(family_name, |read| match read_kind {
            SharedNeighborhoodReadKind::SharedEndpoint => read.local_shared_endpoint_collection(
                TOPOLOGY_ENTITY_ROOT,
                schema_view,
                relations,
                |query| identity_topology_query(query, &query_parts),
                |shape| identity_topology_result_shape(shape, &result_shape),
            ),
            SharedNeighborhoodReadKind::SharedAttachment => read
                .local_shared_attachment_collection(
                    TOPOLOGY_ENTITY_ROOT,
                    schema_view,
                    relations,
                    |query| identity_topology_query(query, &query_parts),
                    |shape| identity_topology_result_shape(shape, &result_shape),
                ),
        })
        .map_err(map_read_family_execution_error)?;
    execute_access_planned_topology_read_family(
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
    let query_parts = identity_topology_query_parts(start_identity)?;
    let result_shape = identity_topology_result_shape_parts()?;
    let admitted_cycle_depth = admitted_matching_topology_traversal_depth(
        request,
        cycle_depth,
        TopologyReadRequestFamily::LoopCycleNeighborhood,
        "loop cycle",
    )?;
    let family = workspace
        .define_read_family(
            format!("topology.loop_cycle_neighborhood:{start_identity}:{cycle_depth}"),
            |read| {
                if admitted_cycle_depth == 1 {
                    read.local_direct_edge_collection(
                        TOPOLOGY_ENTITY_ROOT,
                        schema_view,
                        successor_relation_name(),
                        |query| identity_topology_query(query, &query_parts),
                        |shape| identity_topology_result_shape(shape, &result_shape),
                    )
                } else {
                    read.explicit_broad_search_frontier_collection(
                        TOPOLOGY_ENTITY_ROOT,
                        schema_view,
                        [successor_relation_name()],
                        admitted_cycle_depth,
                        |query| identity_topology_query(query, &query_parts),
                        |shape| identity_topology_result_shape(shape, &result_shape),
                    )
                }
            },
        )
        .map_err(map_read_family_execution_error)?;
    execute_access_planned_topology_read_family(
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
    let query_parts = identity_topology_query_parts(moved_identity)?;
    let result_shape = identity_topology_result_shape_parts()?;
    let admitted_cycle_depth = admitted_matching_topology_traversal_depth(
        request,
        cycle_depth,
        TopologyReadRequestFamily::LocalRewireNeighborhood,
        "local rewire neighborhood",
    )?;
    let family = workspace
        .define_read_family(
            format!("topology.local_rewire_neighborhood:{moved_identity}:{cycle_depth}"),
            |read| {
                read.explicit_broad_search_frontier_collection(
                    TOPOLOGY_ENTITY_ROOT,
                    schema_view,
                    [successor_relation_name()],
                    admitted_cycle_depth,
                    |query| identity_topology_query(query, &query_parts),
                    |shape| identity_topology_result_shape(shape, &result_shape),
                )
            },
        )
        .map_err(map_read_family_execution_error)?;
    execute_access_planned_topology_read_family(
        workspace,
        execution_target,
        &family,
        lowering_artifact,
        "local rewire neighborhood",
    )
}

#[derive(Clone)]
struct IdentityTopologyQueryParts {
    identity_selector: forge_query::facade::AspectFieldSelector,
    topology_kind_selector: forge_query::facade::AspectFieldSelector,
    identity_anchor_predicate: forge_query::facade::EqualityPredicate,
    identity_ordering: forge_query::facade::OrderingSelector,
}

#[derive(Clone)]
struct IdentityTopologyResultShapeParts {
    identity_result_field: forge_query::facade::AuthoredResultShapeField,
    topology_kind_result_field: forge_query::facade::AuthoredResultShapeField,
}

fn identity_topology_query_parts(
    anchor_identity: &str,
) -> Result<IdentityTopologyQueryParts, TopologyReadError> {
    Ok(IdentityTopologyQueryParts {
        identity_selector: identity_selector()?,
        topology_kind_selector: topology_kind_selector()?,
        identity_anchor_predicate: identity_anchor_predicate(anchor_identity)?,
        identity_ordering: identity_ordering()?,
    })
}

fn identity_topology_result_shape_parts(
) -> Result<IdentityTopologyResultShapeParts, TopologyReadError> {
    Ok(IdentityTopologyResultShapeParts {
        identity_result_field: identity_result_field()?,
        topology_kind_result_field: topology_kind_result_field()?,
    })
}

fn identity_topology_query(
    query: CollectionReadOperatorQueryBuilder,
    parts: &IdentityTopologyQueryParts,
) -> CollectionReadOperatorQueryBuilder {
    query
        .project(parts.identity_selector.clone())
        .project(parts.topology_kind_selector.clone())
        .where_equal(parts.identity_anchor_predicate.clone())
        .order_by(parts.identity_ordering.clone())
}

fn identity_topology_result_shape(
    shape: CollectionResultShapeBuilder,
    parts: &IdentityTopologyResultShapeParts,
) -> CollectionResultShapeBuilder {
    shape
        .field(parts.identity_result_field.clone())
        .field(parts.topology_kind_result_field.clone())
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

fn map_read_family_execution_error(error: ForgeQueryRuntimeError) -> TopologyReadError {
    TopologyReadError::from_query_runtime_error(error)
}

fn admitted_topology_traversal_depth(
    request: &TopologyReadRequest,
    requested_depth: usize,
) -> Result<u8, TopologyReadError> {
    u8::try_from(requested_depth).map_err(|_| {
        TopologyReadError::unsupported_traversal_depth(
            request.family(),
            requested_depth,
            usize::from(u8::MAX),
        )
    })
}

fn admitted_matching_topology_traversal_depth(
    request: &TopologyReadRequest,
    requested_depth: usize,
    expected_family: TopologyReadRequestFamily,
    read_surface: &str,
) -> Result<u8, TopologyReadError> {
    let requested_depth = admitted_topology_traversal_depth(request, requested_depth)?;
    if request.family() != expected_family {
        return Err(TopologyReadError::read_family_execution_denied(format!(
            "{read_surface} read family received `{actual:?}` request proof",
            actual = request.family()
        )));
    }
    let lowered_depth = request_lowered_traversal_depth(request);
    if requested_depth != lowered_depth {
        return Err(TopologyReadError::read_family_execution_denied(format!(
            "{read_surface} read family requested depth `{requested}` but lowered request proof carries depth `{lowered}`",
            requested = requested_depth,
            lowered = lowered_depth
        )));
    }
    Ok(requested_depth)
}

fn request_lowered_traversal_depth(request: &TopologyReadRequest) -> u8 {
    match request {
        TopologyReadRequest::LoopCycleNeighborhood { depth, .. } => *depth,
        TopologyReadRequest::LocalRewireNeighborhood { cycle_depth, .. } => *cycle_depth,
        TopologyReadRequest::WireNeighborhood { wire_depth, .. } => *wire_depth,
        TopologyReadRequest::HalfEdgeSharedVertexNeighborhood { .. }
        | TopologyReadRequest::HalfEdgeRadialNeighborhood { .. }
        | TopologyReadRequest::ShellBoundaryNeighborhood { .. } => 1,
    }
}
