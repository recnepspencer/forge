use forge_query::facade::runtime::{
    ForgeQueryLowerRuntimeBoundaryBoundContributionTarget, ForgeQueryLowerRuntimeSupportDraft,
};

fn main() {
    let _ = ForgeQueryLowerRuntimeSupportDraft {
        domain: "worth.spatial".to_string(),
        target: unsafe { std::mem::zeroed::<ForgeQueryLowerRuntimeBoundaryBoundContributionTarget>() },
        semantic_code: "routing.signal_invalidation".to_string(),
    };
}
