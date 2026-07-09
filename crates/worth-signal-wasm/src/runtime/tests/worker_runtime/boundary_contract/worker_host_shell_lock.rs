use crate::runtime::tests::support::*;

#[test]
fn runtime_core_exposes_worker_runtime_shell_lock() {
    let runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();

    let lock = runtime.worker_runtime_shell_lock();

    assert_eq!(lock.identity.deployment_posture, "workerFirst");
    assert_eq!(lock.identity.runtime_authority, "workerOwnedRuntime");
    assert_eq!(lock.graph_publication_admission, "portableDefinitionsOnly");
    assert_eq!(lock.committed_envelope_family, "transactionResult");
    assert_eq!(lock.callback_publication_before_lowering, "denied");
}
