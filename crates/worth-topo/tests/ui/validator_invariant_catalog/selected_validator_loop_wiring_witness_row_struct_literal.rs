use topology::facade::{
    WorthTopologyLoopWiringViolationKind, WorthTopologyLoopWiringWitnessRow,
};

fn main() {
    let _ = WorthTopologyLoopWiringWitnessRow {
        violation_kind: WorthTopologyLoopWiringViolationKind::EmptyLoop,
        validator: "loop_wiring",
        touched_loop_id: None,
        touched_half_edge_id: None,
        related_half_edge_id: None,
        message: String::new(),
        witness_digest: String::new(),
    };
}
