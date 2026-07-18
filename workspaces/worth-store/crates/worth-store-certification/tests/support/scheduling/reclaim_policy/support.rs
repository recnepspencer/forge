use worth_store_physical_backend::{
    BackendCapabilityAdmissionRequest, BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet,
    BackendMediaAssumptionSet, BackendRebindTriggers, BackendTargetProfile,
    PhysicalBackendCapabilityAdmissionAuthority,
};
use worth_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalReclaimRegion,
    PhysicalRecordSlot, PhysicalReference, PhysicalReferenceAuthority, PhysicalSegmentId,
    ReclaimedByteInterpretation,
};
use worth_store_physical_isolation::{
    admit_seed_stable_read_plan, BackupReachabilityLeaseIndexSnapshot,
    CompactProtectedReferenceSet, CurrentPhysicalRoot, ExecutedReachabilityEvidence,
    HazardLeaseEpochIndexSnapshot, HazardLeaseTable, HazardLeaseTableCapacity,
    PhysicalReadPlanReleaseSemantics, PostProtectionPhysicalReadObservation,
    ProtectedPhysicalReferenceSet, ProtectedReferenceLease, PublishedReaderHazard,
    ReadPlanAdmissionScratchArena, ReclaimCandidateSet, ReclaimEligibilityProof,
    ReleasedOldReachability, S6ReclaimReachabilityRemovalEvidence, TraversalAdmissionGuard,
    UnprotectedReadIntent,
};
use worth_store_reclaim_policy::{
    AdmittedReclaimPolicy, PhysicalStoreReclaimPolicyExecutor, ReclaimPermit,
    ReclaimPolicyAdmission, ReclaimPolicyExecutionObservation, ReclaimPolicyExecutionReceipt,
    ReclaimPolicyExecutionRequest, ReclaimPolicyExecutionSession, ReclaimPolicyProofAuthority,
    ReclaimPolicyReachabilityDenial, ReclaimPolicyReachabilityProof, ReclaimPolicyRequest,
    ReclaimPolicySecurityScope, ReclaimPolicyViolation, StoreOwnedReclaimPolicyExecution,
};

use super::support::{
    current_generation_page_reference, current_root_from_authority,
    physical_authority_from_complete_closeout,
};

#[derive(Clone)]
struct ObservingBackend {
    observation: ReclaimPolicyExecutionObservation,
}

pub struct S6ReclaimFixture {
    candidates: ReclaimCandidateSet,
    released: ReleasedOldReachability,
    root: CurrentPhysicalRoot,
    live_lease: ProtectedReferenceLease,
}

impl PhysicalStoreReclaimPolicyExecutor for ObservingBackend {
    type Error = ();

    fn execute_reclaim_policy(
        &mut self,
        request: ReclaimPolicyExecutionRequest,
    ) -> Result<ReclaimPolicyExecutionObservation, Self::Error> {
        let _ = request;
        Ok(self.observation.clone())
    }
}

impl S6ReclaimFixture {
    pub fn new(generation: u64) -> Self {
        let authority = physical_authority_from_complete_closeout();
        let root = current_root_from_authority(&authority);
        let reference = current_generation_page_reference(generation);
        let references = ProtectedPhysicalReferenceSet::from_current_generation_refs_with_scratch(
            [reference],
            ReadPlanAdmissionScratchArena::for_protected_reference_capacity(1),
        )
        .unwrap();
        let observed_references = references.clone();
        let compact = CompactProtectedReferenceSet::from_reference_set_with_scratch(
            references.clone(),
            ReadPlanAdmissionScratchArena::for_protected_reference_capacity(1),
        )
        .unwrap();
        let intent = UnprotectedReadIntent::for_known_footprint(root, references, 4096)
            .with_release_semantics(PhysicalReadPlanReleaseSemantics::reader_releases_all());
        let hazard = PublishedReaderHazard::publish(&authority, intent).unwrap();
        let live_lease =
            ProtectedReferenceLease::from_reader_hazard(&hazard, compact.clone()).unwrap();
        let observed =
            PostProtectionPhysicalReadObservation::from_authority_after_hazard_publication(
                &authority,
                &hazard,
                root,
                observed_references,
            )
            .unwrap();
        let validated = hazard
            .observe_authority_after_publication(&authority, observed)
            .unwrap()
            .validate()
            .unwrap();
        let receipt = TraversalAdmissionGuard::from_validated_root(validated)
            .admit(ReadPlanAdmissionScratchArena::for_protected_reference_capacity(1))
            .unwrap();
        let read_release = admit_seed_stable_read_plan(receipt.into_cursor().finish())
            .unwrap()
            .into_execution_ready_handle()
            .release();
        let released =
            worth_store_physical_isolation::OldReachabilityPreservation::from_protected_footprint(
                compact.declared_footprint_basis(),
            )
            .unwrap()
            .admit_release(read_release)
            .unwrap();
        let candidates =
            ReclaimCandidateSet::from_released_old_reachability(released, &compact).unwrap();
        Self {
            candidates,
            released,
            root,
            live_lease,
        }
    }

    pub fn executed_reachability(&self) -> ExecutedReachabilityEvidence {
        ExecutedReachabilityEvidence::from_released_old_reachability(
            self.released,
            self.candidates.clone(),
        )
        .unwrap()
    }

    pub fn live_hazard_snapshot(&self) -> HazardLeaseEpochIndexSnapshot {
        let mut table =
            HazardLeaseTable::with_capacity(HazardLeaseTableCapacity::bounded_slots(1).unwrap());
        table.acquire(self.root, self.live_lease.clone()).unwrap();
        table.live_index_snapshot()
    }
}

pub fn admitted_policy_for_region(
    region: PhysicalReclaimRegion,
    interpretation: ReclaimedByteInterpretation,
) -> AdmittedReclaimPolicy {
    let backend = admitted_backend();
    let authority = ReclaimPolicyProofAuthority::for_admitted_backend(&backend);
    ReclaimPolicyAdmission::admit(
        authority,
        base_real_chain_request(
            region,
            real_reachability_for_region(region.reference().generation().get(), region),
        )
        .with_posture(authority.cold_tier_io_posture(interpretation).unwrap())
        .with_security_scope(internal_security_scope())
        .with_reclaim_permit(ReclaimPermit::new(1).unwrap())
        .with_later_handoff_policy(authority.non_claim_later_handoff()),
    )
    .unwrap()
}

pub fn base_real_chain_request(
    region: PhysicalReclaimRegion,
    reachability: ReclaimPolicyReachabilityProof,
) -> ReclaimPolicyRequest {
    ReclaimPolicyRequest::new()
        .for_region(region)
        .with_reachability(reachability)
}

pub fn execute_policy_with_observation(
    policy: AdmittedReclaimPolicy,
    observation: ReclaimPolicyExecutionObservation,
) -> Result<ReclaimPolicyExecutionReceipt, ReclaimPolicyViolation> {
    let mut executor = ObservingBackend { observation };
    ReclaimPolicyExecutionSession::for_store_backend(
        &mut executor,
        StoreOwnedReclaimPolicyExecution::for_certification_test_authority(),
    )
    .execute(policy)
    .unwrap()
}

pub fn internal_security_scope() -> ReclaimPolicySecurityScope {
    ReclaimPolicySecurityScope::from_admitted_scope(
        &worth_store_security::admitted_store_internal_security_scope_for_io_qos_test(),
    )
}

pub fn real_reachability_for_region(
    generation: u64,
    region: PhysicalReclaimRegion,
) -> ReclaimPolicyReachabilityProof {
    let world = S6ReclaimFixture::new(generation);
    let proof = ReclaimEligibilityProof::admit(
        world.executed_reachability(),
        HazardLeaseTable::with_capacity(HazardLeaseTableCapacity::bounded_slots(1).unwrap())
            .live_index_snapshot(),
        BackupReachabilityLeaseIndexSnapshot::empty(),
    )
    .unwrap();
    let removal = proof.admit_reachability_removal().unwrap();
    reachability_from_physical_isolation_removal(
        removal.lower_for_io_qos_reclaim_policy(region).unwrap(),
        region,
    )
    .unwrap()
}

pub fn reachability_from_physical_isolation_removal(
    evidence: S6ReclaimReachabilityRemovalEvidence,
    requested_region: PhysicalReclaimRegion,
) -> Result<ReclaimPolicyReachabilityProof, ReclaimPolicyReachabilityDenial> {
    ReclaimPolicyReachabilityProof::from_physical_isolation_reclaim_reachability_removal(
        evidence,
        requested_region,
    )
}

pub fn admitted_backend() -> worth_store_physical_backend::AdmittedBackendCapabilityWitness {
    PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(BackendCapabilityAdmissionRequest::new(
            BackendTargetProfile::PosixFileFsyncDirSync,
            BackendCapabilityEvidenceBasis::certified_backend_profile(),
            BackendCapabilitySupportSet::all_supported(),
            BackendMediaAssumptionSet::platform_file_defaults()
                .with_trim_posture()
                .with_punch_hole_posture()
                .with_sparse_posture()
                .with_cold_tier_io_posture(),
            BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
        ))
        .unwrap()
}

pub fn backend_without_reclaim_posture(
) -> worth_store_physical_backend::AdmittedBackendCapabilityWitness {
    PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(BackendCapabilityAdmissionRequest::new(
            BackendTargetProfile::PosixFileFsyncDirSync,
            BackendCapabilityEvidenceBasis::certified_backend_profile(),
            BackendCapabilitySupportSet::all_supported(),
            BackendMediaAssumptionSet::platform_file_defaults(),
            BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
        ))
        .unwrap()
}

pub fn region_for_generation(generation: u64) -> PhysicalReclaimRegion {
    PhysicalReclaimRegion::new(reference_for_generation(generation), 4096).unwrap()
}

pub fn reference_for_generation(generation: u64) -> PhysicalReference {
    let cell = PhysicalGenerationAuthority::for_canonical_physical_format()
        .slot_cell(
            PhysicalSegmentId::from_raw(17).unwrap(),
            PhysicalPageId::from_raw(23).unwrap(),
            PhysicalRecordSlot::from_raw(1).unwrap(),
        )
        .with_slot_generation(PhysicalGeneration::from_raw(generation).unwrap());
    PhysicalReferenceAuthority::for_canonical_physical_format()
        .admit_page_slot(cell)
        .reference()
}
