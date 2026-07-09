use worth_foundational::{
    aspects, AspectContract, AspectKey, AspectValue, InternedString, ScalarAspectType,
};
use worth_proof::TransitionOutcome;
use worth_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
    StorePhysicalBoundaryWitness,
};
use worth_store_authority::{require_current_store_authority, StoreCurrentAuthorityWitness};
use worth_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};
use worth_store_physical_format::{
    AllocationClassKind, PhysicalBinaryEncodingWitness, PhysicalDecodedHeader, PhysicalExtentId,
    PhysicalFrameHeader, PhysicalFrameKind, PhysicalGeneration, PhysicalGenerationAuthority,
    PhysicalHeaderAuthority, PhysicalPageHeader, PhysicalPageId, PhysicalPageKind,
    PhysicalPublicationState, PhysicalRecordSlot, PhysicalReferenceAuthority, PhysicalRootManifest,
    PhysicalRootReference, PhysicalSegmentId, PHYSICAL_HEADER_LENGTH,
};

use crate::{
    admit_store_security_scope, StoreAuthenticityRequirement, StoreAuthenticityRequirementClass,
    StoreCurrentSecurityScopeWitnessSet, StoreCustodyPosture, StoreKeyScope,
    StoreKeyVersionPosture, StoreLegacySecurityPosture, StorePhysicalSecurityMetadataCarrier,
    StoreSecurityScopeAdmissionRequest, StoreTenantScope,
};

pub(super) fn current_authority(identity_key: &str, value: &str) -> StoreCurrentAuthorityWitness {
    require_current_store_authority(boundary_fact(identity_key, value))
}

pub(super) fn admitted_scope(
    authority: &StoreCurrentAuthorityWitness,
) -> StoreCurrentSecurityScopeWitnessSet {
    let request = StoreSecurityScopeAdmissionRequest::platform_page_envelope(
        authority,
        StoreKeyVersionPosture::Current,
        StoreCustodyPosture::InternalStoreCustody,
    );
    let admitted = match admit_store_security_scope(request) {
        TransitionOutcome::Success(admitted) => admitted,
        outcome => panic!("platform scope should admit: {outcome:?}"),
    };
    admitted.into_witnesses_for_readiness_handoff()
}

pub(super) fn assert_platform_metadata(
    metadata: StorePhysicalSecurityMetadataCarrier,
    legacy_posture: StoreLegacySecurityPosture,
    key_version_posture: StoreKeyVersionPosture,
) {
    assert_eq!(metadata.key_scope(), StoreKeyScope::PageEnvelope);
    assert_eq!(
        metadata.tenant_scope(),
        StoreTenantScope::TenantPhysicalBoundary
    );
    assert_eq!(
        metadata.authenticity_requirement(),
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        )
    );
    assert_eq!(
        metadata.custody_posture(),
        StoreCustodyPosture::InternalStoreCustody
    );
    assert_eq!(metadata.legacy_posture(), legacy_posture);
    assert_eq!(metadata.key_version_posture(), key_version_posture);
}

pub(super) fn decoded_page_header(generation_value: u64) -> PhysicalPageHeader {
    let cell = PhysicalGenerationAuthority::s1()
        .page_cell(segment(1), page(2))
        .with_page_generation(generation(generation_value));
    let report = header_authority()
        .decode_page_header(
            cell,
            &header_bytes(
                PhysicalPageKind::DataPage.tag(),
                generation_value,
                PhysicalPublicationState::Published,
                b"page",
            ),
            PhysicalPageKind::DataPage,
        )
        .unwrap();
    match report.witness().header() {
        PhysicalDecodedHeader::Page(header) => header,
        PhysicalDecodedHeader::Frame(_) => panic!("expected decoded page header"),
    }
}

pub(super) fn decoded_frame_header(generation_value: u64) -> PhysicalFrameHeader {
    let generations = PhysicalGenerationAuthority::s1();
    let references = PhysicalReferenceAuthority::s1();
    let cell = generations
        .slot_cell(segment(1), page(2), slot(3))
        .with_slot_generation(generation(generation_value));
    let admitted = references.admit_page_slot(cell);
    let reference = references.validate_page_slot(admitted, cell).unwrap();
    let report = header_authority()
        .decode_frame_header(
            reference,
            &header_bytes(
                PhysicalFrameKind::RecordFrame.tag(),
                generation_value,
                PhysicalPublicationState::Published,
                b"frame",
            ),
            PhysicalFrameKind::RecordFrame,
        )
        .unwrap();
    match report.witness().header() {
        PhysicalDecodedHeader::Frame(header) => header,
        PhysicalDecodedHeader::Page(_) => panic!("expected decoded frame header"),
    }
}

pub(super) fn root_manifest_with_all_entry_kinds() -> PhysicalRootManifest {
    let generations = PhysicalGenerationAuthority::s1();
    let root_cell = generations
        .root_publication_cell(PhysicalRootReference::from_raw(9).unwrap())
        .with_root_publication_generation(generation(1));
    let segment_cell = generations
        .segment_cell(segment(1))
        .with_segment_generation(generation(2));
    let slot_cell = generations
        .slot_cell(segment(1), page(2), slot(3))
        .with_slot_generation(generation(3));
    let extent_cell = generations
        .extent_cell(segment(1), extent(4))
        .with_extent_generation(generation(4));
    let free_space_cell = generations
        .free_space_slot_cell(
            segment(1),
            page(2),
            slot(5),
            AllocationClassKind::OrdinaryRecordPage,
        )
        .unwrap()
        .with_free_space_generation(generation(5));

    worth_store_physical_format::PhysicalManifestUniverseBuilder::s1(root_cell)
        .segment(segment_cell)
        .ordinary_page(slot_cell)
        .extent(extent_cell)
        .free_space_reuse(free_space_cell)
        .publish()
}

pub(super) fn physical_witness() -> StorePhysicalBoundaryWitness {
    StorePhysicalBoundaryWitness::from_physical_authority(
        StorePhysicalAuthorityWitness::for_aspect_native_boundary(
            ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
        )
        .unwrap(),
    )
    .unwrap()
}

fn boundary_fact(identity_key: &str, value: &str) -> StoreAspectBoundaryFact {
    let key = aspects().vocabulary().key(identity_key).unwrap();
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

fn header_authority() -> PhysicalHeaderAuthority {
    PhysicalHeaderAuthority::s1(PhysicalBinaryEncodingWitness::s1_canonical().unwrap())
}

fn header_bytes(
    kind_tag: u8,
    generation_value: u64,
    publication: PhysicalPublicationState,
    payload: &[u8],
) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(PHYSICAL_HEADER_LENGTH as usize + payload.len());
    bytes.push(kind_tag);
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&PHYSICAL_HEADER_LENGTH.to_le_bytes());
    bytes.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&generation_value.to_le_bytes());
    bytes.push(publication.code());
    bytes.extend_from_slice(&0u32.to_le_bytes());
    bytes.extend_from_slice(&0u64.to_le_bytes());
    bytes.extend_from_slice(payload);
    bytes
}

fn segment(value: u64) -> PhysicalSegmentId {
    PhysicalSegmentId::from_raw(value).unwrap()
}

fn page(value: u64) -> PhysicalPageId {
    PhysicalPageId::from_raw(value).unwrap()
}

fn extent(value: u64) -> PhysicalExtentId {
    PhysicalExtentId::from_raw(value).unwrap()
}

fn slot(value: u16) -> PhysicalRecordSlot {
    PhysicalRecordSlot::from_raw(value).unwrap()
}

fn generation(value: u64) -> PhysicalGeneration {
    PhysicalGeneration::from_raw(value).unwrap()
}
