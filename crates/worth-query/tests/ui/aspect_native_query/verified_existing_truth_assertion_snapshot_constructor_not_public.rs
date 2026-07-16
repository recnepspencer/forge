use worth_query::facade::foundation::WorthQuerySnapshotIdentity;
use worth_query::facade::runtime::{WorthQueryAuthoredAspectMutation, WorthQueryExistingTruthTargetBinding, WorthQueryVerifiedExistingTruthAssertion};

fn forbidden(
    binding: &WorthQueryExistingTruthTargetBinding,
    aspects: &[WorthQueryAuthoredAspectMutation],
    snapshot_identity: &WorthQuerySnapshotIdentity,
) {
    let _ = WorthQueryVerifiedExistingTruthAssertion::from_snapshot_identity(
        binding,
        aspects,
        snapshot_identity,
    );
}

fn main() {}
