use crate::courtroom::harness::test_support::physical_container_integrity_test_support::{
    frame_start, inspect_frame_with_witness_payload, inspect_page_denial, page_payload_with_record,
};
use crate::courtroom::harness::test_support::physical_scope_admission_test_support::validation;
use crate::courtroom::harness::test_support::pre_decode_physical_admission_test_support::{
    checksum_declaration, crc32c, frame_witness, with_entry_seed,
};
use forge_store_physical_format::PhysicalFrameKind;
use forge_store_physical_integrity::{
    DeclaredPhysicalChecksum, PhysicalBoundaryLocalization, PhysicalContainerIntegrityDenialKind,
    PhysicalIntegrityAdmissionRequest, PreDecodePhysicalDenialKind,
};

#[test]
fn page_local_frame_header_field_damage_denies_before_record_view() {
    assert_nested_frame_header_damage(|bytes, start| {
        bytes[start + 1..start + 3].copy_from_slice(&2u16.to_le_bytes());
    });
    assert_nested_frame_header_damage(|bytes, start| {
        bytes[start + 9] = bytes[start + 9].wrapping_add(1);
    });
    assert_nested_frame_header_damage(|bytes, start| {
        bytes[start + 17] = 0xFF;
    });
    assert_nested_frame_header_damage(|bytes, start| {
        bytes[start + 18] = 1;
    });
}

#[test]
fn top_level_overlong_frame_body_denies_as_malformed_frame() {
    let denial = inspect_frame_with_witness_payload(b"frame-body-extra", b"frame-body");

    assert_eq!(
        denial.kind(),
        PhysicalContainerIntegrityDenialKind::MalformedFrame
    );
    assert_eq!(
        denial.localization(),
        PhysicalBoundaryLocalization::FrameBody
    );
    assert!(denial.torn_frame().is_some());
}

#[test]
fn top_level_truncated_frame_body_denies_before_container_inspection() {
    let protected_payload = b"frame-body";
    let witness_payload = b"frame-body-extra";
    let validation = validation(1, 2, 3, 7);
    let mut denial = None;

    with_entry_seed(protected_payload, |seed| {
        let declaration = checksum_declaration().admit_for_physical_integrity_entry(seed.entry_witness());
        let admission = seed.with_checksum_declaration(declaration).unwrap();
        denial = Some(
            admission
                .admit_frame(PhysicalIntegrityAdmissionRequest::frame(
                    validation,
                    frame_witness(witness_payload),
                    PhysicalFrameKind::RecordFrame,
                    DeclaredPhysicalChecksum::new(crc32c(protected_payload)),
                ))
                .unwrap_err(),
        );
    });

    let denial = denial.unwrap();
    assert_eq!(
        denial.kind(),
        PreDecodePhysicalDenialKind::TruncatedPhysicalFrame
    );
    assert_eq!(
        denial.protected_byte_count(),
        protected_payload.len() as u64
    );
    assert_eq!(denial.counters().checksum_execution_count(), 0);
    assert_eq!(
        denial.counters().skipped_logical_decode().skipped_count(),
        1
    );
}

fn assert_nested_frame_header_damage(corrupt: impl FnOnce(&mut Vec<u8>, usize)) {
    let mut page_payload = page_payload_with_record(b"nested-header-damage");
    let start = frame_start(&page_payload);
    corrupt(&mut page_payload, start);

    let denial = inspect_page_denial(&page_payload);
    assert_eq!(
        denial.kind(),
        PhysicalContainerIntegrityDenialKind::HeaderWitnessMismatch
    );
    assert_eq!(
        denial.localization(),
        PhysicalBoundaryLocalization::FrameHeader
    );
    assert_eq!(denial.counters().skipped_record_view_constructions(), 1);
}
