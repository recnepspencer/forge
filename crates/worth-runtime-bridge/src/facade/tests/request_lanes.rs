use std::sync::Arc;

use super::runtime;
use crate::policy::BridgeRuntimePolicy;

#[test]
fn managed_request_lanes_share_installed_adapters_but_not_signal_or_basis_ownership() {
    let installed = runtime(BridgeRuntimePolicy::default());
    let first = installed.fork_managed_request_lane();
    let second = installed.fork_managed_request_lane();

    assert_ne!(installed.signal_runtime_key, first.signal_runtime_key);
    assert_ne!(first.signal_runtime_key, second.signal_runtime_key);
    assert!(Arc::ptr_eq(
        &installed.snapshot_read_source,
        &first.snapshot_read_source
    ));
    assert!(Arc::ptr_eq(
        &installed.committed_patch_source,
        &first.committed_patch_source
    ));
    assert!(!Arc::ptr_eq(
        &installed.execution_basis_reservations,
        &first.execution_basis_reservations
    ));
    assert!(!Arc::ptr_eq(
        &first.execution_basis_reservations,
        &second.execution_basis_reservations
    ));
}
