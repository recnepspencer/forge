use worth_query::facade::WorthQueryVerificationReadSetBreadth;

fn main() {
    let _ = WorthQueryVerificationReadSetBreadth {
        target_binding_count: 1,
        asserted_aspect_count: 1,
        distinct_asserted_aspect_touch_count: 1,
        cleared_assertion_count: 0,
        counter_snapshot: String::new(),
    };
}
