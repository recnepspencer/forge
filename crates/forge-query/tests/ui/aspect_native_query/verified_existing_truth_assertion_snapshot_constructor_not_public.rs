use forge_query::facade::{
    ForgeQueryAdmittedAspectValue, ForgeQueryExistingTruthTargetBinding, ForgeQuerySnapshotIdentity,
    ForgeQueryVerifiedExistingTruthAssertion,
};

fn forbidden(
    binding: &ForgeQueryExistingTruthTargetBinding,
    aspects: &[ForgeQueryAdmittedAspectValue],
    snapshot_identity: &ForgeQuerySnapshotIdentity,
) {
    let _ = ForgeQueryVerifiedExistingTruthAssertion::from_snapshot_identity(
        binding,
        aspects,
        snapshot_identity,
    );
}

fn main() {}
