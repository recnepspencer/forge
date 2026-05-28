use forge_query::facade::{ForgeQueryEntity, ForgeQueryWorkspace};
use schema::facade::platform::relations::TopologyRelationKind;

use super::super::shared::{entity_id_from_query_identity, relation_id_from_query_identity};
use super::scale_pressure_span::scaled_successor_span_batch;
use crate::certification::error::TopologyCertificationError;
use crate::topology_operators::{LoopEndpointKind, TopologyEditBatch, TopologyEditContract};

pub(super) fn high_cardinality_loop_batches(
    workspace: &mut ForgeQueryWorkspace,
    relation_rows: &[ForgeQueryEntity],
    moved_half_edge_identity: &str,
) -> Result<Vec<TopologyEditBatch>, TopologyCertificationError> {
    let successor_span_batch =
        scaled_successor_span_batch(workspace, relation_rows, moved_half_edge_identity)?;
    let endpoint_batch = TopologyEditBatch::new(vec![endpoint_rewire_contract(
        relation_rows,
        moved_half_edge_identity,
    )?])
    .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    Ok(vec![successor_span_batch, endpoint_batch])
}

fn endpoint_rewire_contract(
    relation_rows: &[ForgeQueryEntity],
    source_identity: &str,
) -> Result<TopologyEditContract, TopologyCertificationError> {
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
    Ok(TopologyEditContract::rewire_loop_endpoint(
        relation_id_from_query_identity(relation.identity.as_str())?,
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
        && row
            .payload
            .get("topology")
            .and_then(|value| value.get("source_identity"))
            .and_then(|value| value.as_str())
            == Some(source_identity)
}

fn relation_kind(row: &ForgeQueryEntity) -> Option<&str> {
    row.payload
        .get("topology")
        .and_then(|value| value.get("kind"))
        .and_then(|value| value.as_str())
}

fn relation_target_identity(row: &ForgeQueryEntity) -> Option<String> {
    row.payload
        .get("topology")
        .and_then(|value| value.get("target_identity"))
        .and_then(|value| value.as_str())
        .map(str::to_string)
}

fn scale_pressure_loop_error(reason: &str) -> TopologyCertificationError {
    TopologyCertificationError::Query(format!("milestone three scale loop failed: {reason}"))
}




