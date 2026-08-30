mod admission_affinity;
mod branch_registry;
mod cell_progress;
mod lifecycle;
mod progress_bound;

#[cfg(feature = "test-operation-control")]
#[test]
fn test_operation_control_build_preserves_kernel_authority_boundary() {
    use super::super::SignalOwnerOperationBoundary;

    let boundary = SignalOwnerOperationBoundary::TargetCellAdmission;
    assert_eq!(boundary, SignalOwnerOperationBoundary::TargetCellAdmission);
}
