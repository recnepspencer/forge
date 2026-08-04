pub(crate) fn assert_recovery_lifecycle_dag(explanation: &crate::CanonicalOwnerPlanDagExplanation) {
    use crate::{OwnerPlanAccess, OwnerPlanEffect, OwnerPlanExecutionStage, StoreOwnerKind};

    assert_eq!(explanation.node_count(), 5);
    assert_eq!(explanation.edge_count(), 4);
    for (owner, effect, stage, access) in [
        (
            StoreOwnerKind::PhysicalBackend,
            OwnerPlanEffect::CopyBackupComponent,
            OwnerPlanExecutionStage::Staging,
            OwnerPlanAccess::Mutate,
        ),
        (
            StoreOwnerKind::RecoveryPhysics,
            OwnerPlanEffect::ReplayWalToExactFrontier,
            OwnerPlanExecutionStage::Staging,
            OwnerPlanAccess::Mutate,
        ),
        (
            StoreOwnerKind::PhysicalIntegrity,
            OwnerPlanEffect::ValidatePhysicalIntegrity,
            OwnerPlanExecutionStage::PostVerification,
            OwnerPlanAccess::Observe,
        ),
        (
            StoreOwnerKind::LayoutIndexes,
            OwnerPlanEffect::VerifyLayoutArtifacts,
            OwnerPlanExecutionStage::PostVerification,
            OwnerPlanAccess::Observe,
        ),
        (
            StoreOwnerKind::BlobChunks,
            OwnerPlanEffect::VerifyBlobArtifacts,
            OwnerPlanExecutionStage::PostVerification,
            OwnerPlanAccess::Observe,
        ),
    ] {
        assert!(explanation.nodes().iter().any(|node| {
            node.owner() == owner
                && node.effect() == effect
                && node.stage() == stage
                && node.access() == access
                && node.expected_receipt_fingerprint() != [0; 32]
        }));
    }
    let first = explanation.first_irreversible_node().unwrap();
    assert_eq!(
        explanation
            .nodes()
            .iter()
            .find(|node| node.identity() == first)
            .unwrap()
            .owner(),
        StoreOwnerKind::PhysicalBackend
    );
}
