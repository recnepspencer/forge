use forge_query::facade::{
    ForgeQueryVerificationReadSetBreadth, ForgeQueryVerifiedAssumptionSet,
};

fn main() {
    let read_set = ForgeQueryVerificationReadSetBreadth {
        target_binding_count: 1,
        asserted_aspect_count: 1,
        distinct_asserted_aspect_touch_count: 1,
        cleared_assertion_count: 0,
        counter_snapshot: String::new(),
    };
    let _ = ForgeQueryVerifiedAssumptionSet {
        binding_digest: String::new(),
        asserted_aspects: Vec::new(),
        assumption_snapshot_token: String::new(),
        assumption_snapshot_digest: String::new(),
        verified_precondition_digest: String::new(),
        verification_read_set_breadth: read_set,
        verified_assumption_digest: String::new(),
    };
}
