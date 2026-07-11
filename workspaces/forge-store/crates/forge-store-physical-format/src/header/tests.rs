use crate::{
    PhysicalBinaryEncodingWitness, PhysicalFrameKind, PhysicalGeneration,
    PhysicalGenerationAuthority, PhysicalHeaderAuthority, PhysicalHeaderDecodeDenialKind,
    PhysicalHeaderReservedField, PhysicalPageId, PhysicalPageKind, PhysicalPublicationState,
    PhysicalRecordSlot, PhysicalReferenceAuthority, PhysicalSegmentId, PHYSICAL_HEADER_LENGTH,
};

#[test]
fn identical_frame_headers_decode_to_replay_comparable_witnesses() {
    let authority = header_authority();
    let reference = validated_slot_reference(7);
    let bytes = header_bytes(
        PhysicalFrameKind::RecordFrame.tag(),
        7,
        PhysicalPublicationState::Published,
        b"payload",
        0,
        0,
    );

    let first = authority
        .decode_frame_header(reference, &bytes, PhysicalFrameKind::RecordFrame)
        .unwrap();
    let second = authority
        .decode_frame_header(reference, &bytes, PhysicalFrameKind::RecordFrame)
        .unwrap();

    assert_eq!(first.witness().kind(), second.witness().kind());
    assert_eq!(first.witness().owner(), second.witness().owner());
    assert_eq!(first.witness().payload_length(), 7);
    assert_eq!(first.counters().frame_header_decode_count(), 1);
}

#[test]
fn payload_view_requires_admitted_header_witness() {
    let authority = header_authority();
    let reference = validated_slot_reference(3);
    let bytes = header_bytes(
        PhysicalFrameKind::RecordFrame.tag(),
        3,
        PhysicalPublicationState::Published,
        b"abc",
        0,
        0,
    );
    let report = authority
        .decode_frame_header(reference, &bytes, PhysicalFrameKind::RecordFrame)
        .unwrap();

    let payload = authority.payload_view(&bytes, report.witness()).unwrap();

    assert_eq!(payload.view().as_bytes(), b"abc");
    assert_eq!(
        payload
            .view()
            .witness()
            .counters()
            .logical_decode_after_invalid_header_count(),
        0
    );
}

#[test]
fn payload_view_rejects_witness_reused_against_unadmitted_header_bytes() {
    let authority = header_authority();
    let reference = validated_slot_reference(3);
    let admitted = header_bytes(
        PhysicalFrameKind::RecordFrame.tag(),
        3,
        PhysicalPublicationState::Published,
        b"abc",
        0,
        0,
    );
    let unadmitted = header_bytes(0xEE, 3, PhysicalPublicationState::Published, b"abc", 0, 0);
    let report = authority
        .decode_frame_header(reference, &admitted, PhysicalFrameKind::RecordFrame)
        .unwrap();

    let denial = authority
        .payload_view(&unadmitted, report.witness())
        .unwrap_err();

    assert_eq!(
        denial.kind(),
        PhysicalHeaderDecodeDenialKind::UnknownHeaderKind
    );
    assert_eq!(
        denial
            .counters()
            .logical_decode_after_invalid_header_count(),
        0
    );
}

#[test]
fn payload_view_revalidates_every_witness_header_fact_before_exposure() {
    let cases = [
        (
            WitnessReuseMutation::UnsupportedVersion,
            PhysicalHeaderDecodeDenialKind::UnsupportedVersion,
        ),
        (
            WitnessReuseMutation::GenerationDrift,
            PhysicalHeaderDecodeDenialKind::InvalidGeneration,
        ),
        (
            WitnessReuseMutation::ReservedChecksumUse,
            PhysicalHeaderDecodeDenialKind::ReservedFieldMisuse,
        ),
        (
            WitnessReuseMutation::PayloadLengthDrift,
            PhysicalHeaderDecodeDenialKind::PayloadLengthMismatch,
        ),
    ];

    for (mutation, expected_denial) in cases {
        let authority = header_authority();
        let admitted = header_bytes(
            PhysicalFrameKind::RecordFrame.tag(),
            3,
            PhysicalPublicationState::Published,
            b"abc",
            0,
            0,
        );
        let mut unadmitted = admitted.clone();
        mutation.apply(&mut unadmitted);
        let report = authority
            .decode_frame_header(
                validated_slot_reference(3),
                &admitted,
                PhysicalFrameKind::RecordFrame,
            )
            .unwrap();

        let denial = authority
            .payload_view(&unadmitted, report.witness())
            .unwrap_err();

        assert_eq!(denial.kind(), expected_denial);
        assert_eq!(
            denial
                .counters()
                .logical_decode_after_invalid_header_count(),
            0
        );
    }
}

#[test]
fn unknown_kind_denies_before_payload_view() {
    let authority = header_authority();
    let bytes = header_bytes(
        0xEE,
        9,
        PhysicalPublicationState::Published,
        b"payload",
        0,
        0,
    );

    let denial = authority
        .decode_frame_header(
            validated_slot_reference(9),
            &bytes,
            PhysicalFrameKind::RecordFrame,
        )
        .unwrap_err();

    assert_eq!(
        denial.kind(),
        PhysicalHeaderDecodeDenialKind::UnknownHeaderKind
    );
    assert_eq!(denial.counters().unknown_kind_denial_count(), 1);
    assert_eq!(
        denial
            .counters()
            .logical_decode_after_invalid_header_count(),
        0
    );
}

#[test]
fn unsupported_version_denies_before_payload_view() {
    let authority = header_authority();
    let mut bytes = header_bytes(
        PhysicalFrameKind::RecordFrame.tag(),
        9,
        PhysicalPublicationState::Published,
        b"payload",
        0,
        0,
    );
    bytes[1..3].copy_from_slice(&2u16.to_le_bytes());

    let denial = authority
        .decode_frame_header(
            validated_slot_reference(9),
            &bytes,
            PhysicalFrameKind::RecordFrame,
        )
        .unwrap_err();

    assert_eq!(
        denial.kind(),
        PhysicalHeaderDecodeDenialKind::UnsupportedVersion
    );
    assert_eq!(denial.counters().unsupported_version_denial_count(), 1);
}

#[test]
fn length_mismatch_denies_before_payload_view() {
    let authority = header_authority();
    let mut bytes = header_bytes(
        PhysicalFrameKind::RecordFrame.tag(),
        9,
        PhysicalPublicationState::Published,
        b"payload",
        0,
        0,
    );
    bytes.truncate(PHYSICAL_HEADER_LENGTH as usize + 2);

    let denial = authority
        .decode_frame_header(
            validated_slot_reference(9),
            &bytes,
            PhysicalFrameKind::RecordFrame,
        )
        .unwrap_err();

    assert_eq!(
        denial.kind(),
        PhysicalHeaderDecodeDenialKind::PayloadLengthMismatch
    );
    assert_eq!(denial.counters().length_mismatch_denial_count(), 1);
}

#[test]
fn reserved_checksum_or_lsn_use_denies_without_integrity_claim() {
    let authority = header_authority();
    let bytes = header_bytes(
        PhysicalFrameKind::RecordFrame.tag(),
        9,
        PhysicalPublicationState::Published,
        b"payload",
        11,
        0,
    );

    let denial = authority
        .decode_frame_header(
            validated_slot_reference(9),
            &bytes,
            PhysicalFrameKind::RecordFrame,
        )
        .unwrap_err();

    assert_eq!(
        denial.kind(),
        PhysicalHeaderDecodeDenialKind::ReservedFieldMisuse
    );
    assert_eq!(
        denial.reserved_field(),
        Some(PhysicalHeaderReservedField::ChecksumSlot)
    );
    assert_eq!(denial.counters().reserved_field_denial_count(), 1);
}

#[test]
fn page_header_uses_page_generation_owner() {
    let authority = header_authority();
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let cell = generations
        .page_cell(segment(1), page(2))
        .with_page_generation(generation(4));
    let bytes = header_bytes(
        PhysicalPageKind::DataPage.tag(),
        4,
        PhysicalPublicationState::Published,
        b"page",
        0,
        0,
    );

    let report = authority
        .decode_page_header(cell, &bytes, PhysicalPageKind::DataPage)
        .unwrap();

    assert_eq!(
        report.witness().kind().tag(),
        PhysicalPageKind::DataPage.tag()
    );
    assert_eq!(report.counters().page_header_decode_count(), 1);
}

fn header_authority() -> PhysicalHeaderAuthority {
    PhysicalHeaderAuthority::for_canonical_physical_format(PhysicalBinaryEncodingWitness::physical_format_canonical().unwrap())
}

fn validated_slot_reference(generation_value: u64) -> crate::PhysicalReferenceValidationWitness {
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let cell = generations
        .slot_cell(segment(1), page(2), slot(3))
        .with_slot_generation(generation(generation_value));
    let admitted = references.admit_page_slot(cell);
    references.validate_page_slot(admitted, cell).unwrap()
}

fn header_bytes(
    kind_tag: u8,
    generation_value: u64,
    publication: PhysicalPublicationState,
    payload: &[u8],
    checksum_slot: u32,
    recovery_lsn: u64,
) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(PHYSICAL_HEADER_LENGTH as usize + payload.len());
    bytes.push(kind_tag);
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&PHYSICAL_HEADER_LENGTH.to_le_bytes());
    bytes.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&generation_value.to_le_bytes());
    bytes.push(publication.code());
    bytes.extend_from_slice(&checksum_slot.to_le_bytes());
    bytes.extend_from_slice(&recovery_lsn.to_le_bytes());
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

#[derive(Clone, Copy)]
enum WitnessReuseMutation {
    UnsupportedVersion,
    GenerationDrift,
    ReservedChecksumUse,
    PayloadLengthDrift,
}

impl WitnessReuseMutation {
    fn apply(self, bytes: &mut [u8]) {
        match self {
            Self::UnsupportedVersion => bytes[1..3].copy_from_slice(&2u16.to_le_bytes()),
            Self::GenerationDrift => bytes[9..17].copy_from_slice(&4u64.to_le_bytes()),
            Self::ReservedChecksumUse => bytes[18..22].copy_from_slice(&5u32.to_le_bytes()),
            Self::PayloadLengthDrift => bytes[5..9].copy_from_slice(&4u32.to_le_bytes()),
        }
    }
}
