use worth_query::facade::runtime::{PreviewSubscriptionIsolationArtifact, PreviewSubscriptionLifecycleState};

fn main() {
    let mut preview: PreviewSubscriptionIsolationArtifact = todo!();
    preview.lifecycle_state = PreviewSubscriptionLifecycleState::PreviewPromoted;
}
