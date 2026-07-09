use worth_query::facade::{PreviewSubscriptionDiscardCloseout, PreviewSubscriptionIsolationArtifact};

fn discard(_: PreviewSubscriptionDiscardCloseout) {}

fn main() {
    let preview: PreviewSubscriptionIsolationArtifact = todo!();
    discard(preview);
}
