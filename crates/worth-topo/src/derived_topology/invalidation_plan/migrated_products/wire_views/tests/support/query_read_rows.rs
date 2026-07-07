use forge_query::facade::{ForgeQueryGraphReadAccessAdmissionPosture, ForgeQueryReadScopeClass};
use schema::facade::platform::authority::WireInterpretationClass;

use super::super::super::{WireViewQueryReadRow, WireViewReadSource};
use super::identity::entity_id;
use super::selected_plan::{
    selected_wire_view_plan, selected_wire_view_plan_with_query_read_digest,
};
use super::touched_closure::selected_wire_view_touched_closure;
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationSelectedPlan;
use crate::projection::read_views::domain::request::TopologyReadRequest;
use crate::projection::read_views::domain::{
    read_proof::TopologyReadGraphAccessProof, TopologyReadAnchorIdentity,
    TopologyReadExecutionEngine, TopologyReadFallbackPosture, TopologyReadRequestReport,
};
use crate::projection::runtime_boundary::read_lowering::lower_topology_read;

pub(in super::super) struct WireViewReadSourceFixture {
    pub(in super::super) plan: DerivedInvalidationSelectedPlan,
    pub(in super::super) read_source: WireViewReadSource,
}

pub(in super::super) fn selected_wire_view_read_source(
    operator_family: &'static str,
) -> WireViewReadSource {
    selected_wire_view_read_source_fixture(operator_family).read_source
}

pub(in super::super) fn selected_wire_view_read_source_fixture(
    operator_family: &'static str,
) -> WireViewReadSourceFixture {
    let provisional_plan = selected_wire_view_plan(operator_family);
    let touched_closure = selected_wire_view_touched_closure(operator_family);
    let query_rows = selected_wire_view_query_read_rows();
    let provisional_source =
        WireViewReadSource::from_query_wire_views(&provisional_plan, &touched_closure, &query_rows)
            .unwrap();
    let query_report_digest = provisional_source
        .query_report_digests()
        .first()
        .expect("query wire view should expose a report digest")
        .to_string();
    let plan =
        selected_wire_view_plan_with_query_read_digest(operator_family, &query_report_digest);
    let read_source =
        WireViewReadSource::from_query_wire_views(&plan, &touched_closure, &query_rows).unwrap();
    WireViewReadSourceFixture { plan, read_source }
}

pub(in super::super) fn selected_wire_view_query_read_rows() -> Vec<WireViewQueryReadRow> {
    vec![wire_query_read_row(
        100,
        WireInterpretationClass::OpenChain,
        1,
        &[100, 101, 102, 103],
        &[101, 105],
        &[],
    )]
}

pub(in super::super) fn closed_wire_view_query_read_rows() -> Vec<WireViewQueryReadRow> {
    vec![wire_query_read_row(
        200,
        WireInterpretationClass::ClosedCycle,
        1,
        &[200, 201, 202, 203],
        &[],
        &[],
    )]
}

pub(in super::super) fn branching_wire_view_query_read_rows() -> Vec<WireViewQueryReadRow> {
    vec![wire_query_read_row(
        300,
        WireInterpretationClass::ConnectedBranch,
        1,
        &[300, 301, 302, 303, 304],
        &[301],
        &[333],
    )]
}

fn wire_query_read_row(
    wire_slot: u64,
    class: WireInterpretationClass,
    connected_component_count: usize,
    half_edge_slots: &[u64],
    terminal_vertex_slots: &[u64],
    branch_vertex_slots: &[u64],
) -> WireViewQueryReadRow {
    let first_half_edge_slot = half_edge_slots
        .first()
        .copied()
        .expect("wire query row must name at least one half-edge");
    WireViewQueryReadRow::new(
        valid_wire_query_report(first_half_edge_slot, half_edge_slots.len()),
        entity_id(wire_slot),
        class,
        connected_component_count,
        half_edge_slots.iter().copied().map(entity_id).collect(),
        terminal_vertex_slots
            .iter()
            .copied()
            .map(entity_id)
            .collect(),
        branch_vertex_slots.iter().copied().map(entity_id).collect(),
    )
}

fn valid_wire_query_report(
    source_half_edge_slot: u64,
    wire_depth: usize,
) -> TopologyReadRequestReport {
    let lowering_artifact = lower_wire_neighborhood_read(source_half_edge_slot, wire_depth);
    TopologyReadRequestReport {
        request_family: lowering_artifact.request_family(),
        lowering_artifact,
        execution_engine: TopologyReadExecutionEngine::QueryRuntimeCurrent,
        executed_scope_class: Some(ForgeQueryReadScopeClass::AnchoredExpansion),
        executed_query_digest: Some(format!("wire.query.digest.{source_half_edge_slot}")),
        executed_basis_digest: Some(format!("wire.basis.digest.{source_half_edge_slot}")),
        executed_snapshot_identity: None,
        executed_built_in_operator_coverage: Vec::new(),
        fallback_posture: TopologyReadFallbackPosture::None,
        query_execution_count: 1,
        lowered_traversal_count: 2,
        relationship_proof_admission_count: 1,
        row_scan_fallback_count: 0,
        whole_view_fallback_count: 0,
        repeated_rediscovery_denied_count: 0,
        graph_access_proof: Some(no_caller_owned_wire_graph_access_proof(
            source_half_edge_slot,
        )),
    }
}

fn lower_wire_neighborhood_read(
    source_half_edge_slot: u64,
    wire_depth: usize,
) -> crate::projection::runtime_boundary::read_lowering::TopologyReadLoweringArtifact {
    let anchor_label = format!("entity:main:{source_half_edge_slot}:1");
    let request = TopologyReadRequest::WireNeighborhood {
        source_half_edge_identity: TopologyReadAnchorIdentity::from_runtime_row_label(anchor_label),
        wire_depth: u8::try_from(wire_depth).expect("test wire depth should fit query depth"),
    };
    lower_topology_read(&request).expect("wire query report should lower canonically")
}

fn no_caller_owned_wire_graph_access_proof(
    source_half_edge_slot: u64,
) -> TopologyReadGraphAccessProof {
    TopologyReadGraphAccessProof {
        admission_posture: ForgeQueryGraphReadAccessAdmissionPosture::InlineIndexed,
        plan_digest: format!("wire.access.plan.{source_half_edge_slot}"),
        admission_digest: format!("wire.access.admission.{source_half_edge_slot}"),
        requirement_set_digest: format!("wire.access.requirements.{source_half_edge_slot}"),
        cost_estimate_digest: format!("wire.access.cost.{source_half_edge_slot}"),
        budget_digest: format!("wire.access.budget.{source_half_edge_slot}"),
        graph_index_inventory_match_report_digest: format!(
            "wire.access.inventory.{source_half_edge_slot}"
        ),
        planned_access_step_count: 1,
        consumed_access_step_count: 1,
        executor_entry_count: 1,
        executor_strategy_rediscovery_count: 0,
        edge_scan_execution_count: 0,
        per_result_neighbor_lookup_count: 0,
        persistent_artifact_bypass_count: 0,
        adjacency_buffer_build_count: 0,
        frontier_buffer_build_count: 0,
        visited_buffer_build_count: 0,
        result_buffer_build_count: 0,
    }
}
