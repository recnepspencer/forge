use forge_query::facade::runtime::{ForgeQueryMutationBatchBuilder, ForgeQueryWorkspace};

fn forbidden(mut workspace: ForgeQueryWorkspace) {
    let _ = workspace.batch(|batch: ForgeQueryMutationBatchBuilder| batch);
}

fn main() {}
