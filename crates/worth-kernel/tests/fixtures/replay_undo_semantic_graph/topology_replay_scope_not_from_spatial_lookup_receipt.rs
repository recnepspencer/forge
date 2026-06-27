use topology::facade::{lower_topology_replay_scope_identity, TopologyTouchedGraphBasis};
use worth_spatial::facade::evidence_lookup_execution::EvidenceLookupExecutionReceipt;

fn main() {}

fn rejects_spatial_lookup_receipt_for_topology_replay_scope(
    touched_graph_basis: &TopologyTouchedGraphBasis,
    spatial_lookup_receipt: &EvidenceLookupExecutionReceipt,
) {
    let _ = lower_topology_replay_scope_identity(touched_graph_basis, spatial_lookup_receipt);
}
