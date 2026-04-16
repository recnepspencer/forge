use forge_relational::facade::history::CommitId;
use forge_store::{BasisFreeCheckpoint, DerivedDurableCheckpointKind, NoContainedCommits};

fn main() {
    let _ = BasisFreeCheckpoint::<DerivedDurableCheckpointKind, NoContainedCommits>::new(
        "checkpoint-free",
        "embedded-runtime",
    )
    .with_basis_commit(CommitId(1));
}
