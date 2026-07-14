use worth_store_io_scheduler::IoSchedulerIsolationAdmission;
use worth_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalReclaimRegion,
    PhysicalRecordSlot, PhysicalReference, PhysicalReferenceAuthority, PhysicalSegmentId,
    ReclaimedByteInterpretation,
};
use worth_store_reclaim_policy::{
    BackendCapabilityAdmissionRequest, BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet,
    BackendMediaAssumptionSet, BackendRebindTriggers, BackendTargetProfile,
    PhysicalBackendCapabilityAdmissionAuthority, PhysicalStoreReclaimPolicyExecutor, ReclaimPermit,
    ReclaimPolicyAdmission, ReclaimPolicyExecutionObservation, ReclaimPolicyExecutionRequest,
    ReclaimPolicyExecutionSession, ReclaimPolicyProofAuthority, ReclaimPolicyReachabilityProof,
    ReclaimPolicyRequest, ReclaimPolicySecurityScope,
};
use worth_store_security::admitted_store_internal_security_scope_for_io_qos_test;

use crate::{admit_tier_placement_io, ColdTierIoPosture};

#[test]
fn tiering_layout_reports_preserve_budget_and_owner_identity_basis() {
    let posture = real_cold_tier_posture();
    let admission = admit_tier_placement_io(
        IoSchedulerIsolationAdmission::for_certification_test(),
        posture.clone(),
    );

    let placement = admission.project_tier_placement_layout();
    assert_eq!(placement.declared_budget().reclaim_permits(), 1);
    assert_eq!(placement.reclaim_region().byte_len(), 4096);
    assert_eq!(placement.security_scope(), posture.security_scope());
    assert_eq!(placement.exact_counters(), admission.scheduler().counters());

    let recall = posture.project_cold_recall_layout();
    assert_eq!(recall.declared_budget().reclaim_permits(), 1);
    assert_eq!(recall.declared_budget().region_bytes(), 4096);
    assert_eq!(recall.security_scope(), posture.security_scope());
    assert_eq!(recall.exact_counters().executed(), 1);

    let amplification = posture.project_recall_amplification_layout();
    assert_eq!(amplification.declared_budget().reclaim_permits(), 1);
    assert_eq!(amplification.declared_budget().region_bytes(), 4096);
    assert_eq!(amplification.security_scope(), posture.security_scope());
    assert_eq!(amplification.exact_counters().executed(), 1);
}

fn real_cold_tier_posture() -> ColdTierIoPosture {
    let backend = PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(BackendCapabilityAdmissionRequest::new(
            BackendTargetProfile::PosixFileFsyncDirSync,
            BackendCapabilityEvidenceBasis::certified_backend_profile(),
            BackendCapabilitySupportSet::all_supported(),
            BackendMediaAssumptionSet::platform_file_defaults().with_cold_tier_io_posture(),
            BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
        ))
        .expect("backend should admit");
    let authority = ReclaimPolicyProofAuthority::for_admitted_backend(&backend);
    let region = test_region();
    let scope = admitted_store_internal_security_scope_for_io_qos_test();
    let policy = ReclaimPolicyAdmission::admit(
        authority,
        ReclaimPolicyRequest::new()
            .for_region(region)
            .with_posture(
                authority
                    .cold_tier_io_posture(
                        ReclaimedByteInterpretation::NonObservableReclaimedStorage,
                    )
                    .expect("cold-tier posture should admit"),
            )
            .with_reachability(
                ReclaimPolicyReachabilityProof::for_certification_test_authority(region),
            )
            .with_security_scope(ReclaimPolicySecurityScope::from_admitted_scope(&scope))
            .with_reclaim_permit(ReclaimPermit::new(1).expect("permit should admit"))
            .with_later_handoff_policy(authority.non_claim_later_handoff()),
    )
    .expect("policy should admit");

    let mut backend = TestReclaimBackend {
        observation: ReclaimPolicyExecutionObservation::new(
            region,
            ReclaimedByteInterpretation::NonObservableReclaimedStorage,
            ReclaimPolicySecurityScope::from_admitted_scope(&scope),
            true,
        ),
    };
    let receipt = ReclaimPolicyExecutionSession::for_owned_backend(&mut backend)
        .execute(policy)
        .expect("backend execution should succeed")
        .expect("policy execution should succeed");

    ColdTierIoPosture::from_reclaim_receipt(receipt)
        .expect("cold-tier posture should admit from the real receipt")
}

fn test_region() -> PhysicalReclaimRegion {
    let cell = PhysicalGenerationAuthority::for_canonical_physical_format()
        .slot_cell(
            PhysicalSegmentId::from_raw(1).unwrap(),
            PhysicalPageId::from_raw(1).unwrap(),
            PhysicalRecordSlot::from_raw(1).unwrap(),
        )
        .with_slot_generation(PhysicalGeneration::from_raw(1).unwrap());
    let reference: PhysicalReference = PhysicalReferenceAuthority::for_canonical_physical_format()
        .admit_page_slot(cell)
        .reference();
    PhysicalReclaimRegion::new(reference, 4096).unwrap()
}

struct TestReclaimBackend {
    observation: ReclaimPolicyExecutionObservation,
}

impl PhysicalStoreReclaimPolicyExecutor for TestReclaimBackend {
    type Error = core::convert::Infallible;

    fn execute_reclaim_policy(
        &mut self,
        _request: ReclaimPolicyExecutionRequest,
    ) -> Result<ReclaimPolicyExecutionObservation, Self::Error> {
        Ok(self.observation.clone())
    }
}
