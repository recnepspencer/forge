use forge_relational::facade::identity::EntityId;
use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::platform::authority::CreateKey;
use schema::facade::platform::entities::{EntityKind, TopologyEntityKind};
use schema::facade::platform::relations::{RelationKind, TopologyRelationKind};
use schema::facade::topology_authoring::{DerivedTopologyReadBasis, MilestoneOnePrimitiveCase};

use super::super::mutation_sequence_support::{
    aggregate_topology_mutation_digest_for_declarations, TopologyCloseoutDeclaration,
};
use super::super::shared::derived_validation_report_from_materialized;
use super::primitive_family_closure::{primitive_family_closure_error, PrimitiveClosureExecution};
use crate::certification::error::TopologyCertificationError;
use crate::certification::shared::primitive_family_name;
use crate::certification::support::declaration_runtime::execute_current_head_topology_declaration;
use crate::certification::support::parity::digest_materialized_topology_view;
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::test_support::schema_topology_authoring_boundary::seed_milestone_one_primitive_through_schema_execution;
use crate::topology_operators::application::TopologyDeclarationMutationPayload;
use crate::topology_operators::{
    TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration,
    TopologySplitConnectedHalfEdgeSetToNewWireDeclaration, TopologyWireRehomeHalfEdgeMember,
    TopologyWireSplitHalfEdgeMember,
};

pub(in crate::certification::topology_operator_closeout) fn execute_wire_split_collapse_primitive_closure<
    F,
>(
    runtime_factory: &mut F,
    stem: &str,
    primitive: MilestoneOnePrimitiveCase,
    split_offset: usize,
) -> Result<PrimitiveClosureExecution, TopologyCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let primitive_family = primitive_family_name(&primitive).to_string();
    let mut runtime = runtime_factory();
    let verified = seed_milestone_one_primitive_through_schema_execution(
        &mut runtime,
        &format!("{stem}.primitive_family_closure.{primitive_family}"),
        &primitive,
    )?;
    let half_edge_ids = seeded_wire_half_edges(&runtime, &verified.read_basis())?;
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(
        adapters,
        format!("{stem}.primitive_family_closure.{primitive_family}.runtime"),
    )
    .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let moved_half_edge_ids = moved_wire_split_segment(&half_edge_ids, split_offset)?;
    let split_wire_key = CreateKey::new(format!(
        "{stem}.primitive_family_closure.{primitive_family}.split_wire"
    ));
    let split_declaration = split_wire_declaration(split_wire_key.as_str(), &moved_half_edge_ids)?;
    let split_execution = execute_current_head_topology_declaration(
        &mut workspace,
        &surfaces,
        split_declaration.clone(),
    )
    .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let split_wire_id = split_execution
        .materialized()
        .topology()
        .wires
        .iter()
        .find(|wire| wire.label == split_wire_key.as_str())
        .map(|wire| wire.entity_id)
        .ok_or_else(|| {
            primitive_family_closure_error("wire split closure should materialize split wire")
        })?;
    let collapse_wire_key = CreateKey::new(format!(
        "{stem}.primitive_family_closure.{primitive_family}.collapse_wire"
    ));
    let collapse_declaration = rehome_wire_declaration(
        collapse_wire_key.as_str(),
        &moved_half_edge_ids,
        Some(split_wire_id),
    )?;
    let collapse_execution = execute_current_head_topology_declaration(
        &mut workspace,
        &surfaces,
        collapse_declaration.clone(),
    )
    .map_err(|error| TopologyCertificationError::Query(error.to_string()))?;
    let validation =
        derived_validation_report_from_materialized(collapse_execution.materialized())?;
    let mutation_families = split_declaration
        .semantic_families()
        .into_iter()
        .chain(collapse_declaration.semantic_families())
        .collect();
    Ok(PrimitiveClosureExecution {
        primitive_family,
        mutation_families,
        topology_mutation_digest: aggregate_topology_mutation_digest_for_declarations(vec![
            TopologyCloseoutDeclaration::SplitConnectedHalfEdgeSetToNewWire(split_declaration),
            TopologyCloseoutDeclaration::RehomeAllOwnedHalfEdgesToNewWire(collapse_declaration),
        ]),
        final_materialized_topology_digest: digest_materialized_topology_view(
            collapse_execution.materialized(),
        ),
        derived_validation_row_count: validation.rows.len(),
    })
}

fn split_wire_declaration(
    wire_key: &str,
    moved_half_edge_ids: &[EntityId],
) -> Result<TopologySplitConnectedHalfEdgeSetToNewWireDeclaration, TopologyCertificationError> {
    Ok(TopologySplitConnectedHalfEdgeSetToNewWireDeclaration::new(
        wire_key.to_string(),
        moved_half_edge_ids
            .iter()
            .enumerate()
            .map(|(index, half_edge_id)| {
                TopologyWireSplitHalfEdgeMember::new(
                    format!("{wire_key}.owns_half_edge_{}", index + 1),
                    *half_edge_id,
                )
            })
            .collect(),
    ))
}

fn rehome_wire_declaration(
    wire_key: &str,
    moved_half_edge_ids: &[EntityId],
    retired_wire_id: Option<EntityId>,
) -> Result<TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration, TopologyCertificationError> {
    let retired_wire_id = retired_wire_id.ok_or_else(|| {
        primitive_family_closure_error("wire collapse closure requires a retired source wire")
    })?;
    Ok(TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration::new(
        wire_key.to_string(),
        retired_wire_id,
        moved_half_edge_ids
            .iter()
            .enumerate()
            .map(|(index, half_edge_id)| {
                TopologyWireRehomeHalfEdgeMember::new(
                    format!("{wire_key}.owns_half_edge_{}", index + 1),
                    *half_edge_id,
                )
            })
            .collect(),
    ))
}

fn moved_wire_split_segment(
    half_edge_ids: &[EntityId],
    split_offset: usize,
) -> Result<Vec<EntityId>, TopologyCertificationError> {
    if split_offset == 0 || split_offset >= half_edge_ids.len() {
        return Err(primitive_family_closure_error(
            "wire split closure requires retained and moved half-edge segments",
        ));
    }
    Ok(half_edge_ids[split_offset..].to_vec())
}

fn seeded_wire_half_edges(
    runtime: &RelationalRuntime,
    read_basis: &DerivedTopologyReadBasis,
) -> Result<Vec<EntityId>, TopologyCertificationError> {
    let read_view = runtime
        .read_truth()
        .read_snapshot(read_basis.snapshot())
        .ok_or_else(|| {
            primitive_family_closure_error("seeded wire-open snapshot should remain readable")
        })?;
    let wire = read_view
        .entities()
        .iter()
        .find(|record| {
            EntityKind::from_kind_id(record.kind.kind_id)
                == Some(EntityKind::Topology(TopologyEntityKind::Wire))
        })
        .map(|record| record.entity_id)
        .ok_or_else(|| {
            primitive_family_closure_error("wire-open closure primitive should contain a wire")
        })?;
    let mut half_edge_ids = read_view
        .relations()
        .iter()
        .filter(|record| {
            record.source == wire
                && RelationKind::from_kind_id(record.kind.kind_id)
                    == Some(RelationKind::Topology(
                        TopologyRelationKind::WireOwnsHalfEdge,
                    ))
        })
        .map(|record| record.target)
        .collect::<Vec<_>>();
    half_edge_ids.sort();
    Ok(half_edge_ids)
}
