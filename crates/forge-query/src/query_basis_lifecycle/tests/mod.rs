pub(super) use super::*;

mod basis_projection;
mod binding;
mod binding_support;
mod core;
mod core_inventory;
mod scoped;
mod support;

fn test_branch_identity(label: &str) -> forge_runtime_bridge::facade::BridgeIdentityEvidence {
    forge_runtime_bridge::facade::TruthBranchIdentity::from_bridge_harness_label(label)
        .evidence_identity()
}

fn test_snapshot_identity(label: &str) -> forge_runtime_bridge::facade::BridgeIdentityEvidence {
    forge_runtime_bridge::facade::TruthSnapshotIdentity::from_bridge_harness_label(label)
        .evidence_identity()
}

fn test_commit_identity(label: &str) -> forge_runtime_bridge::facade::BridgeIdentityEvidence {
    forge_runtime_bridge::facade::TruthCommitIdentity::from_bridge_harness_label(label)
        .evidence_identity()
}

fn test_preview_identity(label: &str) -> forge_runtime_bridge::facade::BridgeIdentityEvidence {
    forge_runtime_bridge::facade::BridgePreviewSessionIdentity::from_stable_name(label)
        .evidence_identity()
}
