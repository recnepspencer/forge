use worth_spatial::facade::planar_boolean_events::{
    PlanarBooleanEventClassifierInput, PlanarBooleanPredicateBoundPair,
};

fn unreachable_bound_pair() -> &'static PlanarBooleanPredicateBoundPair {
    panic!("compile-fail fixture never executes")
}

fn main() {
    let _ = PlanarBooleanEventClassifierInput {
        bound_pair: unreachable_bound_pair(),
    };
}
