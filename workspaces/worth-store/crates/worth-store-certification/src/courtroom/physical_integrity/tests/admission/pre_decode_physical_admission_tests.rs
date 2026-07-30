use crate::courtroom::harness::test_support::pre_decode_physical_admission_test_support::{
    admit_checked_frame, assert_localized_pre_decode_denial,
    assert_localized_pre_decode_denial_counters, assert_pre_decode_denial_counters, crc32c,
    current_frame_bytes, current_page_bytes, current_page_cell, deny_checked_frame, frame_witness,
    page_witness, stale_validation, with_entry_seed, with_pre_decode_admission,
    CountingSemanticDecoder,
};
use worth_store_physical_format::{PhysicalFrameKind, PhysicalPageKind};
use worth_store_physical_integrity::{
    ChecksumAlgorithmClaim, DeclaredPhysicalChecksum, LogicalDecoder, PhysicalIntegrityAdmission,
    PhysicalIntegrityAdmissionRequest, PreDecodePhysicalDenialKind,
};

#[test]
fn intact_physical_bytes_replay_to_same_checked_form_and_gate_identity() {
    let first = admit_checked_frame(b"stable-physical-payload", b"stable-physical-payload");
    let second = admit_checked_frame(b"stable-physical-payload", b"stable-physical-payload");

    assert_eq!(first.identity, second.identity);
    assert_eq!(first.checked_bytes, second.checked_bytes);
    assert_eq!(
        first.checked_byte_count,
        current_frame_bytes(b"stable-physical-payload").len() as u64
    );
    assert_eq!(second.checked_byte_count, first.checked_byte_count);
    assert_eq!(first.checksum_executions, 1);
    assert_eq!(second.checksum_executions, 1);
    assert_eq!(first.skipped_decodes, 0);
    assert_eq!(second.skipped_decodes, 0);
}

#[test]
fn damaged_and_mismatched_bytes_skip_logical_decode_with_exact_counters() {
    let protected_page = current_page_bytes(b"page-poisoned");
    with_entry_seed(&protected_page, |seed| {
        let declaration = crate::courtroom::harness::test_support::
                pre_decode_physical_admission_test_support::checksum_declaration()
                .admit_for_physical_integrity_entry(seed.entry_witness());
        let admission: PhysicalIntegrityAdmission<'_, '_> =
            seed.with_checksum_declaration(declaration).unwrap();
        let denial = admission
            .admit_page(PhysicalIntegrityAdmissionRequest::page(
                current_page_cell(),
                page_witness(b"page-poisoned"),
                PhysicalPageKind::DataPage,
                DeclaredPhysicalChecksum::new(crc32c(&current_page_bytes(b"page-expected"))),
            ))
            .unwrap_err();

        assert_eq!(denial.kind(), PreDecodePhysicalDenialKind::ChecksumMismatch);
        assert_localized_pre_decode_denial_counters(denial, protected_page.len() as u64, 1);
    });

    assert_localized_pre_decode_denial(deny_checked_frame(
        b"poisoned-but-parseable",
        b"poisoned-but-parseablf",
        PreDecodePhysicalDenialKind::ChecksumMismatch,
    ));
    assert_localized_pre_decode_denial(deny_checked_frame(
        b"checksum-source",
        b"checksum-different",
        PreDecodePhysicalDenialKind::ChecksumMismatch,
    ));
}

#[test]
fn truncated_frames_skip_logical_decode_with_exact_counters() {
    with_pre_decode_admission(b"abc", |admission, validation, _witness| {
        let denial = admission
            .admit_frame(PhysicalIntegrityAdmissionRequest::frame(
                validation,
                frame_witness(b"abc-but-witness-expects-more"),
                PhysicalFrameKind::RecordFrame,
                DeclaredPhysicalChecksum::new(crc32c(&current_frame_bytes(b"abc"))),
            ))
            .unwrap_err();

        assert_eq!(
            denial.kind(),
            PreDecodePhysicalDenialKind::TruncatedPhysicalFrame
        );
        assert_localized_pre_decode_denial_counters(
            denial,
            current_frame_bytes(b"abc").len() as u64,
            0,
        );
    });
}

#[test]
fn unsupported_checksum_algorithm_denies_before_inspection() {
    with_entry_seed(b"unsupported-algorithm", |seed| {
        let denial = seed
            .with_checksum_claim(
                ChecksumAlgorithmClaim::declared_text("sha256"),
                crate::courtroom::harness::test_support::pre_decode_physical_admission_test_support::checksum_scope(),
            )
            .unwrap_err();

        assert_eq!(
            denial.kind(),
            PreDecodePhysicalDenialKind::UnsupportedChecksumAlgorithm
        );
        assert_eq!(denial.counters().checksum_execution_count(), 0);
        assert_eq!(denial.locality(), None);
        assert_pre_decode_denial_counters(denial, b"unsupported-algorithm".len() as u64, 0);
    });
}

#[test]
fn stale_generation_denies_before_logical_decode() {
    with_pre_decode_admission(b"stale-generation", |admission, validation, witness| {
        let stale_validation = stale_validation();
        assert_ne!(validation.owner(), stale_validation.owner());
        let denial = admission
            .admit_frame(PhysicalIntegrityAdmissionRequest::frame(
                stale_validation,
                witness,
                PhysicalFrameKind::RecordFrame,
                DeclaredPhysicalChecksum::new(crc32c(&current_frame_bytes(b"stale-generation"))),
            ))
            .unwrap_err();

        assert_eq!(denial.kind(), PreDecodePhysicalDenialKind::StaleGeneration);
        assert_localized_pre_decode_denial_counters(
            denial,
            current_frame_bytes(b"stale-generation").len() as u64,
            0,
        );
    });
}

#[test]
fn poisoned_parseable_input_never_invokes_semantic_decoder_or_constructor() {
    let mut decoder = CountingSemanticDecoder::default();
    let denial = deny_checked_frame(
        b"{\"looks\":\"semantic\"}",
        b"{\"looks\":\"semantid\"}",
        PreDecodePhysicalDenialKind::ChecksumMismatch,
    );

    assert_localized_pre_decode_denial(denial);
    assert_eq!(decoder.invocations, 0);
    assert_eq!(decoder.semantic_index_lookups, 0);
    assert_eq!(decoder.domain_constructors, 0);

    with_pre_decode_admission(
        b"{\"looks\":\"semantic\"}",
        |admission, validation, witness| {
            let checked = admission
                .admit_frame(PhysicalIntegrityAdmissionRequest::frame(
                    validation,
                    witness,
                    PhysicalFrameKind::RecordFrame,
                    DeclaredPhysicalChecksum::new(crc32c(&current_frame_bytes(
                        b"{\"looks\":\"semantic\"}",
                    ))),
                ))
                .unwrap();
            decoder.decode(checked.logical_decode_gate());
        },
    );
    assert_eq!(decoder.invocations, 1);
    assert_eq!(decoder.semantic_index_lookups, 1);
    assert_eq!(decoder.domain_constructors, 1);
}
