use worth_spatial::facade::planar_boolean_loop_reconstruction::{
    PlanarBooleanClosedWalkCandidateSet, PlanarBooleanFragmentConsumptionProof,
    PlanarBooleanWalkOutcomeSetInput,
};

fn bogus<T>() -> T {
    panic!("compile-fail fixture")
}

fn main() {
    let _ = PlanarBooleanWalkOutcomeSetInput {
        closed_walk_candidates: bogus::<&PlanarBooleanClosedWalkCandidateSet>(),
        fragment_consumption_proof: bogus::<&PlanarBooleanFragmentConsumptionProof>(),
    };
}
