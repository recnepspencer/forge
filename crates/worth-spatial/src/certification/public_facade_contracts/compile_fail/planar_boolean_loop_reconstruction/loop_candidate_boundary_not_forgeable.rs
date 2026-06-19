use worth_spatial::facade::planar_boolean_loop_reconstruction::{
    PlanarBooleanDeniedLoopCandidateSet, PlanarBooleanLoopCandidateBoundary,
    PlanarBooleanLoopCandidateCounters, PlanarBooleanLoopCandidateSet,
};

fn bogus<T>() -> T {
    panic!("compile-fail fixture")
}

fn main() {
    let _ = PlanarBooleanLoopCandidateBoundary {
        loop_candidates: bogus::<PlanarBooleanLoopCandidateSet>(),
        denied_loop_candidates: bogus::<PlanarBooleanDeniedLoopCandidateSet>(),
        counters: PlanarBooleanLoopCandidateCounters::default(),
    };
}
