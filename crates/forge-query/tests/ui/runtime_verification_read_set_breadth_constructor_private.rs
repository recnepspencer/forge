use forge_query::facade::ForgeQueryVerificationReadSetBreadth;

fn main() {
    let _ = ForgeQueryVerificationReadSetBreadth {
        target_binding_count: 1,
        asserted_aspect_count: 1,
        distinct_asserted_aspect_path_count: 1,
        cleared_assertion_count: 0,
        counter_snapshot: String::new(),
    };
}
