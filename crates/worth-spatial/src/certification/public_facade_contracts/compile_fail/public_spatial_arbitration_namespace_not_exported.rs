use worth_spatial::facade::arbitration::{
    analyze_spatial_intent_conflict, SpatialAuthoredActKind,
};

fn main() {
    let _ = analyze_spatial_intent_conflict(SpatialAuthoredActKind::Move, &[]);
}
