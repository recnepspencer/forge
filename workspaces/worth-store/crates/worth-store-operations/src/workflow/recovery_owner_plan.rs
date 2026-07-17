use sha2::{Digest, Sha256};
use worth_store_offline_verifier::StagedRecoveryOwnerVerificationSet;

use crate::owner_plan_dag::{
    CanonicalOwnerPlanDag, OwnerPlanEffect, OwnerPlanExecutionStage, OwnerPlanFootprint,
    OwnerPlanNode, OwnerPlanPrerequisite, StoreOwnerKind,
};

pub(super) struct RecoveryLifecycleOwnerPlan {
    pub(super) dag: CanonicalOwnerPlanDag,
    pub(super) explanation: crate::CanonicalOwnerPlanDagExplanation,
    pub(super) verification: StagedRecoveryOwnerVerificationSet,
}

pub(super) fn lower_recovery_lifecycle_owners(
    backend_fingerprint: [u8; 32],
    recovery_fingerprint: [u8; 32],
    footprint: OwnerPlanFootprint,
    verification: StagedRecoveryOwnerVerificationSet,
) -> Result<RecoveryLifecycleOwnerPlan, crate::OwnerPlanDagDenial> {
    let backend = staging_node(
        StoreOwnerKind::PhysicalBackend,
        OwnerPlanEffect::CopyBackupComponent,
        footprint,
        backend_fingerprint,
        true,
    );
    let recovery = staging_node(
        StoreOwnerKind::RecoveryPhysics,
        OwnerPlanEffect::ReplayWalToExactFrontier,
        footprint,
        recovery_fingerprint,
        true,
    );
    let integrity = verification_node(
        StoreOwnerKind::PhysicalIntegrity,
        OwnerPlanEffect::ValidatePhysicalIntegrity,
        footprint,
        verification.physical_integrity(),
    );
    let layout = verification_node(
        StoreOwnerKind::LayoutIndexes,
        OwnerPlanEffect::VerifyLayoutArtifacts,
        footprint,
        verification.layout_indexes(),
    );
    let blob = verification_node(
        StoreOwnerKind::BlobChunks,
        OwnerPlanEffect::VerifyBlobArtifacts,
        footprint,
        verification.blob_chunks(),
    );
    let edges = vec![
        OwnerPlanPrerequisite::new(backend.identity(), recovery.identity(), true),
        OwnerPlanPrerequisite::new(recovery.identity(), integrity.identity(), true),
        OwnerPlanPrerequisite::new(recovery.identity(), layout.identity(), true),
        OwnerPlanPrerequisite::new(recovery.identity(), blob.identity(), true),
    ];
    let dag =
        CanonicalOwnerPlanDag::admit(vec![backend, recovery, integrity, layout, blob], edges)?;
    Ok(RecoveryLifecycleOwnerPlan {
        explanation: dag.explanation().clone(),
        dag,
        verification,
    })
}

fn staging_node(
    owner: StoreOwnerKind,
    effect: OwnerPlanEffect,
    footprint: OwnerPlanFootprint,
    fingerprint: [u8; 32],
    irreversible: bool,
) -> OwnerPlanNode {
    OwnerPlanNode::from_owner_binding_at_stage(
        owner,
        effect,
        OwnerPlanExecutionStage::Staging,
        footprint,
        footprint.end_exclusive().saturating_sub(footprint.start()),
        irreversible,
        fingerprint,
        Sha256::digest(fingerprint).into(),
    )
}

fn verification_node(
    owner: StoreOwnerKind,
    effect: OwnerPlanEffect,
    footprint: OwnerPlanFootprint,
    receipt_fingerprint: [u8; 32],
) -> OwnerPlanNode {
    OwnerPlanNode::from_owner_observation_binding(
        owner,
        effect,
        OwnerPlanExecutionStage::PostVerification,
        footprint,
        footprint.end_exclusive().saturating_sub(footprint.start()),
        receipt_fingerprint,
        receipt_fingerprint,
    )
}
