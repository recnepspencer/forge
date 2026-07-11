use forge_store_io_scheduler::admit_store_published_isolation_capability;
use forge_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalReclaimRegion,
    PhysicalRecordSlot, PhysicalReferenceAdmissionWitness, PhysicalReferenceAuthority,
    PhysicalSegmentId, ReclaimedByteInterpretation,
};
use forge_store_physical_isolation::{
    publish_scheduler_isolation_capability_for_certification_test,
    GenerationCountedPhysicalReference, ReclaimEligibilityProof,
};
use forge_store_reclaim_policy::{
    PhysicalStoreReclaimPolicyExecutor, ReclaimPermit, ReclaimPolicyAdmission,
    ReclaimPolicyExecutionObservation, ReclaimPolicyExecutionReceipt,
    ReclaimPolicyExecutionRequest, ReclaimPolicyExecutionSession, ReclaimPolicyProofAuthority,
    ReclaimPolicyReachabilityProof, ReclaimPolicyRequest, ReclaimPolicySecurityScope,
};
use forge_store_security::{
    admit_store_security_scope, StoreAdmittedSecurityScope, StoreSecurityScopeAdmissionExpectation,
    StoreSecurityScopeAdmissionRequest, StoreSecurityScopeIdentity,
};
use forge_store_tiering::admit_tier_placement_io;

use super::super::backend::{admitted_backend, current_authority};

pub(in crate::harness_execution) fn placement_readiness(
    security_scope: StoreSecurityScopeIdentity,
) -> forge_store_tiering::TierPlacementIoAdmission {
    let published_readiness = publish_scheduler_isolation_capability_for_certification_test(2, 1)
        .expect("published scheduler readiness");
    let scheduler_readiness = admit_store_published_isolation_capability(&published_readiness)
        .expect("scheduler readiness");
    admit_tier_placement_io(
        scheduler_readiness,
        forge_store_tiering::ColdTierIoPosture::from_reclaim_receipt(cold_tier_reclaim_receipt(
            security_scope,
        ))
        .expect("cold-tier posture"),
    )
}

fn cold_tier_reclaim_receipt(
    security_scope: StoreSecurityScopeIdentity,
) -> ReclaimPolicyExecutionReceipt {
    let backend = admitted_backend();
    let authority = ReclaimPolicyProofAuthority::for_admitted_backend(&backend);
    let physical_reference = current_physical_reference_raw(1);
    let region =
        PhysicalReclaimRegion::new(physical_reference.reference(), 4096).expect("reclaim region");
    let reachability = lower_physical_isolation_reclaim_reachability_for_region(region, physical_reference);
    let request = ReclaimPolicyRequest::new()
        .for_region(region)
        .with_posture(
            authority
                .cold_tier_io_posture(ReclaimedByteInterpretation::NonObservableReclaimedStorage)
                .expect("cold posture"),
        )
        .with_reachability(reachability)
        .with_security_scope(ReclaimPolicySecurityScope::from_admitted_scope(
            &admitted_security_scope_for_identity(security_scope),
        ))
        .with_reclaim_permit(ReclaimPermit::new(1).expect("reclaim permit"))
        .with_later_handoff_policy(authority.non_claim_later_handoff());
    let policy = ReclaimPolicyAdmission::admit(authority, request).expect("reclaim policy");
    let mut executor = ObservingReclaimBackend;
    ReclaimPolicyExecutionSession::for_owned_backend(&mut executor)
        .execute(policy)
        .expect("reclaim execution")
        .expect("reclaim receipt")
}

fn lower_physical_isolation_reclaim_reachability_for_region(
    region: PhysicalReclaimRegion,
    physical_reference: PhysicalReferenceAdmissionWitness,
) -> ReclaimPolicyReachabilityProof {
    let current_generation =
        GenerationCountedPhysicalReference::from_admitted_reference(physical_reference)
            .require_current_generation(physical_reference.reference().generation())
            .expect("current generation reference");
    let removal_receipt = ReclaimEligibilityProof::for_certification_reference(current_generation)
        .admit_reachability_removal()
        .expect("reclaim reachability removal");
    let evidence = removal_receipt
        .lower_for_io_qos_reclaim_policy(region)
        .expect("reclaim policy evidence");
    ReclaimPolicyReachabilityProof::from_physical_isolation_reclaim_reachability_removal(evidence, region)
        .expect("reclaim reachability proof")
}

fn admitted_security_scope_for_identity(
    identity: StoreSecurityScopeIdentity,
) -> StoreAdmittedSecurityScope {
    let authority = current_authority("blob-harness-cold-tier-scope", "reclaim-policy");
    let expectation = StoreSecurityScopeAdmissionExpectation::new(
        identity.key_scope(),
        identity.tenant_scope(),
        identity.authenticity_requirement(),
        identity.custody_posture(),
    );
    match admit_store_security_scope(StoreSecurityScopeAdmissionRequest::new(
        &authority,
        identity.key_scope(),
        identity.key_version_posture(),
        identity.tenant_scope(),
        identity.authenticity_requirement(),
        identity.custody_posture(),
        expectation,
    )) {
        forge_proof::TransitionOutcome::Success(scope) => scope,
        outcome => panic!("security scope admission: {outcome:?}"),
    }
}

struct ObservingReclaimBackend;

impl PhysicalStoreReclaimPolicyExecutor for ObservingReclaimBackend {
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

fn current_physical_reference_raw(slot: u16) -> PhysicalReferenceAdmissionWitness {
    let generation = PhysicalGeneration::from_raw(7).expect("generation");
    PhysicalReferenceAuthority::for_canonical_physical_format().admit_page_slot(
        PhysicalGenerationAuthority::for_canonical_physical_format()
            .slot_cell(
                PhysicalSegmentId::from_raw(1).expect("segment"),
                PhysicalPageId::from_raw(1).expect("page"),
                PhysicalRecordSlot::from_raw(slot).expect("slot"),
            )
            .with_slot_generation(generation),
    )
}
