use worth_query::facade::foundation::WorthQuerySnapshotIdentity;
use worth_query::facade::runtime::{WorthQueryAdmittedAspectValue, WorthQueryExistingTruthTargetBinding, WorthQueryVerifiedExistingTruthAssertion};

fn forbidden(
    binding: &WorthQueryExistingTruthTargetBinding,
    aspects: &[WorthQueryAdmittedAspectValue],
    snapshot_identity: &WorthQuerySnapshotIdentity,
) {
    let _ = WorthQueryVerifiedExistingTruthAssertion::from_snapshot_identity(
        binding,
        aspects,
        snapshot_identity,
    );
}

fn main() {}
