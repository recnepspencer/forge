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
    stable_physical_read_plan_for_certification_test, BlobOrphanReclaimBarrier,
    BlobPartialChunkOrphan, CurrentGenerationPhysicalReference, GenerationCountedPhysicalReference,
};
use worth_store_reclaim_policy::{
    PhysicalStoreReclaimPolicyExecutor, ReclaimPermit, ReclaimPolicyAdmission,
    ReclaimPolicyExecutionObservation, ReclaimPolicyExecutionRequest,
    ReclaimPolicyExecutionSession, ReclaimPolicyProofAuthority, ReclaimPolicyReachabilityProof,
    ReclaimPolicyRequest, ReclaimPolicySecurityScope, StoreOwnedReclaimPolicyExecution,
};
use worth_store_security::StoreTenantScope;

use crate::publication::test_support::publish_generation_with_bytes_and_chunk_size;
use crate::test_support::{admitted_blob_security_scope, admitted_sequence_for_scope, blob_scope};
use crate::{
    BlobChunkIdentity, BlobChunkReachabilityRegistry, BlobChunkSecurityMetadataWitness,
    BlobReachabilityEdge, BlobReachabilityReclaimDecision, BlobRetentionHoldKind,
    BlobRetentionReclaimAdmission, BlobRetentionReclaimAdmissionAuthority,
    BlobRetentionReclaimDenial, BlobRetentionReclaimOutcome, BlobRetentionReclaimRequest,
    BlobRetentionSafeReclaimPlanner, S6BlobReclaimNonClaimHandoff,
};

pub(crate) fn plan(request: BlobRetentionReclaimRequest) -> BlobRetentionReclaimOutcome {
    BlobRetentionSafeReclaimPlanner::new_store_owned().plan_reclaim(request)
}

pub(crate) fn reclaim_fixture(case: &str, physical_slot: u16) -> BlobRetentionReclaimAdmission {
    let (registry, chunk_identity, metadata) = reachability_registry(case);
    let s6 = s6_handoff_for_metadata(
        case,
        current_physical_reference_raw(physical_slot),
        metadata,
    );
    BlobRetentionReclaimAdmissionAuthority::store_owned()
        .admit_reachability_orphan(&registry, &chunk_identity, s6)
        .expect("admission should bind current reachability, physical orphan identity, and S.6")
}

pub(crate) fn mismatched_abandoned_resume_barrier_admission(
    case: &str,
    s6_slot: u16,
    barrier_slot: u16,
) -> Result<BlobRetentionReclaimAdmission, BlobRetentionReclaimDenial> {
    let (registry, chunk_identity, metadata) = reachability_registry(case);
    let s6 = s6_handoff_for_metadata(case, current_physical_reference_raw(s6_slot), metadata);
    let barrier = resume_barrier_for_chunk(&chunk_identity, barrier_slot);
    BlobRetentionReclaimAdmissionAuthority::store_owned().admit_abandoned_resume_orphan(
        &registry,
        &chunk_identity,
        &barrier,
        s6,
    )
}

pub(crate) fn mismatched_scope_admission(
    case: &str,
    physical_slot: u16,
) -> Result<BlobRetentionReclaimAdmission, BlobRetentionReclaimDenial> {
    let (registry, chunk_identity, _) = reachability_registry(case);
    let region_reference = current_physical_reference_raw(physical_slot);
    let tenant_scope = StoreTenantScope::MultiTenantPhysicalBoundary;
    let scope = blob_scope(case, tenant_scope);
    let admitted = admitted_blob_security_scope(case, tenant_scope);
    let s6 = s6_handoff_from_parts(region_reference, scope.metadata(), &admitted);
    BlobRetentionReclaimAdmissionAuthority::store_owned().admit_reachability_orphan(
        &registry,
        &chunk_identity,
        s6,
    )
}

pub(crate) fn live_read_hold_admission(
    case: &str,
    physical_slot: u16,
) -> Result<BlobRetentionReclaimAdmission, BlobRetentionReclaimDenial> {
    let (mut registry, chunk_identity, metadata) = reachability_registry(case);
    registry
        .admit_stable_read_plan_hold(&stable_physical_read_plan_for_certification_test(64))
        .expect("live read hold should bind to current registry authority");
    let s6 = s6_handoff_for_metadata(
        case,
        current_physical_reference_raw(physical_slot),
        metadata,
    );
    BlobRetentionReclaimAdmissionAuthority::store_owned().admit_reachability_orphan(
        &registry,
        &chunk_identity,
        s6,
    )
}

pub(crate) fn retention_hold_admission(
    case: &str,
    physical_slot: u16,
    hold: crate::BlobRetentionHold,
) -> Result<BlobRetentionReclaimAdmission, BlobRetentionReclaimDenial> {
    let (mut registry, chunk_identity, metadata) = reachability_registry(case);
    registry
        .admit_retention_hold(&hold)
        .expect("retention hold should bind to current registry authority");
    let s6 = s6_handoff_for_metadata(
        case,
        current_physical_reference_raw(physical_slot),
        metadata,
    );
    BlobRetentionReclaimAdmissionAuthority::store_owned().admit_reachability_orphan(
        &registry,
        &chunk_identity,
        s6,
    )
}

fn s6_handoff_for_metadata(
    case: &str,
    region_reference: PhysicalReference,
    metadata: BlobChunkSecurityMetadataWitness,
) -> S6BlobReclaimNonClaimHandoff {
    let admitted = admitted_blob_security_scope(case, StoreTenantScope::TenantPhysicalBoundary);
    s6_handoff_from_parts(region_reference, metadata, &admitted)
}

fn s6_handoff_from_parts(
    region_reference: PhysicalReference,
    metadata: crate::BlobChunkSecurityMetadataWitness,
    admitted: &worth_store_security::StoreAdmittedSecurityScope,
) -> S6BlobReclaimNonClaimHandoff {
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
    let region = PhysicalReclaimRegion::new(region_reference, 4096).expect("region should admit");
    let request = ReclaimPolicyRequest::new()
        .for_region(region)
        .with_posture(
            authority
                .cold_tier_io_posture(ReclaimedByteInterpretation::NonObservableReclaimedStorage)
                .expect("cold posture should admit"),
        )
        .with_reachability(ReclaimPolicyReachabilityProof::for_certification_test_authority(region))
        .with_security_scope(ReclaimPolicySecurityScope::from_admitted_scope(admitted))
        .with_reclaim_permit(ReclaimPermit::new(1).expect("permit count should admit"))
        .with_later_handoff_policy(authority.non_claim_later_handoff());
    let policy = ReclaimPolicyAdmission::admit(authority, request).expect("policy should admit");
    let mut executor = ObservingBackend;
    let receipt = ReclaimPolicyExecutionSession::for_store_backend(
        &mut executor,
        StoreOwnedReclaimPolicyExecution::for_certification_test_authority(),
    )
    .execute(policy)
    .expect("execution should run")
    .expect("execution should succeed");
    S6BlobReclaimNonClaimHandoff::from_reclaim_receipt(receipt, metadata)
        .expect("S.6 blob reclaim handoff should bind metadata")
}

pub(crate) fn reachability_registry(
    case: &str,
) -> (
    BlobChunkReachabilityRegistry,
    BlobChunkIdentity,
    BlobChunkSecurityMetadataWitness,
) {
    let bytes = b"phase15 retention reclaim";
    let scope = blob_scope(case, StoreTenantScope::TenantPhysicalBoundary);
    let sequence = admitted_sequence_for_scope(scope, bytes);
    let (published, _) =
        publish_generation_with_bytes_and_chunk_size(case, bytes, bytes.len() as u64);
    let leaf = sequence.proof_frontier().first_leaf();
    let edge =
        BlobReachabilityEdge::primary_blob_reference(&published, leaf).expect("edge should admit");
    let chunk_identity = leaf.identity().clone();
    let metadata = leaf.security_metadata();
    let mut registry = BlobChunkReachabilityRegistry::new_store_owned();
    registry
        .admit_edge(edge.clone())
        .expect("edge should admit");
    registry
        .release_edge(&edge)
        .expect("edge release should admit");
    match registry.reclaim_decision_for(leaf.identity()) {
        BlobReachabilityReclaimDecision::ReclaimPermitted(_) => {
            (registry, chunk_identity, metadata)
        }
        other => panic!("released edge should permit reachability-local reclaim: {other:?}"),
    }
}

fn resume_barrier_for_chunk(
    chunk_identity: &BlobChunkIdentity,
    physical_slot: u16,
) -> BlobOrphanReclaimBarrier {
    let orphan = BlobPartialChunkOrphan::unreached(
        "phase15-abandoned-resume",
        0,
        chunk_identity.chunk_digest().as_str(),
        4096,
        current_physical_reference(physical_slot),
    )
    .expect("partial orphan should admit");
    BlobOrphanReclaimBarrier::from_unreached_orphan(orphan, false)
        .expect("orphan barrier should admit")
}

pub(crate) fn hold_counter_for_kind(
    counters: crate::BlobRetentionReclaimCounterSnapshot,
    kind: BlobRetentionHoldKind,
) -> u64 {
    match kind {
        BlobRetentionHoldKind::Generation => counters.generation_hold_denials(),
        BlobRetentionHoldKind::TimeWindow => counters.time_window_hold_denials(),
        BlobRetentionHoldKind::Export => counters.export_hold_denials(),
        BlobRetentionHoldKind::Capsule => counters.capsule_hold_denials(),
        BlobRetentionHoldKind::Quarantine => counters.quarantine_hold_denials(),
        BlobRetentionHoldKind::ReadPlan => counters.read_plan_hold_denials(),
        BlobRetentionHoldKind::Checkpoint => counters.checkpoint_hold_denials(),
        BlobRetentionHoldKind::TenantCustody => counters.tenant_custody_hold_denials(),
        BlobRetentionHoldKind::ResumeSession => counters.resume_session_hold_denials(),
        BlobRetentionHoldKind::PlacementMove => counters.placement_move_hold_denials(),
        BlobRetentionHoldKind::Backup => counters.backup_hold_denials(),
    }
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

fn current_physical_reference_raw(slot: u16) -> PhysicalReference {
    let generation = PhysicalGeneration::from_raw(7).expect("generation");
    PhysicalReferenceAuthority::s1()
        .admit_page_slot(
            PhysicalGenerationAuthority::s1()
                .slot_cell(
                    PhysicalSegmentId::from_raw(1).expect("segment"),
                    PhysicalPageId::from_raw(1).expect("page"),
                    PhysicalRecordSlot::from_raw(slot).expect("slot"),
                )
                .with_slot_generation(generation),
        )
        .reference()
}

fn current_physical_reference(slot: u16) -> CurrentGenerationPhysicalReference {
    let generation = PhysicalGeneration::from_raw(7).expect("generation");
    GenerationCountedPhysicalReference::from_admitted_reference(
        PhysicalReferenceAuthority::s1().admit_page_slot(
            PhysicalGenerationAuthority::s1()
                .slot_cell(
                    PhysicalSegmentId::from_raw(1).expect("segment"),
                    PhysicalPageId::from_raw(1).expect("page"),
                    PhysicalRecordSlot::from_raw(slot).expect("slot"),
                )
                .with_slot_generation(generation),
        ),
    )
    .require_current_generation(generation)
    .expect("current generation reference")
}
