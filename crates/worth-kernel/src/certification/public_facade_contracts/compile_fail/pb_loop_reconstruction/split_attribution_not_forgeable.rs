use worth_spatial::facade::planar_boolean_loop_reconstruction::{
    PlanarBooleanSourceLoopSplitAttribution, PlanarBooleanSourceLoopSplitAttributionCounters,
    PlanarBooleanSourceLoopSplitAttributionRow,
};

fn bogus<T>() -> T {
    panic!("compile-fail fixture")
}

fn main() {
    let _ = PlanarBooleanSourceLoopSplitAttribution {
        attribution_identity: String::new(),
        request_identity: String::new(),
        rows: vec![bogus::<PlanarBooleanSourceLoopSplitAttributionRow>()],
        counters: PlanarBooleanSourceLoopSplitAttributionCounters::default(),
    };
}
