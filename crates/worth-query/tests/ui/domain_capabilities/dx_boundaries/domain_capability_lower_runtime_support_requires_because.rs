use worth_query::facade::runtime::{WorthQueryLowerRuntimeBoundaryBoundContributionTarget, WorthQueryLowerRuntimeSupportDraft};

fn main() {
    let _ = WorthQueryLowerRuntimeSupportDraft {
        domain: "worth.spatial".to_string(),
        target: unsafe { std::mem::zeroed::<WorthQueryLowerRuntimeBoundaryBoundContributionTarget>() },
        semantic_code: "routing.signal_invalidation".to_string(),
    };
}
