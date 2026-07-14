use worth_query::facade::runtime::{PreviewResidueWidth, PreviewSubscriptionIsolationArtifact, PreviewSubscriptionLifecycleState};

fn main() {
    let _ = PreviewSubscriptionIsolationArtifact {
        active_lane_digest: todo!(),
        attachment_digest: todo!(),
        preview_epoch_digest: "epoch".to_string(),
        lifecycle_state: PreviewSubscriptionLifecycleState::PreviewActive,
        preview_residue_budget_width: PreviewResidueWidth::measured(1),
        counters: todo!(),
        isolation_digest: "isolation".to_string(),
    };
}
