use worth_query::facade::policy::{assess_preview_live_drift, PreviewSessionPlanBinding, PreviewSessionQueryContext};

fn main() {
    let binding: PreviewSessionPlanBinding = todo!();
    let context: PreviewSessionQueryContext = todo!();
    let _ = assess_preview_live_drift(&binding, context);
}
