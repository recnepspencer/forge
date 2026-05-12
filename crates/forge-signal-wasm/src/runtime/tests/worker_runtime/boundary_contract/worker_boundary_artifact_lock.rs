use crate::runtime::tests::support::*;

fn runtime() -> RuntimeCore {
    RuntimeCore::new(RuntimePolicySpec::default()).unwrap()
}

#[test]
fn worker_boundary_artifact_lock_exposes_frozen_bridge_vocabulary() {
    let runtime = runtime();

    let lock = runtime.worker_boundary_artifact_lock();

    assert_eq!(lock.artifact_lock_version, 1);
    assert_eq!(
        lock.envelope_families
            .iter()
            .map(|family| family.label)
            .collect::<Vec<_>>(),
        [
            "transactionSubmission",
            "transactionResult",
            "hostCapabilityIngress",
            "browserHistoryIngress",
            "hostEffectEgress",
            "outputDelivery",
            "observationDelivery",
            "diagnosticsHistoryRead",
            "lifecycleControl",
        ]
    );
    assert_eq!(
        lock.causality_model.ordering_basis,
        "transactionSequenceThenGeneration"
    );
    assert_eq!(
        lock.deployment_postures
            .iter()
            .map(|posture| posture.label)
            .collect::<Vec<_>>(),
        ["workerFirst", "mainThreadCompatibility"]
    );
    assert_eq!(
        lock.proof_stages
            .iter()
            .map(|stage| stage.label)
            .collect::<Vec<_>>(),
        [
            "rawPlacementDeclaration",
            "placementClassifiedDeclaration",
            "loweredWorkerExecutionPlan",
            "loweredMainThreadHostedExecutionPlan",
            "boundaryBridgedReadmission",
        ]
    );
    assert!(lock
        .fallback_policies
        .iter()
        .all(|policy| !policy.hidden_fallback_allowed));
}
