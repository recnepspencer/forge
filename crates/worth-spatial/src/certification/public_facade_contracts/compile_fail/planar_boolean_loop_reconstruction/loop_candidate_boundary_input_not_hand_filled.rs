use worth_spatial::facade::planar_boolean_loop_reconstruction::{
    PlanarBooleanLoopCandidateBoundaryInput, PlanarBooleanWalkOutcomeSet,
};

fn bogus<T>() -> T {
    panic!("compile-fail fixture")
}

fn main() {
    let _ = PlanarBooleanLoopCandidateBoundaryInput {
        walk_outcomes: bogus::<&PlanarBooleanWalkOutcomeSet>(),
    };
}
