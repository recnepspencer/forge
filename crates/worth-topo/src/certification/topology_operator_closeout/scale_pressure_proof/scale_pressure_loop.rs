use forge_query::facade::{ForgeQueryEntity, ForgeQueryWorkspace};
use schema::facade::platform::relations::TopologyRelationKind;

use super::super::shared::{entity_id_from_query_identity, relation_id_from_query_identity};
use super::scale_pressure_span::scaled_successor_span_declaration;
use crate::certification::error::TopologyCertificationError;
use crate::query_native_runtime_boundary::{row_text_at, TopologyNativeQueryRowField};
use crate::topology_operators::{
    LoopEndpointKind, TopologyRewireLoopEndpointDeclaration,
    TopologyRewireLoopSuccessorProgramDeclaration,
};

pub(super) fn high_cardinality_loop_declarations(
    workspace: &mut ForgeQueryWorkspace,
    relation_rows: &[ForgeQueryEntity],
    moved_half_edge_identity: &str,
) -> Result<
    (
        TopologyRewireLoopSuccessorProgramDeclaration,
        TopologyRewireLoopEndpointDeclaration,
    ),
    TopologyCertificationError,
> {
    let successor_span_declaration =
        scaled_successor_span_declaration(workspace, relation_rows, moved_half_edge_identity)?;
    let endpoint_declaration =
        endpoint_rewire_declaration(relation_rows, moved_half_edge_identity)?;
    Ok((successor_span_declaration, endpoint_declaration))
}

fn endpoint_rewire_declaration(
    relation_rows: &[ForgeQueryEntity],
    source_identity: &str,
) -> Result<TopologyRewireLoopEndpointDeclaration, TopologyCertificationError> {
    let relation = relation_rows
        .iter()
        .find(|row| {
            row_matches_source_kind(
                row,
                source_identity,
                TopologyRelationKind::HalfEdgeStartsAtVertex,
            )
        })
        .ok_or_else(|| {
            scale_pressure_loop_error("endpoint rewire source relation should resolve")
        })?;
    let current_target_identity = relation_target_identity(relation).ok_or_else(|| {
        scale_pressure_loop_error("endpoint rewire current target should resolve")
    })?;
    let alternate_target_identity = relation_rows
        .iter()
        .filter(|row| {
            relation_kind(row) == Some(TopologyRelationKind::HalfEdgeStartsAtVertex.kind_name())
        })
        .filter_map(relation_target_identity)
        .find(|target_identity| target_identity != &current_target_identity)
        .ok_or_else(|| {
            scale_pressure_loop_error("endpoint rewire alternate vertex should resolve")
        })?;
    Ok(TopologyRewireLoopEndpointDeclaration::new(
        relation_id_from_query_identity(relation.identity())?,
        LoopEndpointKind::Start,
        entity_id_from_query_identity(source_identity)?,
        entity_id_from_query_identity(&alternate_target_identity)?,
    ))
}

fn row_matches_source_kind(
    row: &ForgeQueryEntity,
    source_identity: &str,
    relation_kind: TopologyRelationKind,
) -> bool {
    self::relation_kind(row) == Some(relation_kind.kind_name())
        && row_text_at(
            row,
            TopologyNativeQueryRowField::TopologySourceIdentity.row_segments(),
        ) == Some(source_identity)
}

fn relation_kind(row: &ForgeQueryEntity) -> Option<&str> {
    row_text_at(
        row,
        TopologyNativeQueryRowField::TopologyKind.row_segments(),
    )
}

fn relation_target_identity(row: &ForgeQueryEntity) -> Option<String> {
    row_text_at(
        row,
        TopologyNativeQueryRowField::TopologyTargetIdentity.row_segments(),
    )
    .map(str::to_string)
}

fn scale_pressure_loop_error(reason: &str) -> TopologyCertificationError {
    TopologyCertificationError::Query(format!("milestone three scale loop failed: {reason}"))
}
