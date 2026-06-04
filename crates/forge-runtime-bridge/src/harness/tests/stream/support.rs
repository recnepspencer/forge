use crate::facade::TruthSnapshotIdentity;
use forge_harness::facade::ScenarioPlan;

use crate::facade::TruthCommitIdentity;
use crate::harness::adapter::BridgeHarnessTargetId;
use crate::harness::fixtures::BridgeHarnessFixture;

use super::super::support::{committed_patch, registration, snapshot};

pub(super) fn stream_fixture(
    name: &str,
) -> forge_harness::facade::ScenarioFixture<BridgeHarnessFixture> {
    ScenarioPlan::new(
        name,
        BridgeHarnessFixture::new(vec![registration()])
            .with_policy(crate::facade::BridgeRuntimePolicy::development())
            .with_committed_patch(committed_patch(
                crate::facade::TruthCommitIdentity::new("commit-a"),
                crate::facade::TruthPatchIdentity::new("patch-a"),
                TruthSnapshotIdentity::new("snapshot-a"),
                forge_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid harness field key"),
            ))
            .with_committed_patch(committed_patch(
                crate::facade::TruthCommitIdentity::new("commit-b"),
                crate::facade::TruthPatchIdentity::new("patch-b"),
                TruthSnapshotIdentity::new("snapshot-a"),
                forge_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid harness field key"),
            ))
            .with_snapshot(snapshot(TruthSnapshotIdentity::new("snapshot-a"), "alice")),
    )
    .declare_input("stream")
    .declare_observation("stream")
    .compile()
}

pub(super) fn routing_target() -> BridgeHarnessTargetId {
    BridgeHarnessTargetId::stream_routing(stream_commit_window())
}

pub(super) fn replay_audit_target() -> BridgeHarnessTargetId {
    BridgeHarnessTargetId::stream_replay_audit(stream_commit_window())
}

fn stream_commit_window() -> [TruthCommitIdentity; 2] {
    [
        TruthCommitIdentity::new("commit-a"),
        TruthCommitIdentity::new("commit-b"),
    ]
}
