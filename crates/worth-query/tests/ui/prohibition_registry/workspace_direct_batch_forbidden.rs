use worth_query::facade::runtime::{WorthQueryMutationBatchBuilder, WorthQueryWorkspace};

fn forbidden(mut workspace: WorthQueryWorkspace) {
    let _ = workspace.batch(|batch: WorthQueryMutationBatchBuilder| batch);
}

fn main() {}
