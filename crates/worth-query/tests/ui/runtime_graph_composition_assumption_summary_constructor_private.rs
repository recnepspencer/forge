use worth_query::facade::WorthQueryGraphCompositionAssumptionSummary;
use worth_query::facade::WorthQueryVerificationReadSetBreadth;

fn main() {
    let _ = WorthQueryGraphCompositionAssumptionSummary {
        assumption_snapshot_digests: Vec::new(),
        verified_precondition_digests: Vec::new(),
        verified_step_count: 0,
        verification_read_set_breadth: WorthQueryVerificationReadSetBreadth {
            target_binding_count: 0,
            asserted_aspect_count: 0,
            distinct_asserted_aspect_touch_count: 0,
            cleared_assertion_count: 0,
            counter_snapshot: String::new(),
        },
        counter_snapshot: String::new(),
        aggregate_assumption_snapshot_digest: String::new(),
        aggregate_verified_precondition_digest: String::new(),
        assumption_summary_digest: String::new(),
    };
}
