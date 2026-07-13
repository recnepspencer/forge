use worth_query::facade::runtime::{PreviewSubscriptionDiscardCloseout, PreviewSubscriptionIsolationArtifact};

fn discard(_: PreviewSubscriptionDiscardCloseout) {}

fn main() {
    let preview: PreviewSubscriptionIsolationArtifact = todo!();
    discard(preview);
}
