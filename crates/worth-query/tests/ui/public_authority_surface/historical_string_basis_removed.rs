use worth_query::facade::foundation::{HistoricalEvaluationRequest, HistoricalPathReuseDescriptor};

fn main() {
    let _ = HistoricalEvaluationRequest::retained_snapshot(
        "basis:raw",
        1,
        1,
        HistoricalPathReuseDescriptor::no_reuse(),
    );
}
