use crate::{
    courtroom::harness::test_support::physical_container_integrity_test_support::{
        frame_start, inspect_extent_report, inspect_frame_with_witness_payload,
        inspect_page_denial, inspect_page_report, page_payload_with_record,
    },
    courtroom::harness::test_support::physical_scope_admission_test_support::{
        page_cell, page_request, root_with_slot, scope_membership, validation, with_checked_frame,
        with_checked_page,
    },
};
use forge_store_physical_format::PhysicalReferenceScope;
use forge_store_physical_integrity::{
    PhysicalBoundaryLocalization, PhysicalContainerIntegrity, PhysicalContainerIntegrityDenialKind,
    PhysicalScopeAdmission, ScopedPhysicalValidatorInput,
};

#[test]
fn physical_container_parity_matches_for_independent_page_and_extent_views() {
    let page_payload = page_payload_with_record(b"stable-record");
    let first_page = inspect_page_report(&page_payload);
    let second_page = inspect_page_report(&page_payload);
    assert_eq!(first_page, second_page);
    assert_eq!(first_page.slot_directory().slot_count(), 3);
    assert_eq!(first_page.slot_directory().occupied_slots(), 1);
    assert_eq!(first_page.slot_directory().free_or_reserved_slots(), 2);
    assert_eq!(first_page.counters().slot_entries_inspected(), 3);
    assert_eq!(first_page.counters().skipped_record_view_constructions(), 1);

    let first_extent = inspect_extent_report(b"stable-extent");
    let second_extent = inspect_extent_report(b"stable-extent");
    assert_eq!(first_extent, second_extent);
    assert_eq!(
        first_extent.boundary(),
        PhysicalBoundaryLocalization::ExtentBoundary
    );
    assert_eq!(first_extent.counters().extent_boundary_checks(), 1);
    assert_eq!(
        first_extent.frame().boundary(),
        PhysicalBoundaryLocalization::FrameBody
    );
}

#[test]
fn slot_directory_damage_emits_ambiguous_boundary_without_record_view() {
    let mut page_payload = page_payload_with_record(b"directory-damage");
    page_payload[0] = 9;

    let denial = inspect_page_denial(&page_payload);
    assert_eq!(
        denial.kind(),
        PhysicalContainerIntegrityDenialKind::SlotDirectoryMalformed
    );
    assert_eq!(
        denial.localization(),
        PhysicalBoundaryLocalization::AmbiguousBoundary
    );
    assert!(denial.ambiguous_boundary_damage().is_some());
    assert_eq!(denial.counters().skipped_record_view_constructions(), 1);
}

#[test]
fn page_local_frame_header_and_length_damage_localize_before_decode() {
    let mut bad_header = page_payload_with_record(b"header-damage");
    let start = frame_start(&bad_header);
    bad_header[start] = 0xFF;
    let denial = inspect_page_denial(&bad_header);
    assert_eq!(
        denial.kind(),
        PhysicalContainerIntegrityDenialKind::HeaderWitnessMismatch
    );
    assert_eq!(
        denial.localization(),
        PhysicalBoundaryLocalization::FrameHeader
    );

    let mut bad_header_length = page_payload_with_record(b"header-length-damage");
    let start = frame_start(&bad_header_length);
    bad_header_length[start + 3..start + 5].copy_from_slice(&0u16.to_le_bytes());
    let denial = inspect_page_denial(&bad_header_length);
    assert_eq!(
        denial.kind(),
        PhysicalContainerIntegrityDenialKind::HeaderWitnessMismatch
    );
    assert_eq!(
        denial.localization(),
        PhysicalBoundaryLocalization::LengthField
    );

    let mut bad_length = page_payload_with_record(b"length-damage");
    let start = frame_start(&bad_length);
    bad_length[start + 5] = bad_length[start + 5].wrapping_add(4);
    let denial = inspect_page_denial(&bad_length);
    assert_eq!(
        denial.kind(),
        PhysicalContainerIntegrityDenialKind::TornFrame
    );
    assert_eq!(
        denial.localization(),
        PhysicalBoundaryLocalization::FrameBody
    );
    assert!(denial.torn_frame().is_some());
}

#[test]
fn unknown_slot_state_emits_ambiguous_boundary_without_false_slot_precision() {
    let mut page_payload = page_payload_with_record(b"unknown-slot-state");
    page_payload[occupied_slot_entry_offset()] = 0xFF;

    let denial = inspect_page_denial(&page_payload);
    assert_eq!(
        denial.kind(),
        PhysicalContainerIntegrityDenialKind::SlotDirectoryMalformed
    );
    assert_eq!(
        denial.localization(),
        PhysicalBoundaryLocalization::AmbiguousBoundary
    );
    assert!(denial.ambiguous_boundary_damage().is_some());
    assert_eq!(denial.counters().skipped_record_view_constructions(), 1);
}

#[test]
fn torn_and_overlong_page_local_frames_deny_before_record_view() {
    let mut torn = page_payload_with_record(b"torn-frame");
    let torn_length = first_slot_frame_length(&torn) - 1;
    rewrite_first_slot_frame_length(&mut torn, torn_length);
    let denial = inspect_page_denial(&torn);
    assert_eq!(
        denial.kind(),
        PhysicalContainerIntegrityDenialKind::TornFrame
    );
    assert_eq!(
        denial.localization(),
        PhysicalBoundaryLocalization::FrameBody
    );
    assert_eq!(denial.counters().skipped_record_view_constructions(), 1);

    let mut overlong = page_payload_with_record(b"overlong-frame");
    overlong.push(0xAB);
    let overlong_length = first_slot_frame_length(&overlong) + 1;
    rewrite_first_slot_frame_length(&mut overlong, overlong_length);
    let denial = inspect_page_denial(&overlong);
    assert_eq!(
        denial.kind(),
        PhysicalContainerIntegrityDenialKind::MalformedFrame
    );
    assert_eq!(
        denial.localization(),
        PhysicalBoundaryLocalization::FrameBody
    );
}

#[test]
fn container_facade_rejects_scope_admitted_wrong_family_inputs() {
    let cell = page_cell(1, 2, 7);
    with_checked_page(b"manifest-page-family", cell, |checked| {
        let scope = PhysicalReferenceScope::manifest_page(cell);
        let root = root_with_slot(1, 2, 3, 7);
        let membership = scope_membership(&root, scope);
        let request = page_request(&checked, scope, membership);
        let admission = PhysicalScopeAdmission::admit_page(checked, request).unwrap();

        let page_input = ScopedPhysicalValidatorInput::manifest(admission.clone()).unwrap();
        let denial = PhysicalContainerIntegrity::inspect_page(page_input).unwrap_err();
        assert_eq!(
            denial.kind(),
            PhysicalContainerIntegrityDenialKind::WrongPhysicalFamily
        );

        let slot_input = ScopedPhysicalValidatorInput::manifest(admission).unwrap();
        let denial = PhysicalContainerIntegrity::inspect_slot_directory(slot_input).unwrap_err();
        assert_eq!(
            denial.kind(),
            PhysicalContainerIntegrityDenialKind::WrongPhysicalFamily
        );
    });

    let validation = validation(1, 2, 3, 7);
    with_checked_frame(b"wal-frame-family", validation, |checked| {
        let scope = PhysicalReferenceScope::wal_frame(validation);
        let root = root_with_slot(1, 2, 3, 7);
        let membership = scope_membership(&root, scope);
        let request = crate::courtroom::harness::test_support::physical_scope_admission_test_support::frame_request(
            &checked, scope, membership,
        );
        let admission = PhysicalScopeAdmission::admit_frame(checked, request).unwrap();
        let input = ScopedPhysicalValidatorInput::wal_frame(admission).unwrap();

        let denial = PhysicalContainerIntegrity::inspect_frame(input).unwrap_err();
        assert_eq!(
            denial.kind(),
            PhysicalContainerIntegrityDenialKind::WrongPhysicalFamily
        );
    });
}

#[test]
fn top_level_frame_length_mismatch_denies_as_torn_or_malformed() {
    let malformed = inspect_frame_with_witness_payload(b"frame-body-extra", b"frame-body");
    assert_eq!(
        malformed.kind(),
        PhysicalContainerIntegrityDenialKind::MalformedFrame
    );
    assert_eq!(
        malformed.localization(),
        PhysicalBoundaryLocalization::FrameBody
    );
}

fn first_slot_frame_length(page_payload: &[u8]) -> u32 {
    let offset = occupied_slot_entry_offset();
    u32::from_le_bytes([
        page_payload[offset + 8],
        page_payload[offset + 9],
        page_payload[offset + 10],
        page_payload[offset + 11],
    ])
}

fn rewrite_first_slot_frame_length(page_payload: &mut [u8], length: u32) {
    let offset = occupied_slot_entry_offset();
    page_payload[offset + 8..offset + 12].copy_from_slice(&length.to_le_bytes());
}

fn occupied_slot_entry_offset() -> usize {
    4 + ((3 - 1) * 24)
}
