use worth_spatial::facade::planar_boolean_loop_reconstruction::{
    PlanarBooleanDegenerateLoopOutcomeBoundary, PlanarBooleanDegenerateLoopOutcomeBoundaryCounters,
    PlanarBooleanDegenerateLoopOutcomeSet,
};

fn bogus<T>() -> T {
    panic!("compile-fail fixture")
}

fn main() {
    let _ = PlanarBooleanDegenerateLoopOutcomeBoundary {
        outcomes: bogus::<PlanarBooleanDegenerateLoopOutcomeSet>(),
        counters: PlanarBooleanDegenerateLoopOutcomeBoundaryCounters::default(),
    };
}
