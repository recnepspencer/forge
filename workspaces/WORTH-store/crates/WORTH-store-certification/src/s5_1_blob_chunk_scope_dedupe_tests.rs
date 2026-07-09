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
    BlobChunkDedupeAdmission, BlobChunkDedupeAdmissionDenial, BlobChunkDedupeCandidate,
    BlobChunkSecurityScope, BlobChunkSecurityScopeDenial, BlobChunkSequenceAdmission,
    BlobChunkSize, BlobChunkStreamingDenial, BlobChunkStreamingResidencyProof,
    BlobChunkingRuleAdmission, S7BlobChunkSecurityHandoff,
};
use worth_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};
use worth_store_physical_format::{
    PhysicalBinaryEncodingWitness, PhysicalChunkChecksumAuthority, PhysicalGeneration,
    PhysicalGenerationAuthority, PhysicalHeaderAuthority, PhysicalPageId, PhysicalPageKind,
    PhysicalPageRecordAuthority, PhysicalPublicationState, PhysicalRecordSlot,
    PhysicalReferenceAuthority, PhysicalSegmentId, SlotAppendRequest,
    StorePhysicalChunkWriteReceipt, PHYSICAL_HEADER_LENGTH,
};
use worth_store_readiness::{
    accept_s5_1_admitted_security_scope_readiness, S51SecurityScopeReadinessReservation,
};
use worth_store_security::{
    admit_store_security_scope, StoreAdmittedSecurityScope, StoreAuthenticityRequirement,
    StoreAuthenticityRequirementClass, StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture,
    StoreSecurityScopeAdmissionExpectation, StoreSecurityScopeAdmissionRequest, StoreTenantScope,
};

#[test]
fn s5_1_blob_chunk_scope_and_dedupe_readiness_public_api_courtroom() {
    let admitted_blob_scope = blob_scope(
        "cert.s51.blob.scope",
        StoreTenantScope::TenantPhysicalBoundary,
    );
    assert_eq!(
        admitted_blob_scope.key_scope(),
        StoreKeyScope::BlobChunkEnvelope
    );
    assert_eq!(admitted_blob_scope.counters().admitted_scope_consumed(), 1);

    let backup_readiness = blob_readiness_for(
        "cert.s51.blob.backup_tenant",
        StoreTenantScope::BackupRestoreBoundary,
    );
    assert!(matches!(
        S7BlobChunkSecurityHandoff::from_s5_1_readiness(backup_readiness),
        Err(BlobChunkSecurityScopeDenial::WrongTenantScope { counters, .. })
            if counters.denials() == 1
    ));

    let existing = candidate_for_scope(blob_scope(
        "cert.s51.blob.dedupe.left",
        StoreTenantScope::TenantPhysicalBoundary,
    ));
    let candidate = candidate_for_scope(blob_scope(
        "cert.s51.blob.dedupe.left",
        StoreTenantScope::TenantPhysicalBoundary,
    ));
    assert!(matches!(
        BlobChunkDedupeAdmission::compare_candidates(existing, candidate).admit(),
        TransitionOutcome::Denied(
            BlobChunkDedupeAdmissionDenial::MissingFoundationalCanonicalEquivalence { counters }
        ) if counters.digest_only_denials() == 1
    ));

    let tenant_left = candidate_for_scope(blob_scope(
        "cert.s51.blob.tenant.left",
        StoreTenantScope::TenantPhysicalBoundary,
    ));
    let tenant_right = candidate_for_scope(blob_scope(
        "cert.s51.blob.tenant.right",
        StoreTenantScope::MultiTenantPhysicalBoundary,
    ));
    assert!(matches!(
        BlobChunkDedupeAdmission::compare_candidates(tenant_left, tenant_right).admit(),
        TransitionOutcome::Denied(
            BlobChunkDedupeAdmissionDenial::CrossTenantScopeRequiresExplicitEquivalence {
                counters,
                ..
            }
        ) if counters.cross_scope_denials() == 1
    ));

    assert!(matches!(
        BlobChunkStreamingResidencyProof::bounded_window(2048, 2048),
        Err(BlobChunkStreamingDenial::WholeObjectResidencyRequired)
    ));
}

fn candidate_for_scope(scope: BlobChunkSecurityScope) -> BlobChunkDedupeCandidate {
    let bytes = b"cert-s51-blob-content";
    let rule = BlobChunkingRuleAdmission::fixed_size(
        BlobChunkSize::from_bytes(bytes.len() as u64).expect("nonempty chunk size"),
    )
    .expect("fixed-size rule should admit");
    let physical_receipt = record_receipt(bytes);
    let payload = PhysicalChunkChecksumAuthority::s7_canonical()
        .admit_store_payload(physical_receipt)
        .expect("payload should admit");
    let sequence = BlobChunkSequenceAdmission::start(scope, rule, bytes.len() as u64)
        .expect("sequence should start")
        .push_payload(0, payload)
        .expect("window should admit into sequence")
        .finish()
        .expect("sequence should finish");
    BlobChunkDedupeCandidate::from_integrity_proof(sequence.first_chunk().clone())
}

fn blob_scope(identity_key: &str, tenant_scope: StoreTenantScope) -> BlobChunkSecurityScope {
    let handoff = S7BlobChunkSecurityHandoff::from_s5_1_readiness(blob_readiness_for(
        identity_key,
        tenant_scope,
    ))
    .expect("blob handoff should admit");
    BlobChunkSecurityScope::from_s7_handoff(handoff)
}

fn blob_readiness_for(
    identity_key: &str,
    tenant_scope: StoreTenantScope,
) -> worth_store_readiness::S51AdmittedSecurityScopeReadiness {
    accept_s5_1_admitted_security_scope_readiness(
        S51SecurityScopeReadinessReservation::blob_chunk(),
        admitted_blob_security_scope(identity_key, tenant_scope),
    )
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
    raw_value: &str,
) -> worth_foundational::ContractValidatedAspectArtifact {
    match aspects()
        .validate()
        .against(contract)
        .value(AspectValue::String(InternedString::from(raw_value)))
    {
        TransitionOutcome::Success(value) => value,
        outcome => panic!("validation should succeed: {outcome:?}"),
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

fn record_receipt(bytes: &[u8]) -> StorePhysicalChunkWriteReceipt {
    let records = record_authority();
    let generations = PhysicalGenerationAuthority::s1();
    let references = PhysicalReferenceAuthority::s1();
    let page_cell = generations
        .page_cell(segment(7), page(11))
        .with_page_generation(generation(5));
    let slot_cell = generations
        .slot_cell(segment(7), page(11), slot(1))
        .with_slot_generation(generation(9));
    let empty_page = page_bytes(generation(5), &[]);
    let append = records
        .append_record(
            admitted_page(&records, page_cell, &empty_page),
            SlotAppendRequest::ordinary(slot_cell, bytes),
        )
        .expect("physical record append should execute");
    let reopened_page = page_bytes(generation(5), append.page_payload());
    let validation = references
        .validate_page_slot(append.reference_admission(), slot_cell)
        .expect("physical reference should validate");
    let located = records
        .locate_record(
            admitted_page(&records, page_cell, &reopened_page),
            validation,
        )
        .expect("physical record locate should execute");
    StorePhysicalChunkWriteReceipt::from_page_record_view(located.record_view())
        .expect("physical record view should admit chunk receipt")
}

fn admitted_page<'a>(
    records: &PhysicalPageRecordAuthority,
    page_cell: worth_store_physical_format::PageGenerationCell,
    bytes: &'a [u8],
) -> worth_store_physical_format::RecordPagePayload<'a> {
    let header = records
        .decode_record_page_header(page_cell, bytes, PhysicalPageKind::DataPage)
        .expect("record page header should decode");
    records
        .admit_record_page_payload(bytes, header.witness())
        .expect("record page payload should admit")
}

fn record_authority() -> PhysicalPageRecordAuthority {
    PhysicalPageRecordAuthority::s1(PhysicalHeaderAuthority::s1(
        PhysicalBinaryEncodingWitness::s1_canonical().expect("canonical physical binary format"),
    ))
}

fn page_bytes(generation: PhysicalGeneration, payload: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(PHYSICAL_HEADER_LENGTH as usize + payload.len());
    bytes.push(PhysicalPageKind::DataPage.tag());
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&PHYSICAL_HEADER_LENGTH.to_le_bytes());
    bytes.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&generation.get().to_le_bytes());
    bytes.push(PhysicalPublicationState::Published.code());
    bytes.extend_from_slice(&0u32.to_le_bytes());
    bytes.extend_from_slice(&0u64.to_le_bytes());
    bytes.extend_from_slice(payload);
    bytes
}

fn segment(value: u64) -> PhysicalSegmentId {
    PhysicalSegmentId::from_raw(value).expect("segment id")
}

fn page(value: u64) -> PhysicalPageId {
    PhysicalPageId::from_raw(value).expect("page id")
}

fn slot(value: u16) -> PhysicalRecordSlot {
    PhysicalRecordSlot::from_raw(value).expect("record slot")
}

fn generation(value: u64) -> PhysicalGeneration {
    PhysicalGeneration::from_raw(value).expect("generation")
}
