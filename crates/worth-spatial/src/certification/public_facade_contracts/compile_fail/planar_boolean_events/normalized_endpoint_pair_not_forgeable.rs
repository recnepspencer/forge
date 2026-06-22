use worth_spatial::facade::planar_boolean_events::{
    PlanarBooleanNormalizedEndpoint, PlanarBooleanNormalizedEndpointPair,
};

fn main() {
    let _ = PlanarBooleanNormalizedEndpointPair {
        low: unavailable_endpoint(),
        high: unavailable_endpoint(),
        orientation_was_reversed: false,
    };
}

fn unavailable_endpoint() -> PlanarBooleanNormalizedEndpoint {
    panic!("compile-fail fixture must never construct normalized endpoint facts")
}
