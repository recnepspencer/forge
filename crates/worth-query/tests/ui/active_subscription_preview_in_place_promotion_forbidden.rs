use worth_query::facade::{PreviewSubscriptionIsolationArtifact, PreviewSubscriptionLifecycleState};

fn main() {
    let mut preview: PreviewSubscriptionIsolationArtifact = todo!();
    preview.lifecycle_state = PreviewSubscriptionLifecycleState::PreviewPromoted;
}
