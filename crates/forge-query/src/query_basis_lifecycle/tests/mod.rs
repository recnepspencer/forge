pub(super) use super::*;

mod basis_projection;
mod binding;
mod binding_support;
mod core;
mod core_inventory;
mod scoped;
mod support;

fn test_branch_identity(label: &str) -> forge_runtime_bridge::facade::BridgeIdentityEvidence {
    test_branch_truth_identity(label).bridge_admission_evidence()
}

fn test_snapshot_identity(label: &str) -> forge_runtime_bridge::facade::BridgeIdentityEvidence {
    test_snapshot_truth_identity(label).bridge_admission_evidence()
}

fn test_commit_identity(label: &str) -> forge_runtime_bridge::facade::BridgeIdentityEvidence {
    test_commit_truth_identity(label).bridge_admission_evidence()
}

fn test_branch_truth_identity(label: &str) -> forge_runtime_bridge::facade::TruthBranchIdentity {
    forge_runtime_bridge::facade::TruthBranchIdentity::from_relational_branch_id(label)
}

fn test_snapshot_truth_identity(
    label: &str,
) -> forge_runtime_bridge::facade::TruthSnapshotIdentity {
    forge_runtime_bridge::facade::TruthSnapshotIdentity::from_relational_snapshot(
        forge_runtime_bridge::facade::RelationalBridgeSnapshotIdentityParts::new(
            test_fixture_position("snapshot", label),
            test_fixture_position("snapshot-version", label),
        ),
    )
}

fn test_commit_truth_identity(label: &str) -> forge_runtime_bridge::facade::TruthCommitIdentity {
    forge_runtime_bridge::facade::TruthCommitIdentity::from_relational_commit_id(
        test_fixture_position("commit", label),
    )
}

fn test_preview_identity(label: &str) -> forge_runtime_bridge::facade::BridgeIdentityEvidence {
    forge_runtime_bridge::facade::BridgePreviewSessionIdentity::from_stable_name(label)
        .bridge_admission_evidence()
}

fn test_fixture_position(namespace: &str, evidence: &str) -> u64 {
    let mut acc = 14_695_981_039_346_656_037_u64;
    for byte in namespace.bytes().chain(evidence.bytes()) {
        acc ^= u64::from(byte);
        acc = acc.wrapping_mul(1_099_511_628_211_u64);
    }
    acc
}
