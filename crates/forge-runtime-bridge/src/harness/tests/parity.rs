use forge_harness::facade::{
    parity_suite, ExecutionProfile, ExecutionRequest, ScenarioPlan,
};

use crate::harness::adapter::BridgeHarnessAdapter;
use crate::harness::fixtures::BridgeHarnessFixture;
use super::support::{
    committed_patch, field_aspect_registration, field_slice_snapshot, registration, snapshot,
};

#[test]
fn bridge_harness_parity_proves_routing_truth_is_invariant_across_diagnostics_tiers() {
    let fixture = ScenarioPlan::new(
        "bridge-parity",
        BridgeHarnessFixture::new(vec![registration()])
            .with_policy(crate::facade::BridgeRuntimePolicy::development())
            .with_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"))
            .with_snapshot(snapshot("snapshot-a", "alice")),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();
    let request = ExecutionRequest::target("deliver-commit-a", "commit-a".to_string());

    let report = parity_suite(
        BridgeHarnessAdapter,
        fixture,
        request,
        ExecutionProfile::development("baseline"),
    )
    .candidates([
        ExecutionProfile::operational("operational"),
        ExecutionProfile::forensic("forensic"),
    ])
    .compare()
    .expect("bridge parity suite should compare cleanly");

    assert!(report.matched);
    assert_eq!(report.results.len(), 2);
}

#[test]
fn bridge_harness_parity_proves_fine_grained_slice_truth_is_invariant_across_diagnostics_tiers() {
    let fixture = ScenarioPlan::new(
        "bridge-fine-grained-parity",
        BridgeHarnessFixture::new(vec![registration()])
            .with_policy(crate::facade::BridgeRuntimePolicy::development())
            .with_aspect_mapping(field_aspect_registration())
            .with_committed_patch(committed_patch("commit-a", "patch-a", "snapshot-a", "name"))
            .with_snapshot(field_slice_snapshot("snapshot-a", "alice")),
    )
    .declare_input("commit-a")
    .declare_observation("route")
    .compile();
    let request = ExecutionRequest::target("deliver-commit-a", "commit-a".to_string());

    let report = parity_suite(
        BridgeHarnessAdapter,
        fixture,
        request,
        ExecutionProfile::development("baseline"),
    )
    .candidates([
        ExecutionProfile::operational("operational"),
        ExecutionProfile::forensic("forensic"),
    ])
    .compare()
    .expect("fine-grained bridge parity suite should compare cleanly");

    assert!(report.matched);
    assert_eq!(report.results.len(), 2);
}
