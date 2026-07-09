use worth_harness::facade::ScenarioPlan;

use crate::facade::TruthCommitIdentity;
use crate::harness::adapter::BridgeHarnessTargetId;
use crate::harness::fixtures::BridgeHarnessFixture;

use super::super::support::{committed_patch, registration, snapshot};

pub(super) fn stream_fixture(
    name: &str,
) -> worth_harness::facade::ScenarioFixture<BridgeHarnessFixture> {
    ScenarioPlan::new(
        name,
        BridgeHarnessFixture::new(vec![registration()])
            .with_policy(crate::facade::BridgeRuntimePolicy::development())
            .with_committed_patch(committed_patch(
                crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
                crate::truth_identity_fixtures::truth_patch_fixture("patch-a"),
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
                worth_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid harness field key"),
            ))
            .with_committed_patch(committed_patch(
                crate::truth_identity_fixtures::truth_commit_fixture("commit-b"),
                crate::truth_identity_fixtures::truth_patch_fixture("patch-b"),
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
                worth_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid harness field key"),
            ))
            .with_snapshot(snapshot(
                crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
                "alice",
            )),
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
        crate::truth_identity_fixtures::truth_commit_fixture("commit-a"),
        crate::truth_identity_fixtures::truth_commit_fixture("commit-b"),
    ]
}
