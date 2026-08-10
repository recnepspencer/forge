use worth_foundational::facade::{AspectKey, FieldKey};
use worth_runtime_bridge::facade::{
    RelationalBridgeSnapshotIdentityParts, TruthBranchIdentity, TruthCommitIdentity,
    TruthPatchIdentity, TruthSnapshotIdentity,
};

pub(in crate::lower_runtime_routing::certification::surface::fixtures::phase_six) const PHASE_SIX_MAIN_BRANCH: &str = "main";
pub(super) const COMMIT_A: &str = "commit-a";
pub(super) const SNAPSHOT_A: &str = "snapshot-a";

pub(super) fn aspect_key(value: &str) -> AspectKey {
    AspectKey::new(value).expect("valid phase-six bridge aspect key")
}

pub(super) fn field_key(value: &str) -> FieldKey {
    FieldKey::new(value.to_owned()).expect("valid phase-six bridge field key")
}

pub(super) fn fixture_branch_identity(branch: &str) -> TruthBranchIdentity {
    TruthBranchIdentity::from_relational_branch_id(branch)
}

pub(super) fn fixture_commit_identity(commit: &str) -> TruthCommitIdentity {
    TruthCommitIdentity::from_relational_commit_id(match commit {
        COMMIT_A => 6,
        _ => 7,
    })
}

pub(super) fn fixture_patch_identity(commit: &str) -> TruthPatchIdentity {
    TruthPatchIdentity::from_relational_patch_position(match commit {
        COMMIT_A => 6,
        _ => 7,
    })
}

pub(super) fn fixture_snapshot_identity(snapshot: &str) -> TruthSnapshotIdentity {
    let snapshot_id = match snapshot {
        SNAPSHOT_A => 6,
        "external-snapshot" => 7,
        _ => 8,
    };
    TruthSnapshotIdentity::from_relational_snapshot(RelationalBridgeSnapshotIdentityParts::new(
        snapshot_id,
        1,
    ))
}
