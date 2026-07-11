#[path = "s4_closeout/fixture.rs"]
mod closeout_fixture;
#[path = "s5_stable_read_execution/plan_admission.rs"]
mod plan_admission;
#[path = "s5_epoch_scope_and_root_kind/support.rs"]
#[allow(dead_code)]
mod s5_support;

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
    PhysicalByteGuardScope, StableReadSecurityScopePropagation,
    StableReadSecurityScopePropagationInput,
};
use forge_store_security::{
    admit_store_security_scope, StoreAuthenticityRequirement, StoreAuthenticityRequirementClass,
    StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture, StoreLegacySecurityPosture,
    StoreSecurityMetadata, StoreSecurityScopeAdmissionExpectation,
    StoreSecurityScopeAdmissionRequest, StoreSecurityScopePropagationDenialKind, StoreTenantScope,
};

#[test]
fn stable_read_scope_survives_protection_observation_and_decode_entry() {
    let handle = stable_read_handle();
    let metadata = platform_page_metadata("stable-read-preserves-scope");
    let input = stable_read_scope_input(&handle, metadata, metadata);

    let propagation = match StableReadSecurityScopePropagation::protect(input) {
        TransitionOutcome::Success(propagation) => propagation,
        other => panic!("stable read scope should propagate: {other:?}"),
    };
    let observed = match propagation.observe_after_root_check(handle.plan().root()) {
        TransitionOutcome::Success(observed) => observed,
        other => panic!("root observation should preserve propagated scope: {other:?}"),
    };
    let decode_entry = observed.logical_decode_entry_scope();

    assert_eq!(decode_entry.metadata(), metadata);
    assert_eq!(
        decode_entry.carrier_basis().page_header_generation(),
        stable_read_reference().generation()
    );
    assert_eq!(
        decode_entry
            .carrier_basis()
            .manifest_page_slot()
            .generation(),
        stable_read_reference().generation()
    );
    assert_eq!(
        decode_entry.carrier_basis().guard_scope(),
        PhysicalByteGuardScope::for_owned_read_buffer(stable_read_reference())
    );
    assert_eq!(decode_entry.counters().store_counters().preserved(), 1);
    assert_eq!(decode_entry.counters().root_observations(), 1);
    assert_eq!(decode_entry.counters().logical_decode_entries(), 1);
}

#[test]
fn stale_propagated_scope_is_physical_security_denial_before_logical_decode() {
    let handle = stable_read_handle();
    let expected = platform_page_metadata("stale-scope-expected");
    let stale = platform_page_metadata_with_key_version(
        "stale-scope-observed",
        StoreKeyVersionPosture::Stale,
    );
    let input = stable_read_scope_input(&handle, stale, expected);

    let outcome = StableReadSecurityScopePropagation::protect(input);

    match outcome {
        TransitionOutcome::Denied(denial) => {
            assert_eq!(
                denial.store_denial().kind(),
                StoreSecurityScopePropagationDenialKind::StalePropagatedSecurityScope
            );
            assert_eq!(denial.store_denial().counters().stale(), 1);
        }
        other => panic!("stale scope must deny before logical decode entry exists: {other:?}"),
    }
}

#[test]
fn security_scope_drift_between_page_header_and_manifest_denies_before_logical_decode() {
    let handle = stable_read_handle();
    let page_metadata = platform_page_metadata_with_tenant(
        "page-tenant-scope",
        StoreTenantScope::TenantPhysicalBoundary,
    );
    let manifest_metadata = platform_page_metadata_with_tenant(
        "manifest-tenant-scope",
        StoreTenantScope::MultiTenantPhysicalBoundary,
    );
    let input = stable_read_scope_input(&handle, page_metadata, manifest_metadata);

    let outcome = StableReadSecurityScopePropagation::protect(input);

    match outcome {
        TransitionOutcome::Denied(denial) => {
            assert_eq!(
                denial.store_denial().kind(),
                StoreSecurityScopePropagationDenialKind::ScopeDriftBeforeLogicalDecode
            );
            assert_eq!(denial.store_denial().counters().drifted(), 1);
        }
        other => panic!("scope drift must deny before logical decode entry exists: {other:?}"),
    }
}

fn stable_read_handle() -> forge_store_physical_isolation::StablePhysicalReadHandle {
    let authority = s5_support::physical_authority_from_complete_closeout();
    let root = s5_support::current_root_from_authority(&authority);
    let reference = stable_read_reference();
    plan_admission::admit_plan(
        &authority,
        root,
        plan_admission::protected_set([reference], 4),
        8,
        4,
    )
    .into_execution_ready_handle()
}

fn platform_page_metadata(identity: &str) -> StoreSecurityMetadata {
    platform_page_metadata_with_key_version(identity, StoreKeyVersionPosture::Current)
}

fn stable_read_scope_input(
    handle: &forge_store_physical_isolation::StablePhysicalReadHandle,
    page_metadata: StoreSecurityMetadata,
    manifest_metadata: StoreSecurityMetadata,
) -> StableReadSecurityScopePropagationInput {
    let generation = stable_read_reference().generation().get();
    let page = PhysicalSecurityMetadataEnvelope::page_header(
        decoded_page_header(generation),
        page_metadata,
    );
    let manifest = PhysicalSecurityMetadataEnvelope::segment_page_manifest_entry(
        segment_page_entry(generation),
        manifest_metadata,
    );
    StableReadSecurityScopePropagationInput::new(
        handle,
        PhysicalByteGuardScope::for_owned_read_buffer(stable_read_reference()),
        &page,
        &manifest,
    )
}

fn stable_read_reference() -> forge_store_physical_isolation::CurrentGenerationPhysicalReference {
    s5_support::current_generation_page_reference(801)
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

fn platform_page_metadata_with_key_version(
    identity: &str,
    key_version: StoreKeyVersionPosture,
) -> StoreSecurityMetadata {
    let authority = current_authority(identity, "platform-page");
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
        key_version,
        StoreLegacySecurityPosture::NativeScoped,
    )
}

fn platform_page_metadata_with_tenant(
    identity: &str,
    tenant_scope: StoreTenantScope,
) -> StoreSecurityMetadata {
    let authority = current_authority(identity, "platform-page");
    let admitted = match admit_store_security_scope(StoreSecurityScopeAdmissionRequest::new(
        &authority,
        StoreKeyScope::PageEnvelope,
        StoreKeyVersionPosture::Current,
        tenant_scope,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
        StoreSecurityScopeAdmissionExpectation::new(
            StoreKeyScope::PageEnvelope,
            tenant_scope,
            StoreAuthenticityRequirement::required(
                StoreAuthenticityRequirementClass::AuthenticatedFrame,
            ),
            StoreCustodyPosture::InternalStoreCustody,
        ),
    )) {
        TransitionOutcome::Success(admitted) => admitted,
        other => panic!("platform page scope should admit: {other:?}"),
    };
    StoreSecurityMetadata::from_current_security_scope(
        admitted.witnesses(),
        StoreKeyVersionPosture::Current,
        StoreLegacySecurityPosture::NativeScoped,
    )
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
