use super::*;

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

#[test]
fn same_generation_different_owner_is_rejected_before_payload_exposure() {
    let authority = header_authority();
    let bytes = header_bytes(
        PhysicalFrameKind::RecordFrame.tag(),
        7,
        PhysicalPublicationState::Published,
        b"payload",
        0,
        0,
    );
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let other = generations
        .slot_cell(segment(1), page(99), slot(3))
        .with_slot_generation(generation(7));
    let validated = references
        .validate_page_slot(references.admit_page_slot(other), other)
        .expect("reference validation");
    let denial = authority
        .decode_frame_header(validated, &bytes, PhysicalFrameKind::RecordFrame)
        .expect_err("encoded owner coordinates must dominate equal generation");
    assert_eq!(
        denial.kind(),
        PhysicalHeaderDecodeDenialKind::OwnerCoordinateMismatch
    );
}
