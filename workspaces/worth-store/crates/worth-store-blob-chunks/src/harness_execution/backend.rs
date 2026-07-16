use crate::{AdmittedBlobCustody, BlobCustodyPurpose};
use worth_foundational::{aspects, AspectContract, AspectValue, InternedString, ScalarAspectType};
use worth_store_aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
    StorePhysicalBoundaryWitness,
};
use worth_store_authority::{require_current_store_authority, StoreCurrentAuthorityWitness};
use worth_store_physical_backend::{
    AdmittedBackendCapabilityWitness, BackendCapabilityAdmissionRequest,
    BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet, BackendMediaAssumptionSet,
    BackendRebindTriggers, BackendTargetProfile, PhysicalBackendCapabilityAdmissionAuthority,
};
use worth_store_physical_format::{
    PageGenerationCell, PhysicalBinaryEncodingWitness, PhysicalChunkChecksumAuthority,
    PhysicalChunkPayloadIntegrityWitness, PhysicalGeneration, PhysicalGenerationAuthority,
    PhysicalHeaderAuthority, PhysicalPageId, PhysicalPageKind, PhysicalPageRecordAuthority,
    PhysicalRecordSlot, PhysicalReferenceAuthority, PhysicalSegmentId, SlotAppendRequest,
    StorePhysicalChunkWriteReceipt,
};

pub(super) fn export_readiness(case: &str) -> AdmittedBlobCustody {
    crate::test_support::admitted_blob_custody(case, BlobCustodyPurpose::Export)
}

pub(super) fn admitted_backend() -> AdmittedBackendCapabilityWitness {
    PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(BackendCapabilityAdmissionRequest::new(
            BackendTargetProfile::PosixFileFsyncDirSync,
            BackendCapabilityEvidenceBasis::certified_backend_profile(),
            BackendCapabilitySupportSet::all_supported(),
            BackendMediaAssumptionSet::platform_file_defaults()
                .with_direct_io_alignment()
                .with_sector_atomicity()
                .with_page_cache_policy()
                .with_mmap_coherence()
                .with_async_ordering()
                .with_secure_frame_io()
                .with_flush_ordering()
                .with_fdatasync_durability()
                .with_cold_tier_io_posture(),
            BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
        ))
        .expect("backend")
}

pub(super) fn physical_payload_for_bytes(bytes: &[u8]) -> PhysicalChunkPayloadIntegrityWitness {
    PhysicalChunkChecksumAuthority::canonical_blob_checksum()
        .admit_store_payload(record_receipt(bytes))
        .expect("payload")
}

pub(super) fn current_authority(identity_key: &str, value: &str) -> StoreCurrentAuthorityWitness {
    let key = aspects()
        .vocabulary()
        .key(identity_key)
        .expect("aspect key");
    let contract: AspectContract = aspects()
        .contract()
        .for_key(key.clone())
        .identified_by(aspects().vocabulary().identity(1))
        .at_revision(aspects().vocabulary().revision(1))
        .scalar(ScalarAspectType::String);
    let value = expect_success(
        aspects()
            .validate()
            .against(&contract)
            .value(AspectValue::String(InternedString::from(value))),
        "aspect value",
    );
    let admitted_state = expect_success(
        aspects().authoritative_state().admit([value]),
        "aspect state",
    );
    let physical = StorePhysicalBoundaryWitness::from_physical_authority(
        worth_store_contracts::StorePhysicalAuthorityWitness::for_aspect_native_boundary(
            worth_store_contracts::ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
        )
        .expect("physical authority"),
    )
    .expect("physical boundary");
    require_current_store_authority(
        StoreAspectBoundaryFact::from_admitted_state(
            StoreAspectIdentity::from_aspect_key(key),
            StoreAspectAuthorityInput::new(admitted_state, physical),
        )
        .expect("boundary fact"),
    )
}

pub(super) fn stable_digest(raw: &str) -> worth_store_contracts::StableDigest {
    worth_store_contracts::StableDigest::new(raw).expect("stable digest")
}

fn expect_success<S, D, De, St, R, F>(
    outcome: worth_proof::TransitionOutcome<S, D, De, St, R, F>,
    message: &str,
) -> S
where
    S: core::fmt::Debug,
    D: core::fmt::Debug,
    De: core::fmt::Debug,
    St: core::fmt::Debug,
    R: core::fmt::Debug,
    F: core::fmt::Debug,
{
    match outcome {
        worth_proof::TransitionOutcome::Success(value) => value,
        other => panic!("{message}: {other:?}"),
    }
}

fn record_receipt(bytes: &[u8]) -> StorePhysicalChunkWriteReceipt {
    let records = PhysicalPageRecordAuthority::for_canonical_physical_format(
        PhysicalHeaderAuthority::for_canonical_physical_format(
            PhysicalBinaryEncodingWitness::physical_format_canonical().expect("binary encoding"),
        ),
    );
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let page_cell = generations
        .page_cell(segment(7), page(11))
        .with_page_generation(generation(5));
    let slot_cell = generations
        .slot_cell(segment(7), page(11), slot(1))
        .with_slot_generation(generation(9));
    let append = records
        .append_record(
            admitted_page(&records, page_cell, &page_bytes(page_cell, &[])),
            SlotAppendRequest::ordinary(slot_cell, bytes),
        )
        .expect("append");
    let validation = references
        .validate_page_slot(append.reference_admission(), slot_cell)
        .expect("validation");
    let reopened_page = page_bytes(page_cell, append.page_payload());
    let located = records
        .locate_record(
            admitted_page(&records, page_cell, &reopened_page),
            validation,
        )
        .expect("locate");
    StorePhysicalChunkWriteReceipt::from_page_record_view(located.record_view()).expect("receipt")
}

fn admitted_page<'a>(
    records: &PhysicalPageRecordAuthority,
    page_cell: worth_store_physical_format::PageGenerationCell,
    bytes: &'a [u8],
) -> worth_store_physical_format::RecordPagePayload<'a> {
    let header = records
        .decode_record_page_header(page_cell, bytes, PhysicalPageKind::DataPage)
        .expect("header");
    records
        .admit_record_page_payload(bytes, header.witness())
        .expect("payload")
}

fn page_bytes(cell: PageGenerationCell, payload: &[u8]) -> Vec<u8> {
    let binary = PhysicalBinaryEncodingWitness::physical_format_canonical().expect("encoding");
    let headers = PhysicalHeaderAuthority::for_canonical_physical_format(binary);
    let mut bytes = Vec::with_capacity(
        usize::from(worth_store_physical_format::PHYSICAL_HEADER_LENGTH) + payload.len(),
    );
    bytes.extend_from_slice(&headers.encode_page_header(
        cell,
        PhysicalPageKind::DataPage,
        payload.len().try_into().expect("bounded test payload"),
    ));
    bytes.extend_from_slice(payload);
    bytes
}

fn segment(value: u64) -> PhysicalSegmentId {
    PhysicalSegmentId::from_raw(value).expect("segment")
}
fn page(value: u64) -> PhysicalPageId {
    PhysicalPageId::from_raw(value).expect("page")
}
fn slot(value: u16) -> PhysicalRecordSlot {
    PhysicalRecordSlot::from_raw(value).expect("slot")
}
fn generation(value: u64) -> PhysicalGeneration {
    PhysicalGeneration::from_raw(value).expect("generation")
}
