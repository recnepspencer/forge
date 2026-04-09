use forge_harness::facade::ScenarioPlan;

use crate::harness::fixtures::BridgeHarnessFixture;

use super::super::support::{committed_patch, registration, snapshot};

pub(super) fn stream_fixture(
    name: &str,
) -> forge_harness::facade::ScenarioFixture<BridgeHarnessFixture> {
    ScenarioPlan::new(
        name,
        BridgeHarnessFixture::new(vec![registration()])
            .with_policy(crate::facade::BridgeRuntimePolicy::development())
            .with_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"))
            .with_committed_patch(committed_patch("commit-b", "patch-b", "snapshot-a", "name"))
            .with_snapshot(snapshot("snapshot-a", "alice")),
    )
    .declare_input("stream")
    .declare_observation("stream")
    .compile()
}

pub(super) fn routing_target() -> String {
    "stream-routing:commit-a,commit-b".to_string()
}

pub(super) fn replay_audit_target() -> String {
    "stream-replay-audit:commit-a,commit-b".to_string()
}
