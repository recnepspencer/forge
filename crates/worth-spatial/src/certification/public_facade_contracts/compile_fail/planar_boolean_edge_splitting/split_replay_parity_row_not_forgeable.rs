use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanEdgeSplitReplayParityRow, PlanarBooleanEdgeSplitReplayParityRowKind,
};

fn main() {
    let _row = PlanarBooleanEdgeSplitReplayParityRow {
        kind: PlanarBooleanEdgeSplitReplayParityRowKind::DecisionLogReceipt,
        parity_row_identity: String::new(),
        original_identity: String::new(),
        replayed_identity: String::new(),
    };
}
