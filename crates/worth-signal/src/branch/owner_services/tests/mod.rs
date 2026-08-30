mod admission_affinity;
mod basis_denials;
mod branch_registry;
mod cancellation;
mod cell_posture_outcomes;
mod cell_progress;
mod exact_cell_contracts;
mod fork_contracts;
mod fork_sharing;
mod issuance_capability;
mod lifecycle;
mod managed_reference;
mod progress_bound;
mod registry_capacity;
mod root_destruction;
pub(super) mod runtime_root;

fn with_movement_permit(operation: impl FnOnce(&super::SignalOwnerMovementPermit<'_>)) {
    let source = super::SignalOwnerCancellationSource::new();
    let token = source.token();
    let permit = token
        .preflight_movement()
        .expect("the test cancellation source remains open");
    operation(&permit);
}

#[cfg(feature = "test-operation-control")]
#[test]
fn test_operation_control_build_preserves_kernel_authority_boundary() {
    use super::super::SignalOwnerOperationBoundary;

    let boundary = SignalOwnerOperationBoundary::TargetCellAdmission;
    assert_eq!(boundary, SignalOwnerOperationBoundary::TargetCellAdmission);
}
