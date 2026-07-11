use forge_store_physical_format::PhysicalReclaimRegion;
use forge_store_physical_format::ReclaimedByteInterpretation;
use forge_store_reclaim_policy::{
    ReclaimPermit, ReclaimPolicyCounterSnapshot, ReclaimPolicyExecutionReceipt,
    ReclaimPolicyOperation,
};
use forge_store_security::StoreSecurityScopeIdentity;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S6ColdTierIoPosture {
    receipt: ReclaimPolicyExecutionReceipt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S6ColdTierIoPostureDenial {
    NotColdTierIoPosture,
}

impl S6ColdTierIoPosture {
    #[cfg(feature = "certification-test-authority")]
    pub fn for_certification_test_authority(
        security_scope: StoreSecurityScopeIdentity,
        _counters: ReclaimPolicyCounterSnapshot,
    ) -> Self {
        Self::from_reclaim_receipt(certification_test_receipt(security_scope))
            .expect("certification test posture must admit from a real receipt")
    }

    pub fn from_reclaim_receipt(
        receipt: ReclaimPolicyExecutionReceipt,
    ) -> Result<Self, S6ColdTierIoPostureDenial> {
        let policy = receipt.policy();
        if policy.posture().operation() != ReclaimPolicyOperation::ColdTierMovementPosture {
            return Err(S6ColdTierIoPostureDenial::NotColdTierIoPosture);
        }
        Ok(Self { receipt })
    }

    pub const fn reclaim_receipt(&self) -> &ReclaimPolicyExecutionReceipt {
        &self.receipt
    }

    pub fn interpretation(&self) -> ReclaimedByteInterpretation {
        self.receipt.observed_interpretation()
    }

    pub fn security_scope(&self) -> StoreSecurityScopeIdentity {
        self.receipt.policy().security_scope().identity()
    }

    pub fn reclaim_region(&self) -> PhysicalReclaimRegion {
        self.receipt.policy().region()
    }

    pub fn reclaim_permit(&self) -> ReclaimPermit {
        self.receipt.policy().permit()
    }

    pub fn counters(&self) -> ReclaimPolicyCounterSnapshot {
        self.receipt.counters()
    }

    pub const fn carries_tier_placement_claim(&self) -> bool {
        false
    }

    pub const fn carries_compaction_claim(&self) -> bool {
        false
    }
}

#[cfg(feature = "certification-test-authority")]
fn certification_test_receipt(
    security_scope: StoreSecurityScopeIdentity,
) -> ReclaimPolicyExecutionReceipt {
    use forge_store_physical_backend::{
        BackendCapabilityAdmissionRequest, BackendCapabilityEvidenceBasis,
        BackendCapabilitySupportSet, BackendMediaAssumptionSet, BackendRebindTriggers,
        BackendTargetProfile, PhysicalBackendCapabilityAdmissionAuthority,
    };
    use forge_store_reclaim_policy::{
        ReclaimPolicyAdmission, ReclaimPolicyExecutionObservation, ReclaimPolicyExecutionSession,
        ReclaimPolicyProofAuthority, ReclaimPolicyReachabilityProof, ReclaimPolicyRequest,
        ReclaimPolicySecurityScope,
    };

    let backend = PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(BackendCapabilityAdmissionRequest::new(
            BackendTargetProfile::PosixFileFsyncDirSync,
            BackendCapabilityEvidenceBasis::certified_backend_profile(),
            BackendCapabilitySupportSet::all_supported(),
            BackendMediaAssumptionSet::platform_file_defaults().with_cold_tier_io_posture(),
            BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
        ))
        .expect("certification test backend should admit");
    let authority = ReclaimPolicyProofAuthority::for_admitted_backend(&backend);
    let admitted_scope = forge_store_security::admitted_store_internal_security_scope_for_s6_test();
    assert_eq!(admitted_scope.identity(), security_scope);
    let policy = ReclaimPolicyAdmission::admit(
        authority,
        ReclaimPolicyRequest::new()
            .for_region(certification_test_region())
            .with_posture(
                authority
                    .cold_tier_io_posture(
                        ReclaimedByteInterpretation::NonObservableReclaimedStorage,
                    )
                    .expect("cold-tier posture should admit"),
            )
            .with_reachability(
                ReclaimPolicyReachabilityProof::for_certification_test_authority(
                    certification_test_region(),
                ),
            )
            .with_security_scope(ReclaimPolicySecurityScope::from_admitted_scope(
                &admitted_scope,
            ))
            .with_reclaim_permit(ReclaimPermit::new(1).expect("permit should admit"))
            .with_later_handoff_policy(authority.non_claim_later_handoff()),
    )
    .expect("certification test policy should admit");
    let mut backend = CertificationReceiptBackend {
        observation: ReclaimPolicyExecutionObservation::new(
            certification_test_region(),
            ReclaimedByteInterpretation::NonObservableReclaimedStorage,
            ReclaimPolicySecurityScope::from_admitted_scope(&admitted_scope),
            true,
        ),
    };
    ReclaimPolicyExecutionSession::for_owned_backend(&mut backend)
        .execute(policy)
        .expect("certification test backend should execute")
        .expect("certification test execution should succeed")
}

#[cfg(feature = "certification-test-authority")]
fn certification_test_region() -> PhysicalReclaimRegion {
    let cell = forge_store_physical_format::PhysicalGenerationAuthority::s1()
        .slot_cell(
            forge_store_physical_format::PhysicalSegmentId::from_raw(1)
                .expect("test segment should admit"),
            forge_store_physical_format::PhysicalPageId::from_raw(1)
                .expect("test page should admit"),
            forge_store_physical_format::PhysicalRecordSlot::from_raw(1)
                .expect("test slot should admit"),
        )
        .with_slot_generation(
            forge_store_physical_format::PhysicalGeneration::from_raw(1)
                .expect("generation should admit"),
        );
    let reference: forge_store_physical_format::PhysicalReference =
        forge_store_physical_format::PhysicalReferenceAuthority::s1()
            .admit_page_slot(cell)
            .reference();
    PhysicalReclaimRegion::new(reference, 4096).expect("test reclaim region should admit")
}

#[cfg(feature = "certification-test-authority")]
struct CertificationReceiptBackend {
    observation: forge_store_reclaim_policy::ReclaimPolicyExecutionObservation,
}

#[cfg(feature = "certification-test-authority")]
impl forge_store_reclaim_policy::PhysicalStoreReclaimPolicyExecutor
    for CertificationReceiptBackend
{
    type Error = core::convert::Infallible;

    fn execute_reclaim_policy(
        &mut self,
        _request: forge_store_reclaim_policy::ReclaimPolicyExecutionRequest,
    ) -> Result<forge_store_reclaim_policy::ReclaimPolicyExecutionObservation, Self::Error> {
        Ok(self.observation.clone())
    }
}
