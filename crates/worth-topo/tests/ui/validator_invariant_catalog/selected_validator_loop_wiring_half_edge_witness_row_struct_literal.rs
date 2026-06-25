use topology::facade::{EntityId, PartitionId, WorthTopologyLoopWiringHalfEdgeWitnessRow};

fn main() {
    let half_edge_id = EntityId::new(PartitionId::main(), 1, 1);
    let _ = WorthTopologyLoopWiringHalfEdgeWitnessRow {
        half_edge_id,
        loop_id: None,
        next_half_edge_id: None,
        prev_half_edge_id: None,
        row_digest: String::new(),
    };
}
