use forge_query::facade::{ForgeQueryGraphReadAccessAuthorityRequest, ForgeQueryWorkspace};

use super::access_denial::{
    PrimitiveConstructionQueryAccessDenial, PrimitiveConstructionQueryAccessError,
};
use super::access_receipt::PrimitiveConstructionPlannedQueryAccess;
use super::covered_surface::PrimitiveConstructionQueryAccessSurface;
use super::query_shape::{
    birth_digest_predicate, construction_access_schema, dependency_relation, digest_ordering,
    digest_result_field, digest_selector, root_collection, topology_class_result_field,
    topology_class_selector,
};
use crate::construction::admitted_scaffold::PreparedPrimitiveConstructionAdmittedArtifact;

pub(crate) fn plan_anchored_construction_topology_read(
    workspace: &mut ForgeQueryWorkspace,
    artifact: &PreparedPrimitiveConstructionAdmittedArtifact,
    surface: PrimitiveConstructionQueryAccessSurface,
    depth: u8,
) -> Result<PrimitiveConstructionPlannedQueryAccess, PrimitiveConstructionQueryAccessError> {
    let schema = construction_access_schema(depth)?;
    let birth_digest = source_birth_digest(artifact).to_string();
    let relation = dependency_relation()?;
    let digest_selector = digest_selector()?;
    let topology_class_selector = topology_class_selector()?;
    let digest_predicate = birth_digest_predicate(&birth_digest)?;
    let digest_ordering = digest_ordering()?;
    let digest_result_field = digest_result_field()?;
    let topology_class_result_field = topology_class_result_field()?;
    let family = workspace.define_read_family(
        format!(
            "{}:{}",
            surface.as_str(),
            artifact.admitted_handoff_digest()
        ),
        |read| {
            if depth == 1 {
                read.local_direct_edge_collection(
                    root_collection(),
                    schema,
                    relation,
                    |query| {
                        query
                            .project(digest_selector)
                            .project(topology_class_selector)
                            .where_equal(digest_predicate)
                            .order_by(digest_ordering)
                    },
                    |shape| {
                        shape
                            .field(digest_result_field)
                            .field(topology_class_result_field)
                    },
                )
            } else {
                read.local_successor_walk_collection(
                    root_collection(),
                    schema,
                    relation,
                    depth,
                    |query| {
                        query
                            .project(digest_selector)
                            .project(topology_class_selector)
                            .where_equal(digest_predicate)
                            .order_by(digest_ordering)
                    },
                    |shape| {
                        shape
                            .field(digest_result_field)
                            .field(topology_class_result_field)
                    },
                )
            }
        },
    )?;
    plan_family_in_current_head_graph_read_authority(workspace, surface, family)
}

pub(crate) fn plan_family_in_current_head_graph_read_authority(
    workspace: &ForgeQueryWorkspace,
    surface: PrimitiveConstructionQueryAccessSurface,
    family: forge_query::facade::ForgeQueryReadFamily,
) -> Result<PrimitiveConstructionPlannedQueryAccess, PrimitiveConstructionQueryAccessError> {
    let authority =
        workspace
            .admit_graph_read_access_authority(
                ForgeQueryGraphReadAccessAuthorityRequest::current_head(),
            )
            .map_err(|error| {
                PrimitiveConstructionQueryAccessError::Authority(format!(
                    "{}:{}",
                    error.kind().as_str(),
                    error.detail()
                ))
            })?;
    match workspace.plan_graph_read_access_in_authority(&family, &authority) {
        Ok(Some(plan)) => Ok(PrimitiveConstructionPlannedQueryAccess::new(
            surface, family, plan,
        )),
        Ok(None) => {
            let admission = workspace
                .admit_graph_read_access_in_authority(&family, &authority)
                .map_err(|error| {
                    PrimitiveConstructionQueryAccessError::Lowering(error.as_str().to_string())
                })?;
            Err(PrimitiveConstructionQueryAccessError::AccessDenied(
                PrimitiveConstructionQueryAccessDenial::new(admission),
            ))
        }
        Err(error) => Err(PrimitiveConstructionQueryAccessError::Lowering(
            error.as_str().to_string(),
        )),
    }
}

fn source_birth_digest(artifact: &PreparedPrimitiveConstructionAdmittedArtifact) -> &str {
    artifact
        .topology_query_admitted_handoff()
        .topology_query_handoff()
        .source_birth_digest()
}
