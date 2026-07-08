use crate::{
    PhysicalAlignmentClass, PhysicalBinaryEncodingWitness, PhysicalBinaryFormatError,
    PhysicalByteOrder, PhysicalByteOrderDeclaration, PhysicalFieldWidth, PhysicalFieldWidthKind,
    PhysicalFormatAuthoritySource, PhysicalFormatDeclaration, PhysicalFormatDeclarationBuilder,
    PhysicalFormatMagic, PhysicalFormatVersion, PhysicalForwardCompatibilityDeclaration,
    PhysicalForwardCompatibilityPolicy, PhysicalGoldenFormatHeaderFixture, PhysicalPageSizeClass,
    PhysicalReservedFieldPolicy, PhysicalReservedFieldPolicyDeclaration,
};

#[test]
fn canonical_format_header_replays_identically() {
    let witness = PhysicalBinaryEncodingWitness::s1_canonical().unwrap();
    let bytes = witness.encode_golden_format_header();

    assert_eq!(
        bytes.as_slice(),
        PhysicalGoldenFormatHeaderFixture::s1_canonical().bytes()
    );

    let decoded = PhysicalBinaryEncodingWitness::decode_golden_format_header(&bytes).unwrap();
    assert_eq!(decoded, witness);
}

#[test]
fn malformed_golden_header_bytes_deny_before_admission() {
    let bytes = PhysicalBinaryEncodingWitness::s1_canonical()
        .unwrap()
        .encode_golden_format_header();

    assert_eq!(
        PhysicalBinaryEncodingWitness::decode_golden_format_header(&bytes[..36]),
        Err(PhysicalBinaryFormatError::GoldenHeaderLengthMismatch {
            expected: 37,
            actual: 36,
        })
    );
    assert_mutated_byte_denial(&bytes, 10, 2, PhysicalBinaryFormatError::ByteOrderMismatch);
    assert_mutated_byte_denial(
        &bytes,
        11,
        1,
        PhysicalBinaryFormatError::UnsupportedPageSize(16_385),
    );
    assert_mutated_byte_denial(
        &bytes,
        15,
        32,
        PhysicalBinaryFormatError::FieldWidthMismatch(PhysicalFieldWidthKind::SegmentId),
    );
    assert_mutated_byte_denial(
        &bytes,
        25,
        8,
        PhysicalBinaryFormatError::AlignmentMismatch(crate::PhysicalAlignmentSite::PageStart),
    );
    assert_mutated_byte_denial(
        &bytes,
        35,
        9,
        PhysicalBinaryFormatError::UnknownReservedFieldPolicy,
    );
    assert_mutated_byte_denial(
        &bytes,
        36,
        2,
        PhysicalBinaryFormatError::ForwardPreservationNotAdmission,
    );
    assert_mutated_byte_denial(
        &bytes,
        36,
        3,
        PhysicalBinaryFormatError::ForwardMigrationNotAdmission,
    );
}

#[test]
fn serializer_layout_and_host_order_are_rejected_as_format_authority() {
    let serde = PhysicalFormatDeclaration::builder()
        .authority_source(PhysicalFormatAuthoritySource::SerdeMapOrder)
        .define();
    assert_eq!(serde, Err(PhysicalBinaryFormatError::SerdeOrderRejected));

    let layout = PhysicalFormatDeclaration::builder()
        .authority_source(PhysicalFormatAuthoritySource::RustStructLayout)
        .define();
    assert_eq!(layout, Err(PhysicalBinaryFormatError::RustLayoutRejected));

    let host_order = PhysicalFormatDeclaration::builder()
        .magic(PhysicalFormatMagic::s1_store())
        .version(PhysicalFormatVersion::s1_initial())
        .byte_order_declaration(PhysicalByteOrderDeclaration::HostEndian)
        .define();
    assert_eq!(
        host_order,
        Err(PhysicalBinaryFormatError::HostEndianRejected)
    );
}

#[test]
fn unsupported_page_and_reserved_declarations_are_rejected() {
    assert_eq!(
        PhysicalPageSizeClass::from_bytes(12_288),
        Err(PhysicalBinaryFormatError::UnsupportedPageSize(12_288))
    );

    let unknown_reserved = complete_builder()
        .reserved_field_policy_declaration(PhysicalReservedFieldPolicyDeclaration::Unknown)
        .define();
    assert_eq!(
        unknown_reserved,
        Err(PhysicalBinaryFormatError::UnknownReservedFieldPolicy)
    );

    let unsupported_forward = complete_builder()
        .forward_compatibility_declaration(PhysicalForwardCompatibilityDeclaration::Unsupported)
        .define();
    assert_eq!(
        unsupported_forward,
        Err(PhysicalBinaryFormatError::UnsupportedForwardCompatibilityPolicy)
    );

    let preserve_forward = complete_builder()
        .forward_compatibility(PhysicalForwardCompatibilityPolicy::PreserveUnknownBytes)
        .define();
    assert_eq!(
        preserve_forward,
        Err(PhysicalBinaryFormatError::ForwardPreservationNotAdmission)
    );

    let migration_forward = complete_builder()
        .forward_compatibility(PhysicalForwardCompatibilityPolicy::MigrationReserved)
        .define();
    assert_eq!(
        migration_forward,
        Err(PhysicalBinaryFormatError::ForwardMigrationNotAdmission)
    );
}

#[test]
fn incomplete_format_declarations_do_not_imply_s1_law() {
    let missing_width = PhysicalFormatDeclaration::builder()
        .magic(PhysicalFormatMagic::s1_store())
        .version(PhysicalFormatVersion::s1_initial())
        .byte_order(PhysicalByteOrder::LittleEndian)
        .page_size(PhysicalPageSizeClass::KiB16)
        .define();

    assert_eq!(
        missing_width,
        Err(PhysicalBinaryFormatError::MissingFieldWidth(
            PhysicalFieldWidthKind::SegmentId
        ))
    );
}

fn complete_builder() -> PhysicalFormatDeclarationBuilder {
    PhysicalFormatDeclaration::builder()
        .magic(PhysicalFormatMagic::s1_store())
        .version(PhysicalFormatVersion::s1_initial())
        .byte_order(PhysicalByteOrder::LittleEndian)
        .field_width(PhysicalFieldWidth::segment_id_u64())
        .field_width(PhysicalFieldWidth::page_id_u64())
        .field_width(PhysicalFieldWidth::generation_u64())
        .field_width(PhysicalFieldWidth::header_length_u16())
        .field_width(PhysicalFieldWidth::payload_length_u32())
        .page_size(PhysicalPageSizeClass::KiB16)
        .alignment(PhysicalAlignmentClass::page_start_4k())
        .alignment(PhysicalAlignmentClass::frame_start_8())
        .alignment(PhysicalAlignmentClass::slot_directory_offset_8())
        .alignment(PhysicalAlignmentClass::extent_start_4k())
        .alignment(PhysicalAlignmentClass::manifest_record_8())
        .reserved_field_policy(PhysicalReservedFieldPolicy::zeroed_and_preserved())
        .forward_compatibility(PhysicalForwardCompatibilityPolicy::reject_unknown_kind())
}

fn assert_mutated_byte_denial(
    original: &[u8],
    offset: usize,
    replacement: u8,
    expected: PhysicalBinaryFormatError,
) {
    let mut mutated = original.to_vec();
    mutated[offset] = replacement;
    assert_eq!(
        PhysicalBinaryEncodingWitness::decode_golden_format_header(&mutated),
        Err(expected)
    );
}
