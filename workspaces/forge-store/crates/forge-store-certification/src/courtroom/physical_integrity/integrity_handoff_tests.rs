use crate::{
    courtroom::harness::test_support::pre_decode_physical_admission_test_support::checksum_scope,
    courtroom::harness::test_support::integrity_handoff_test_support::{
        forged_inspection_envelope_counter_denial_kind, intact_readiness,
        manifest_receipt_swap_denial_kind,
    },
    courtroom::harness::test_support::recovery_blocking_damage_test_support::{
        assert_all_recovery_blocking_sources, recovery_blocking_damage_fixture,
    },
};
use forge_store_physical_integrity::{ChecksumAlgorithmId, WalFrameDamageDenialKind};
use forge_store_recovery_physics::IntegrityHandoffDenialKind;

#[test]
fn intact_inputs_publish_stable_s4_handoff_identity_across_independent_runs() {
    let first = intact_readiness("stable-handoff");
    let second = intact_readiness("stable-handoff");

    assert_eq!(first.payload().identity(), second.payload().identity());
    assert_eq!(first.counters(), second.counters());
    assert_eq!(
        first.payload().root_manifest(),
        second.payload().root_manifest()
    );
    assert_eq!(
        first.payload().segment_manifest(),
        second.payload().segment_manifest()
    );
    assert_eq!(first.payload().wal_frames(), second.payload().wal_frames());
    assert_eq!(
        first.payload().checkpoint_records(),
        second.payload().checkpoint_records()
    );
    assert!(first.proves_no_raw_bytes_crossed());
    assert!(!first.claims_recovery());
}

#[test]
fn damaged_inputs_publish_typed_recovery_blockers_instead_of_replay_inputs() {
    let fixture = recovery_blocking_damage_fixture();
    let damage_map = fixture.damage_map();

    assert_eq!(
        fixture.wal_kind(),
        WalFrameDamageDenialKind::ChecksumFailure
    );
    assert_eq!(
        fixture.checkpoint_kind(),
        WalFrameDamageDenialKind::CheckpointAdjacentCorruption
    );
    assert_all_recovery_blocking_sources(damage_map);
    assert_eq!(damage_map.recovery_blocking_findings().len(), 4);
}

#[test]
fn s4_handoff_payload_exposes_required_integrity_surfaces_and_exact_counters() {
    let readiness = intact_readiness("payload-proof");
    let payload = readiness.payload();
    let counters = payload.counters();

    assert_eq!(payload.root_manifest().counters().root_manifest_reads(), 1);
    assert_eq!(
        payload
            .segment_manifest()
            .counters()
            .segment_manifest_reads(),
        1
    );
    assert_eq!(payload.page_frames().len(), 1);
    assert_eq!(payload.wal_frames().len(), 1);
    assert_eq!(payload.checkpoint_records().len(), 1);
    assert_eq!(payload.damage_map().quarantine_summaries().len(), 1);
    assert_eq!(
        payload.checksum_basis().algorithm(),
        ChecksumAlgorithmId::crc32c()
    );
    assert_eq!(payload.checksum_basis().scope(), &checksum_scope());
    assert_eq!(payload.inspection_envelope().resident_byte_limit(), 128);
    assert_eq!(payload.inspection_envelope().protected_read_limit(), 128);
    assert_eq!(payload.inspection_envelope().streaming_window_limit(), 128);
    assert_eq!(counters.vetted_record_count(), 5);
    assert_eq!(counters.quarantine_summary_count(), 1);
    assert_eq!(counters.recovery_blocking_count(), 0);
    assert_eq!(
        counters.checked_byte_count(),
        payload
            .inspection_envelope()
            .counters()
            .checked_byte_count()
    );
    assert_eq!(counters.checksum_execution_count(), 1);
    assert_eq!(counters.skipped_decode_count(), 0);
    assert!(payload.proves_no_raw_bytes_crossed());
    assert!(!payload.claims_recovery());
}

#[test]
fn handoff_rejects_manifest_receipt_swaps_and_forged_envelope_counters() {
    assert_eq!(
        manifest_receipt_swap_denial_kind(7, 8),
        IntegrityHandoffDenialKind::ReceiptBasisMismatch
    );
    assert_eq!(
        forged_inspection_envelope_counter_denial_kind(b"forged-envelope"),
        IntegrityHandoffDenialKind::InspectionEnvelopeExceeded
    );
}
