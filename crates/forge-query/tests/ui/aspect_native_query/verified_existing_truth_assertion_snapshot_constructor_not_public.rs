use forge_query::facade::{
    ForgeQueryAspectValue, ForgeQueryExistingTruthTargetBinding, ForgeQuerySnapshotIdentity,
    ForgeQueryVerifiedExistingTruthAssertion,
};

fn forbidden(
    binding: &ForgeQueryExistingTruthTargetBinding,
    aspects: &[ForgeQueryAspectValue],
    snapshot_identity: &ForgeQuerySnapshotIdentity,
) {
    let _ = ForgeQueryVerifiedExistingTruthAssertion::from_snapshot_identity(
        binding,
        aspects,
        snapshot_identity,
    );
}

fn main() {}
