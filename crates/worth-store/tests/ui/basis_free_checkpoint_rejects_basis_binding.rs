use worth_relational::facade::history::CommitId;
use worth_store::{BasisFreeCheckpoint, DerivedDurableCheckpointKind, NoContainedCommits};

fn main() {
    let _ = BasisFreeCheckpoint::<DerivedDurableCheckpointKind, NoContainedCommits>::new(
        "checkpoint-free",
        "embedded-runtime",
    )
    .with_basis_commit(CommitId(1));
}
