use topology::facade::{EntityId, PartitionId, WorthTopologyLoopWiringLoopWitnessRow};

fn main() {
    let loop_id = EntityId::new(PartitionId::main(), 1, 1);
    let _ = WorthTopologyLoopWiringLoopWitnessRow {
        loop_id,
        half_edge_ids: Vec::new(),
        row_digest: String::new(),
    };
}
