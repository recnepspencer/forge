use topology::facade::{EdgeSplitValidatorRow, EdgeSplitValidatorRuntimeLane};

fn main() {
    let _ = EdgeSplitValidatorRow {
        validator_name: "FakeSplitValidator",
        runtime_lane: EdgeSplitValidatorRuntimeLane::SpatialPreparedProductValidation,
        governs_topology_legality: false,
        proof_obligations: &[],
    };
}
