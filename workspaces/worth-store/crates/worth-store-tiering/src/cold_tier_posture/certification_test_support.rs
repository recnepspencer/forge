use worth_store_physical_backend::{
    BackendCapabilityAdmissionRequest, BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet,
    BackendMediaAssumptionSet, BackendRebindTriggers, BackendTargetProfile,
    PhysicalBackendCapabilityAdmissionAuthority,
};
use worth_store_physical_format::{PhysicalReclaimRegion, ReclaimedByteInterpretation};
use worth_store_reclaim_policy::{
    ReclaimPermit, ReclaimPolicyAdmission, ReclaimPolicyExecutionObservation,
    ReclaimPolicyExecutionSession, ReclaimPolicyProofAuthority, ReclaimPolicyReachabilityProof,
    ReclaimPolicyRequest, ReclaimPolicySecurityScope,
};
use worth_store_security::StoreSecurityScopeIdentity;

use super::ColdTierIoPosture;

pub fn cold_tier_io_posture_for_certification_test(
    security_scope: StoreSecurityScopeIdentity,
) -> ColdTierIoPosture {
    let backend = PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(BackendCapabilityAdmissionRequest::new(
            BackendTargetProfile::PosixFileFsyncDirSync,
            BackendCapabilityEvidenceBasis::certified_backend_profile(),
            BackendCapabilitySupportSet::all_supported(),
            BackendMediaAssumptionSet::platform_file_defaults().with_cold_tier_io_posture(),
            BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
        ))
        .expect("certification backend should admit");
    let authority = ReclaimPolicyProofAuthority::for_admitted_backend(&backend);
    let admitted_scope =
        worth_store_security::admitted_security_scope_for_identity_for_test(security_scope);
    let region = certification_region();
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
            .with_security_scope(ReclaimPolicySecurityScope::from_admitted_scope(
                &admitted_scope,
            ))
            .with_reclaim_permit(ReclaimPermit::new(1).expect("permit should admit"))
            .with_later_handoff_policy(authority.non_claim_later_handoff()),
    )
    .expect("certification policy should admit");
    let mut backend = CertificationReceiptBackend {
        observation: ReclaimPolicyExecutionObservation::new(
            region,
            ReclaimedByteInterpretation::NonObservableReclaimedStorage,
            ReclaimPolicySecurityScope::from_admitted_scope(&admitted_scope),
            true,
        ),
    };
    let receipt = ReclaimPolicyExecutionSession::for_owned_backend(&mut backend)
        .execute(policy)
        .expect("certification backend should execute")
        .expect("certification execution should succeed");
    ColdTierIoPosture::from_reclaim_receipt(receipt).expect("executed posture should admit")
}

fn certification_region() -> PhysicalReclaimRegion {
    let cell =
        worth_store_physical_format::PhysicalGenerationAuthority::for_canonical_physical_format()
            .slot_cell(
                worth_store_physical_format::PhysicalSegmentId::from_raw(1).expect("test segment"),
                worth_store_physical_format::PhysicalPageId::from_raw(1).expect("test page"),
                worth_store_physical_format::PhysicalRecordSlot::from_raw(1).expect("test slot"),
            )
            .with_slot_generation(
                worth_store_physical_format::PhysicalGeneration::from_raw(1).expect("generation"),
            );
    let reference =
        worth_store_physical_format::PhysicalReferenceAuthority::for_canonical_physical_format()
            .admit_page_slot(cell)
            .reference();
    PhysicalReclaimRegion::new(reference, 4096).expect("test reclaim region")
}

struct CertificationReceiptBackend {
    observation: ReclaimPolicyExecutionObservation,
}

impl worth_store_reclaim_policy::PhysicalStoreReclaimPolicyExecutor
    for CertificationReceiptBackend
{
    type Error = core::convert::Infallible;

    fn execute_reclaim_policy(
        &mut self,
        _request: worth_store_reclaim_policy::ReclaimPolicyExecutionRequest,
    ) -> Result<ReclaimPolicyExecutionObservation, Self::Error> {
        Ok(self.observation.clone())
    }
}
