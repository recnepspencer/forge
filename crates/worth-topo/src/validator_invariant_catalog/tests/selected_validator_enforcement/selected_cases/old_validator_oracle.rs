use crate::brep::topology_graph::{TopologyHalfEdge, TopologyLoop, TopologyView};
use crate::derived_topology::materialized_graph::MaterializedTopologyView;
use crate::validation::loop_wiring_validation_oracle;
use crate::validator_invariant_catalog::WorthTopologyLoopWiringWitnessInput;

use super::admitted_facts::witness_input_from_admitted_facts;
use crate::validator_invariant_catalog::WorthTopologyLoopWiringAdmittedLocalFacts;

pub(in crate::validator_invariant_catalog::tests::selected_validator_enforcement) fn old_loop_wiring_oracle_passes(
    witness_input: &WorthTopologyLoopWiringWitnessInput,
) -> bool {
    loop_wiring_validation_oracle(&materialized_oracle_view(witness_input)).is_ok()
}

pub(in crate::validator_invariant_catalog::tests::selected_validator_enforcement) fn old_loop_wiring_oracle_error_validator(
    witness_input: &WorthTopologyLoopWiringWitnessInput,
) -> Option<&'static str> {
    loop_wiring_validation_oracle(&materialized_oracle_view(witness_input))
        .err()
        .map(|error| error.validator())
}

pub(in crate::validator_invariant_catalog::tests::selected_validator_enforcement) fn whole_view_oracle_passes_with_unrelated_broken_loop(
    admitted_facts: &WorthTopologyLoopWiringAdmittedLocalFacts,
) -> bool {
    let witness_input = witness_input_from_admitted_facts(admitted_facts);
    let mut topology = topology_view_from_witness(&witness_input);
    topology.loops.push(TopologyLoop {
        entity_id: entity_id(10_000),
        label: "outside-broken-loop".to_string(),
        face_ids: Vec::new(),
        half_edge_ids: Vec::new(),
    });
    loop_wiring_validation_oracle(&MaterializedTopologyView::whole_view(topology)).is_ok()
}

fn materialized_oracle_view(
    witness_input: &WorthTopologyLoopWiringWitnessInput,
) -> MaterializedTopologyView {
    MaterializedTopologyView::whole_view(topology_view_from_witness(witness_input))
}

fn topology_view_from_witness(witness_input: &WorthTopologyLoopWiringWitnessInput) -> TopologyView {
    let mut topology = TopologyView::default();
    topology.loops = witness_input
        .loop_rows()
        .iter()
        .map(|row| TopologyLoop {
            entity_id: row.loop_id(),
            label: format!("loop-{:?}", row.loop_id()),
            face_ids: Vec::new(),
            half_edge_ids: row.half_edge_ids().to_vec(),
        })
        .collect();
    topology.half_edges = witness_input
        .half_edge_rows()
        .iter()
        .map(|row| TopologyHalfEdge {
            entity_id: row.half_edge_id(),
            label: format!("half-edge-{:?}", row.half_edge_id()),
            loop_id: row.loop_id(),
            wire_id: None,
            next_half_edge_id: row.next_half_edge_id(),
            prev_half_edge_id: row.prev_half_edge_id(),
            radial_next_half_edge_id: None,
            edge_id: None,
            origin_vertex_id: None,
            target_vertex_id: None,
            face_id: None,
        })
        .collect();
    topology
}

fn entity_id(slot: u64) -> forge_relational::facade::identity::EntityId {
    forge_relational::facade::identity::EntityId::new(
        forge_relational::facade::identity::PartitionId::main(),
        slot,
        1,
    )
}
