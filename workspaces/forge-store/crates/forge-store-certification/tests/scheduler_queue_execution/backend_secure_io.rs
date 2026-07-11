use forge_store_physical_backend::{
    preserve_secure_io_for_backend_completion, BackendQueueExecutionAdaptation,
    BackendQueueExecutionCompletion, BackendQueueExecutionPosture, BackendQueueSpeculativeScope,
    BackendSecureIoPreservationDenial,
};

use super::support::{backend_witness, scheduler_security_scope, secure_backend_binding};

#[test]
fn backend_secure_io_preservation_rejects_wrong_read_ahead_scope() {
    let scope = scheduler_security_scope().permission().identity();
    let binding = secure_backend_binding(scope);
    let witness = backend_witness();
    let posture = BackendQueueExecutionPosture::from_admitted_capability(
        &witness,
        BackendQueueExecutionAdaptation::None,
    )
    .expect("backend posture should admit");
    let wrong_scope = BackendQueueSpeculativeScope::admitted(
        scope,
        forge_store_security::StoreTenantScope::RepairBlastRadius,
        scope.key_scope(),
    );
    let completion = BackendQueueExecutionCompletion::for_certification(binding, posture)
        .observe_read_ahead(1, wrong_scope);

    let denial = preserve_secure_io_for_backend_completion(completion)
        .expect_err("backend must reject cross-scope read-ahead observation");

    assert_eq!(
        denial,
        BackendSecureIoPreservationDenial::ReadAheadScopeMismatch
    );
}
