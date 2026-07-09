use super::reclaim_policy::{S6ReclaimPolicyEvidenceOutcomeKind, S6ReclaimPolicyEvidenceRow};
use worth_foundational::{
    aspects, AspectContract, AspectKey, AspectValue, InternedString, ScalarAspectType,
};
use worth_proof::TransitionOutcome;
use worth_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
    StorePhysicalBoundaryWitness,
};
use worth_store_authority::{require_current_store_authority, StoreCurrentAuthorityWitness};
use worth_store_blob_chunks::{
    BlobChunkSecurityMetadataWitness, S6BlobReclaimNonClaimHandoff, S7BlobChunkSecurityHandoff,
};
use worth_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};
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
use worth_store_readiness::{
    accept_s5_1_admitted_security_scope_readiness, S51SecurityScopeReadinessReservation,
};
use worth_store_reclaim_policy::{
    PhysicalStoreReclaimPolicyExecutor, ReclaimPermit, ReclaimPolicyAdmission,
    ReclaimPolicyExecutionObservation, ReclaimPolicyExecutionRequest,
    ReclaimPolicyExecutionSession, ReclaimPolicyProofAuthority, ReclaimPolicyReachabilityProof,
    ReclaimPolicyRequest, ReclaimPolicySecurityScope, StoreOwnedReclaimPolicyExecution,
};
use worth_store_security::{
    admit_store_security_scope, admitted_store_internal_security_scope_for_s6_test,
    StoreAdmittedSecurityScope, StoreAuthenticityRequirement, StoreAuthenticityRequirementClass,
    StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture,
    StoreSecurityScopeAdmissionExpectation, StoreSecurityScopeAdmissionRequest, StoreTenantScope,
};
use worth_store_tiering::S6ColdTierIoPosture;

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

    let (blob_scope, blob_metadata) =
        blob_reclaim_security_scope_and_metadata("store.s7.phase2.cert.reclaim");
    let blob_admitted =
        ReclaimPolicyAdmission::admit(authority, request_with_security_scope(&backend, blob_scope))
            .unwrap();
    let blob_receipt = ReclaimPolicyExecutionSession::for_store_backend(
        &mut executor,
        StoreOwnedReclaimPolicyExecution::for_certification_test_authority(),
    )
    .execute(blob_admitted)
    .unwrap()
    .unwrap();
    let blob = S6BlobReclaimNonClaimHandoff::from_reclaim_receipt(blob_receipt, blob_metadata)
        .expect("matching blob metadata should bind reclaim handoff");
    assert!(!blob.carries_blob_lifecycle_claim());
    assert_eq!(blob.security_metadata(), blob_metadata);
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

#[test]
fn blob_reclaim_non_claim_handoff_denies_mismatched_security_metadata() {
    let backend = admitted_backend();
    let authority = ReclaimPolicyProofAuthority::for_admitted_backend(&backend);
    let (receipt_scope, receipt_metadata) =
        blob_reclaim_security_scope_and_metadata("store.s7.phase2.cert.reclaim.receipt");
    let copied_admitted = admitted_blob_security_scope(
        "store.s7.phase2.cert.reclaim.copied",
        StoreTenantScope::MultiTenantPhysicalBoundary,
    );
    let copied_readiness = accept_s5_1_admitted_security_scope_readiness(
        S51SecurityScopeReadinessReservation::blob_chunk(),
        copied_admitted,
    );
    let copied_metadata = S7BlobChunkSecurityHandoff::from_s5_1_readiness(copied_readiness)
        .expect("copied blob metadata should admit")
        .permission()
        .metadata();
    let admitted = ReclaimPolicyAdmission::admit(
        authority,
        request_with_security_scope(&backend, receipt_scope),
    )
    .unwrap();
    let mut executor = ObservingBackend;
    let receipt = ReclaimPolicyExecutionSession::for_store_backend(
        &mut executor,
        StoreOwnedReclaimPolicyExecution::for_certification_test_authority(),
    )
    .execute(admitted)
    .unwrap()
    .unwrap();

    let admitted =
        S6BlobReclaimNonClaimHandoff::from_reclaim_receipt(receipt.clone(), receipt_metadata)
            .expect("matching blob metadata should bind reclaim handoff");
    assert_eq!(admitted.security_metadata(), receipt_metadata);

    let denial = S6BlobReclaimNonClaimHandoff::from_reclaim_receipt(receipt, copied_metadata)
        .expect_err("copied blob metadata must not bind reclaim handoff");
    assert_eq!(denial.receipt_scope(), receipt_metadata.identity());
    assert_eq!(denial.metadata_scope(), copied_metadata.identity());
    assert_eq!(denial.receipt(), receipt_metadata.receipt());
    assert_eq!(denial.metadata_receipt(), copied_metadata.receipt());
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
    backend: &worth_store_physical_backend::AdmittedBackendCapabilityWitness,
) -> ReclaimPolicyRequest {
    request_with_security_scope(
        backend,
        ReclaimPolicySecurityScope::from_admitted_scope(
            &admitted_store_internal_security_scope_for_s6_test(),
        ),
    )
}

fn request_with_security_scope(
    backend: &worth_store_physical_backend::AdmittedBackendCapabilityWitness,
    security_scope: ReclaimPolicySecurityScope,
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
        .with_security_scope(security_scope)
        .with_reclaim_permit(ReclaimPermit::new(1).unwrap())
        .with_later_handoff_policy(authority.non_claim_later_handoff())
}

fn blob_reclaim_security_scope_and_metadata(
    identity_key: &str,
) -> (ReclaimPolicySecurityScope, BlobChunkSecurityMetadataWitness) {
    let admitted =
        admitted_blob_security_scope(identity_key, StoreTenantScope::TenantPhysicalBoundary);
    let reclaim_scope = ReclaimPolicySecurityScope::from_admitted_scope(&admitted);
    let readiness = accept_s5_1_admitted_security_scope_readiness(
        S51SecurityScopeReadinessReservation::blob_chunk(),
        admitted,
    );
    let handoff = S7BlobChunkSecurityHandoff::from_s5_1_readiness(readiness).expect("blob handoff");
    (reclaim_scope, handoff.permission().metadata())
}

fn admitted_blob_security_scope(
    identity_key: &str,
    tenant_scope: StoreTenantScope,
) -> StoreAdmittedSecurityScope {
    let authority = current_authority(identity_key, "chunk-authority");
    let authenticity = StoreAuthenticityRequirement::required(
        StoreAuthenticityRequirementClass::AuthenticatedBlobChunk,
    );
    let expectation = StoreSecurityScopeAdmissionExpectation::new(
        StoreKeyScope::BlobChunkEnvelope,
        tenant_scope,
        authenticity,
        StoreCustodyPosture::InternalStoreCustody,
    );
    let request = StoreSecurityScopeAdmissionRequest::new(
        &authority,
        StoreKeyScope::BlobChunkEnvelope,
        StoreKeyVersionPosture::Current,
        tenant_scope,
        authenticity,
        StoreCustodyPosture::InternalStoreCustody,
        expectation,
    );

    match admit_store_security_scope(request) {
        TransitionOutcome::Success(admitted) => admitted,
        outcome => panic!("blob security scope should admit: {outcome:?}"),
    }
}

fn current_authority(identity_key: &str, value: &str) -> StoreCurrentAuthorityWitness {
    require_current_store_authority(boundary_fact(identity_key, value))
}

fn boundary_fact(identity_key: &str, value: &str) -> StoreAspectBoundaryFact {
    let key = aspect_key(identity_key);
    let contract = scalar_string_contract(key.clone());
    let admitted_state = match aspects()
        .authoritative_state()
        .admit([validated_scalar_value(&contract, value)])
    {
        TransitionOutcome::Success(state) => state,
        outcome => panic!("state admission should succeed: {outcome:?}"),
    };

    StoreAspectBoundaryFact::from_admitted_state(
        StoreAspectIdentity::from_aspect_key(key),
        StoreAspectAuthorityInput::new(admitted_state, physical_witness()),
    )
    .expect("Store boundary fact should admit matching identity")
}

fn aspect_key(raw: &str) -> AspectKey {
    aspects().vocabulary().key(raw).unwrap()
}

fn scalar_string_contract(aspect_key: AspectKey) -> AspectContract {
    aspects()
        .contract()
        .for_key(aspect_key)
        .identified_by(aspects().vocabulary().identity(1))
        .at_revision(aspects().vocabulary().revision(1))
        .scalar(ScalarAspectType::String)
}

fn validated_scalar_value(
    contract: &AspectContract,
    value: &str,
) -> worth_foundational::ContractValidatedAspectArtifact {
    match aspects()
        .validate()
        .against(contract)
        .value(AspectValue::String(InternedString::from(value)))
    {
        TransitionOutcome::Success(validated) => validated,
        outcome => panic!("aspect validation should succeed: {outcome:?}"),
    }
}

fn physical_witness() -> StorePhysicalBoundaryWitness {
    StorePhysicalBoundaryWitness::from_physical_authority(
        StorePhysicalAuthorityWitness::for_aspect_native_boundary(
            ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
        )
        .unwrap(),
    )
    .unwrap()
}

fn admitted_backend() -> worth_store_physical_backend::AdmittedBackendCapabilityWitness {
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
