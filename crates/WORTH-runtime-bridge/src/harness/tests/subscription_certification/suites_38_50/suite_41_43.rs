use super::super::support::*;
use crate::policy::BridgeRuntimePolicy;

#[test]
fn bridge_harness_subscription_suite_41_to_43_rows_are_present() {
    let artifact = sealed_phase_18_closeout(BridgeRuntimePolicy::development());
    let rows = artifact.support_matrix().rows();
    assert!(rows
        .iter()
        .any(|row| row.suite_id().as_str() == "suite_41_ordering_hostility"));
    assert!(rows
        .iter()
        .any(|row| row.suite_id().as_str() == "suite_42_stale_checkpoint"));
    assert!(rows
        .iter()
        .any(|row| row.suite_id().as_str() == "suite_43_bundle_insufficiency"));
}
