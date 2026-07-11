use forge_foundational::{
    aspects, AspectContract, AspectKey, AspectValue, InternedString, ScalarAspectType,
};
use forge_proof::TransitionOutcome;
use forge_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
    StorePhysicalBoundaryWitness,
};
use forge_store_authority::{require_current_store_authority, StoreCurrentAuthorityWitness};
use forge_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};
use forge_store_physical_format::{
    PhysicalBinaryEncodingWitness, PhysicalDecodedHeader, PhysicalGeneration,
    PhysicalGenerationAuthority, PhysicalHeaderAuthority, PhysicalPageHeader, PhysicalPageId,
    PhysicalPageKind, PhysicalPublicationState, PhysicalRecordSlot,
    PhysicalSecurityMetadataEnvelope, PhysicalSegmentId, SegmentPageManifestEntry,
    PHYSICAL_HEADER_LENGTH,
};
use forge_store_physical_isolation::{
    LogicalDecodeSecurityScopeEntry, PhysicalByteGuardScope, StablePhysicalReadHandle,
    StableReadSecurityScopePropagation, StableReadSecurityScopePropagationInput,
};
use forge_store_security::{
    admit_store_security_scope, StoreCustodyPosture, StoreKeyVersionPosture,
    StoreLegacySecurityPosture, StoreSecurityMetadata, StoreSecurityScopeAdmissionRequest,
};

#[test]
fn logical_decode_rejects_matching_guard_with_mismatched_carrier_basis_before_bytes() {
    use crate::execution_support::bounded_copy_for_reference;
    use crate::plan_admission::{admit_plan, protected_set};
    use crate::support::{current_generation_page_reference, current_root_from_authority};
    use forge_store_physical_isolation::{
        PhysicalByteGuard, PhysicalReadExecutionDenial, StablePhysicalReadExecution,
    };

    let authority = crate::support::physical_authority_from_complete_closeout();
    let root = current_root_from_authority(&authority);
    let reference = current_generation_page_reference(120);
    let plan = admit_plan(&authority, root, protected_set([reference], 4), 8, 4);
    let handle = plan.into_execution_ready_handle();
    let scope = PhysicalByteGuardScope::for_owned_read_buffer(reference);
    let decode_entry = logical_decode_entry_for_handle_with_carrier_generation(
        &handle,
        scope,
        1,
        "carrier-basis-mismatch",
    );
    let mut execution = StablePhysicalReadExecution::from_execution_ready_handle(handle);
    let guard_admission = execution.admit_byte_guard(scope).unwrap();
    let guard = PhysicalByteGuard::from_bounded_copy(
        guard_admission,
        bounded_copy_for_reference(reference, b"copy"),
    )
    .unwrap();

    let denial = execution
        .read_guarded_bytes_with_security_scope(&guard, decode_entry)
        .unwrap_err();

    assert!(matches!(
        denial,
        PhysicalReadExecutionDenial::LogicalDecodeScopeCarrierMismatch { .. }
    ));
}

pub fn logical_decode_entry_for_handle(
    handle: &StablePhysicalReadHandle,
    guard_scope: PhysicalByteGuardScope,
    identity: &str,
) -> LogicalDecodeSecurityScopeEntry {
    logical_decode_entry_for_handle_with_carrier_generation(
        handle,
        guard_scope,
        guard_scope.reference().generation().get(),
        identity,
    )
}

pub fn logical_decode_entry_for_handle_with_carrier_generation(
    handle: &StablePhysicalReadHandle,
    guard_scope: PhysicalByteGuardScope,
    carrier_generation: u64,
    identity: &str,
) -> LogicalDecodeSecurityScopeEntry {
    let metadata = platform_page_metadata(identity);
    let page = PhysicalSecurityMetadataEnvelope::page_header(
        decoded_page_header(carrier_generation),
        metadata,
    );
    let manifest = PhysicalSecurityMetadataEnvelope::segment_page_manifest_entry(
        segment_page_entry(carrier_generation),
        metadata,
    );
    let input = StableReadSecurityScopePropagationInput::new(handle, guard_scope, &page, &manifest);
    let propagation = match StableReadSecurityScopePropagation::protect(input) {
        TransitionOutcome::Success(propagation) => propagation,
        other => panic!("stable-read security scope should propagate: {other:?}"),
    };
    let observed = match propagation.observe_after_root_check(handle.plan().root()) {
        TransitionOutcome::Success(observed) => observed,
        other => panic!("stable-read security scope should observe root: {other:?}"),
    };
    observed.logical_decode_entry_scope()
}

fn platform_page_metadata(identity: &str) -> StoreSecurityMetadata {
    let authority = current_authority(identity, "stable-read-execution");
    let admitted = match admit_store_security_scope(
        StoreSecurityScopeAdmissionRequest::platform_page_envelope(
            &authority,
            StoreKeyVersionPosture::Current,
            StoreCustodyPosture::InternalStoreCustody,
        ),
    ) {
        TransitionOutcome::Success(admitted) => admitted,
        other => panic!("platform page scope should admit: {other:?}"),
    };
    StoreSecurityMetadata::from_current_security_scope(
        admitted.witnesses(),
        StoreKeyVersionPosture::Current,
        StoreLegacySecurityPosture::NativeScoped,
    )
}

fn decoded_page_header(generation_value: u64) -> PhysicalPageHeader {
    let cell = PhysicalGenerationAuthority::s1()
        .page_cell(segment(1), page(2))
        .with_page_generation(generation(generation_value));
    let report =
        PhysicalHeaderAuthority::s1(PhysicalBinaryEncodingWitness::s1_canonical().unwrap())
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

fn segment_page_entry(generation_value: u64) -> SegmentPageManifestEntry {
    let cell = PhysicalGenerationAuthority::s1()
        .slot_cell(segment(1), page(2), slot(3))
        .with_slot_generation(generation(generation_value));
    SegmentPageManifestEntry::new(cell)
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
) -> forge_foundational::ContractValidatedAspectArtifact {
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

fn slot(value: u16) -> PhysicalRecordSlot {
    PhysicalRecordSlot::from_raw(value).unwrap()
}

fn generation(value: u64) -> PhysicalGeneration {
    PhysicalGeneration::from_raw(value).unwrap()
}
