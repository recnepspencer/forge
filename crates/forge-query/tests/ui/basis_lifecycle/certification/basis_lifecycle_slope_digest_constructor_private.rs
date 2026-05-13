use forge_query::facade::{BasisLifecycleSlopeDigest, BasisLifecycleSlopeFamily};

fn main() {
    let _digest = BasisLifecycleSlopeDigest {
        family: BasisLifecycleSlopeFamily::Normalization,
        operation_lane: "observation",
        counter_digest: "counter".to_string(),
        bounded_by: "anything",
        slope_digest: "slope".to_string(),
    };
}
