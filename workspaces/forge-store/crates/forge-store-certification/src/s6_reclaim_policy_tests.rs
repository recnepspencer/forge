use super::reclaim_policy::{S6ReclaimPolicyEvidenceOutcomeKind, S6ReclaimPolicyEvidenceRow};
use forge_store_blob_chunks::S6BlobReclaimNonClaimHandoff;
use forge_store_physical_backend::{
    BackendCapabilityAdmissionRequest, BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet,
    BackendMediaAssumptionSet, BackendRebindTriggers, BackendTargetProfile,
    PhysicalBackendCapabilityAdmissionAuthority,
};
use forge_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalReclaimRegion,
    PhysicalRecordSlot, PhysicalReference, PhysicalReferenceAuthority, PhysicalSegmentId,
    ReclaimedByteInterpretation,
};
use forge_store_reclaim_policy::{
    PhysicalStoreReclaimPolicyExecutor, ReclaimPermit, ReclaimPolicyAdmission,
    ReclaimPolicyExecutionObservation, ReclaimPolicyExecutionRequest,
    ReclaimPolicyExecutionSession, ReclaimPolicyProofAuthority, ReclaimPolicyReachabilityProof,
    ReclaimPolicyRequest, ReclaimPolicySecurityScope, StoreOwnedReclaimPolicyExecution,
};
use forge_store_security::admitted_store_internal_security_scope_for_s6_test;
use forge_store_tiering::S6ColdTierIoPosture;

#[test]
fn reclaim_policy_evidence_materializes_execution_and_non_claim_handoffs() {
    let backend = admitted_backend();
    let authority = ReclaimPolicyProofAuthority::for_admitted_backend(&backend);
    let admitted = ReclaimPolicyAdmission::admit(authority, request(&backend)).unwrap();
    let mut executor = ObservingBackend;
    let receipt = ReclaimPolicyExecutionSession::for_store_backend(
        &mut executor,
        StoreOwnedReclaimPolicyExecution::for_certification_test_authority(),
    )
    .execute(admitted)
    .unwrap()
    .unwrap();

    let execution_row = S6ReclaimPolicyEvidenceRow::from_execution_receipt(receipt.clone());
    assert_eq!(
        execution_row.outcome(),
        &S6ReclaimPolicyEvidenceOutcomeKind::Executed
    );
    assert_eq!(
        execution_row.interpretation(),
        Some(ReclaimedByteInterpretation::NonObservableReclaimedStorage)
    );

    let blob = S6BlobReclaimNonClaimHandoff::from_reclaim_receipt(receipt.clone());
    assert!(!blob.carries_blob_lifecycle_claim());
    let blob_row = S6ReclaimPolicyEvidenceRow::from_blob_non_claim_handoff(blob);
    assert_eq!(
        blob_row.outcome(),
        &S6ReclaimPolicyEvidenceOutcomeKind::BlobNonClaimHandoff
    );

    let cold = S6ColdTierIoPosture::from_reclaim_receipt(receipt).unwrap();
    assert!(!cold.carries_tier_placement_claim());
    assert!(!cold.carries_compaction_claim());
    let cold_row = S6ReclaimPolicyEvidenceRow::from_cold_tier_non_claim_handoff(cold);
    assert_eq!(
        cold_row.outcome(),
        &S6ReclaimPolicyEvidenceOutcomeKind::ColdTierNonClaimHandoff
    );
}

struct ObservingBackend;

impl PhysicalStoreReclaimPolicyExecutor for ObservingBackend {
    type Error = ();

    fn execute_reclaim_policy(
        &mut self,
        request: ReclaimPolicyExecutionRequest,
    ) -> Result<ReclaimPolicyExecutionObservation, Self::Error> {
        Ok(ReclaimPolicyExecutionObservation::new(
            request.policy().region(),
            ReclaimedByteInterpretation::NonObservableReclaimedStorage,
            request.policy().security_scope(),
            true,
        ))
    }
}

fn request(
    backend: &forge_store_physical_backend::AdmittedBackendCapabilityWitness,
) -> ReclaimPolicyRequest {
    let authority = ReclaimPolicyProofAuthority::for_admitted_backend(backend);
    ReclaimPolicyRequest::new()
        .for_region(PhysicalReclaimRegion::new(reference(), 4096).unwrap())
        .with_posture(
            authority
                .cold_tier_io_posture(ReclaimedByteInterpretation::NonObservableReclaimedStorage)
                .unwrap(),
        )
        .with_reachability(
            ReclaimPolicyReachabilityProof::for_certification_test_authority(
                PhysicalReclaimRegion::new(reference(), 4096).unwrap(),
            ),
        )
        .with_security_scope(ReclaimPolicySecurityScope::from_admitted_scope(
            &admitted_store_internal_security_scope_for_s6_test(),
        ))
        .with_reclaim_permit(ReclaimPermit::new(1).unwrap())
        .with_later_handoff_policy(authority.non_claim_later_handoff())
}

fn admitted_backend() -> forge_store_physical_backend::AdmittedBackendCapabilityWitness {
    PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(BackendCapabilityAdmissionRequest::new(
            BackendTargetProfile::PosixFileFsyncDirSync,
            BackendCapabilityEvidenceBasis::certified_backend_profile(),
            BackendCapabilitySupportSet::all_supported(),
            BackendMediaAssumptionSet::platform_file_defaults().with_cold_tier_io_posture(),
            BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
        ))
        .unwrap()
}

fn reference() -> PhysicalReference {
    let cell = PhysicalGenerationAuthority::s1()
        .slot_cell(
            PhysicalSegmentId::from_raw(2).unwrap(),
            PhysicalPageId::from_raw(7).unwrap(),
            PhysicalRecordSlot::from_raw(1).unwrap(),
        )
        .with_slot_generation(PhysicalGeneration::from_raw(1).unwrap());
    PhysicalReferenceAuthority::s1()
        .admit_page_slot(cell)
        .reference()
}
