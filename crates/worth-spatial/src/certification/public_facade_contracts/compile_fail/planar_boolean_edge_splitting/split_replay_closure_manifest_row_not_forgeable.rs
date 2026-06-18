use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanSplitReplayClosureRow, PlanarBooleanSplitReplayClosureRowKind,
};

fn main() {
    let _row = PlanarBooleanSplitReplayClosureRow {
        kind: PlanarBooleanSplitReplayClosureRowKind::SplitLedgerDigest,
        row_identity: String::new(),
        original_identity: String::new(),
        replayed_identity: String::new(),
    };
}
