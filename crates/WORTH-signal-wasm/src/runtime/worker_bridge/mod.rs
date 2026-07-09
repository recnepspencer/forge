use crate::runtime::core::RuntimeCore;

mod worker_boundary_artifact_lock;
mod worker_boundary_causality;
mod worker_boundary_envelope_family;
mod worker_boundary_proof_topology;
mod worker_boundary_readmission_proof;
mod worker_deployment_posture;
mod worker_fallback_policy;

use worker_boundary_artifact_lock::WorkerBoundaryArtifactLock;

impl RuntimeCore {
    pub fn worker_boundary_artifact_lock(&self) -> WorkerBoundaryArtifactLock {
        WorkerBoundaryArtifactLock::frozen_worker_boundary_contract()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frozen_worker_boundary_contract_exactly_names_envelope_families() {
        let lock = WorkerBoundaryArtifactLock::frozen_worker_boundary_contract();

        assert_eq!(
            envelope_family_labels(&lock),
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
            envelope_family_directions(&lock),
            [
                "mainThreadToWorker",
                "workerToMainThread",
                "mainThreadToWorker",
                "mainThreadToWorker",
                "workerToMainThread",
                "workerToMainThread",
                "workerToMainThread",
                "bidirectional",
                "bidirectional",
            ]
        );
        assert!(lock
            .envelope_families
            .iter()
            .all(|family| family.carries_causality && family.requires_worker_readmission));
    }

    #[test]
    fn causality_stamp_orders_transaction_then_generation() {
        let earlier = worker_boundary_causality::WorkerBoundaryCausalityStamp::new(7, 2);
        let later_same_transaction =
            worker_boundary_causality::WorkerBoundaryCausalityStamp::new(7, 3);
        let later_transaction = worker_boundary_causality::WorkerBoundaryCausalityStamp::new(8, 0);

        assert!(earlier < later_same_transaction);
        assert!(later_same_transaction < later_transaction);
        assert_eq!(
            later_transaction.ordering_basis(),
            "transactionSequenceThenGeneration"
        );
    }

    #[test]
    fn deployment_postures_with_fallback_taxonomies_are_explicit() {
        let lock = WorkerBoundaryArtifactLock::frozen_worker_boundary_contract();

        assert_eq!(
            deployment_posture_labels(&lock),
            ["workerFirst", "mainThreadCompatibility"]
        );
        assert_eq!(
            deployment_runtime_authorities(&lock),
            ["workerOwnedRuntime", "mainThreadRuntime"]
        );
        assert_eq!(
            fallback_policy_labels(&lock),
            ["denyByDefault", "productDeclaredFallbackOnly"]
        );
        assert!(lock
            .fallback_policies
            .iter()
            .all(|policy| !policy.hidden_fallback_allowed && policy.denial_artifact_required));
    }

    #[test]
    fn proof_topology_exactly_names_worth_proof_progression() {
        let lock = WorkerBoundaryArtifactLock::frozen_worker_boundary_contract();

        assert_eq!(
            proof_stage_labels(&lock),
            [
                "rawPlacementDeclaration",
                "placementClassifiedDeclaration",
                "loweredWorkerExecutionPlan",
                "loweredMainThreadHostedExecutionPlan",
                "boundaryBridgedReadmission",
            ]
        );
        assert_eq!(
            proof_stage_worth_proof_labels(&lock),
            [
                "Recipe<Unresolved, RawPlacementDeclaration>",
                "TransitionOutcome<PlacementClassifiedDeclaration, PlacementDenialArtifact>",
                "Recipe<Lowered, LoweredWorkerExecutionPlan>",
                "Recipe<Lowered, LoweredMainThreadHostedExecutionPlan>",
                "Recipe<Admitted, BoundaryBridgedWorkerEnvelope>",
            ]
        );
        assert!(lock
            .proof_stages
            .iter()
            .all(|stage| stage.rust_type.contains("worth_signal_wasm")));
    }

    fn envelope_family_labels(lock: &WorkerBoundaryArtifactLock) -> Vec<&'static str> {
        lock.envelope_families
            .iter()
            .map(|family| family.label)
            .collect()
    }

    fn envelope_family_directions(lock: &WorkerBoundaryArtifactLock) -> Vec<&'static str> {
        lock.envelope_families
            .iter()
            .map(|family| family.direction)
            .collect()
    }

    fn deployment_posture_labels(lock: &WorkerBoundaryArtifactLock) -> Vec<&'static str> {
        lock.deployment_postures
            .iter()
            .map(|posture| posture.label)
            .collect()
    }

    fn deployment_runtime_authorities(lock: &WorkerBoundaryArtifactLock) -> Vec<&'static str> {
        lock.deployment_postures
            .iter()
            .map(|posture| posture.runtime_authority)
            .collect()
    }

    fn fallback_policy_labels(lock: &WorkerBoundaryArtifactLock) -> Vec<&'static str> {
        lock.fallback_policies
            .iter()
            .map(|policy| policy.label)
            .collect()
    }

    fn proof_stage_labels(lock: &WorkerBoundaryArtifactLock) -> Vec<&'static str> {
        lock.proof_stages.iter().map(|stage| stage.label).collect()
    }

    fn proof_stage_worth_proof_labels(lock: &WorkerBoundaryArtifactLock) -> Vec<&'static str> {
        lock.proof_stages
            .iter()
            .map(|stage| stage.worth_proof_stage)
            .collect()
    }
}
