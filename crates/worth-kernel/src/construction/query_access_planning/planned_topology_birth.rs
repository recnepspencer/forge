use forge_query::facade::ForgeQueryWorkspace;

use super::access_denial::PrimitiveConstructionQueryAccessError;
use super::access_receipt::{
    PrimitiveConstructionConsumedQueryAccess, PrimitiveConstructionPlannedQueryAccess,
};
use super::anchored_topology_read::{
    plan_anchored_construction_topology_read, plan_family_in_current_head_graph_read_authority,
};
use super::covered_surface::PrimitiveConstructionQueryAccessSurface;
use super::planned_access_execution::execute_planned_construction_query_access;
use super::query_shape::{
    construction_access_schema, digest_ordering, digest_result_field, digest_selector,
    map_authoring_error, root_collection, topology_class_result_field, topology_class_selector,
};
use crate::construction::admitted_scaffold::prepare_primitive_construction_admitted_artifact;
use crate::construction::admitted_scaffold::PreparedPrimitiveConstructionAdmittedArtifact;
use crate::construction::request::PrimitiveConstructionRequest;

pub(crate) fn plan_topology_birth(
    workspace: &mut ForgeQueryWorkspace,
    artifact: &PreparedPrimitiveConstructionAdmittedArtifact,
) -> Result<PrimitiveConstructionPlannedQueryAccess, PrimitiveConstructionQueryAccessError> {
    plan_anchored_construction_topology_read(
        workspace,
        artifact,
        PrimitiveConstructionQueryAccessSurface::TopologyBirth,
        1,
    )
}

pub(crate) fn plan_topology_birth_broad_scan(
    workspace: &mut ForgeQueryWorkspace,
    artifact: &PreparedPrimitiveConstructionAdmittedArtifact,
    depth: u8,
) -> Result<PrimitiveConstructionPlannedQueryAccess, PrimitiveConstructionQueryAccessError> {
    let schema = construction_access_schema(depth)?;
    let traversal = forge_query::facade::TraversalSelector::bounded_relation_name(
        super::query_shape::dependency_relation()?,
        depth,
    )
    .map_err(|error| map_authoring_error("traversal", error))?;
    let digest_selector = digest_selector()?;
    let topology_class_selector = topology_class_selector()?;
    let topology_class_presence =
        forge_query::facade::PresencePredicate::is_present("topology", "kind")
            .map_err(|error| map_authoring_error("topology-class-presence", error))?;
    let digest_ordering = digest_ordering()?;
    let digest_result_field = digest_result_field()?;
    let topology_class_result_field = topology_class_result_field()?;
    let family = workspace.define_read_family(
        format!(
            "{}:{}:{depth}",
            PrimitiveConstructionQueryAccessSurface::TopologyBirthBroadScan.as_str(),
            artifact.admitted_handoff_digest()
        ),
        |read| {
            read.explicit_broad_search_collection(
                root_collection(),
                schema,
                |query| {
                    query
                        .project(digest_selector)
                        .project(topology_class_selector)
                        .where_present(topology_class_presence)
                        .order_by(digest_ordering)
                        .traverse(traversal)
                },
                |shape| {
                    shape
                        .field(digest_result_field)
                        .field(topology_class_result_field)
                },
            )
        },
    )?;
    plan_family_in_current_head_graph_read_authority(
        workspace,
        PrimitiveConstructionQueryAccessSurface::TopologyBirthBroadScan,
        family,
    )
}

pub(crate) fn execute_planned_topology_birth(
    workspace: &mut ForgeQueryWorkspace,
    planned: PrimitiveConstructionPlannedQueryAccess,
) -> Result<PrimitiveConstructionConsumedQueryAccess, PrimitiveConstructionQueryAccessError> {
    execute_planned_construction_query_access(workspace, planned)
}

pub(crate) fn execute_topology_birth_query_access_for_request(
    workspace: &mut ForgeQueryWorkspace,
    request: &PrimitiveConstructionRequest,
) -> Result<PrimitiveConstructionConsumedQueryAccess, PrimitiveConstructionQueryAccessError> {
    let admitted = prepare_primitive_construction_admitted_artifact(request)
        .map_err(|error| PrimitiveConstructionQueryAccessError::Lowering(error.to_string()))?;
    let planned = plan_topology_birth(workspace, &admitted)?;
    execute_planned_topology_birth(workspace, planned)
}
