use crate::physical_scope_admission_test_support::{
    root_with_slot, scope_membership, validation, with_checked_frame,
};
use forge_store_physical_format::{
    CheckpointAdjacencyPosture, PhysicalReferenceScope, RootManifestIntegrityPosture,
};
use forge_store_physical_integrity::{
    PhysicalScopeAdmission, PhysicalScopeAdmissionRequest, ScopedPhysicalValidatorInput,
    WalFrameDamageDenialKind, WalFrameIntegrityAuthority, WalFrameIntegrityInspectionRequest,
    WalTailIntegrityPosture,
};
use forge_store_recovery_physics::RecoveryPhysicsIntegrityInput;

#[test]
fn intact_wal_frame_independent_reads_produce_same_report_and_s4_input_identity() {
    let first = inspect_intact_wal_frame();
    let second = inspect_intact_wal_frame();
    let first_input = RecoveryPhysicsIntegrityInput::from_wal_integrity_report(&first);
    let second_input = RecoveryPhysicsIntegrityInput::from_wal_integrity_report(&second);

    assert_eq!(first, second);
    assert_eq!(first_input, second_input);
    assert_eq!(first.tail_posture(), WalTailIntegrityPosture::IntactTail);
    assert_eq!(first.counters().protected_window_reads(), 1);
    assert_eq!(first.counters().frame_header_checks(), 1);
    assert_eq!(first.counters().payload_boundary_checks(), 1);
    assert_eq!(first.counters().checkpoint_adjacency_checks(), 1);
    assert_eq!(first.counters().checksum_posture_checks(), 1);
    assert_eq!(first.counters().tail_posture_checks(), 1);
    assert_eq!(first.counters().skipped_replay_attempts(), 1);
}

#[test]
fn wal_frame_denials_stay_typed_and_do_not_construct_replay_inputs() {
    let checksum = inspect_denial(wal_payload("crc32c", 4, "checksum-fail", b"DATA"));
    assert_eq!(checksum.kind(), WalFrameDamageDenialKind::ChecksumFailure);
    assert_eq!(checksum.counters().skipped_replay_attempts(), 1);

    let torn = inspect_denial(wal_payload("crc32c", 16, "ok", b"DATA"));
    assert_eq!(torn.kind(), WalFrameDamageDenialKind::TornWalFrame);
    assert_eq!(torn.tail_posture(), WalTailIntegrityPosture::TornTail);
    assert_eq!(torn.expected_length(), Some(16));
    assert_eq!(torn.actual_length(), Some(4));

    let mismatched = inspect_denial(wal_payload("crc32c", 2, "ok", b"DATA"));
    assert_eq!(
        mismatched.kind(),
        WalFrameDamageDenialKind::MismatchedLength
    );
    assert_eq!(mismatched.expected_length(), Some(2));
    assert_eq!(mismatched.actual_length(), Some(4));

    let unsupported = inspect_denial(wal_payload("sha1", 4, "ok", b"DATA"));
    assert_eq!(
        unsupported.kind(),
        WalFrameDamageDenialKind::UnsupportedAlgorithm
    );
    assert!(unsupported.checksum_denial().is_some());

    let malformed_length = inspect_denial(b"WALF|crc32c||ok|".to_vec());
    assert_eq!(
        malformed_length.kind(),
        WalFrameDamageDenialKind::UnknownTailIntegrity
    );
    assert_eq!(
        malformed_length.tail_posture(),
        WalTailIntegrityPosture::UnknownTailIntegrity
    );
}

#[test]
fn wal_tail_and_checkpoint_adjacent_postures_remain_distinct_without_replay() {
    let checkpoint =
        inspect_checkpoint_denial(wal_payload("crc32c", 4, "checkpoint-damage", b"DATA"));
    assert_eq!(
        checkpoint.kind(),
        WalFrameDamageDenialKind::CheckpointAdjacentCorruption
    );
    assert_eq!(
        checkpoint.tail_posture(),
        WalTailIntegrityPosture::CheckpointAdjacentDamage
    );
    assert!(checkpoint.checkpoint_adjacent_damage().is_some());

    let non_adjacent_checkpoint =
        inspect_denial(wal_payload("crc32c", 4, "checkpoint-damage", b"DATA"));
    assert_eq!(
        non_adjacent_checkpoint.kind(),
        WalFrameDamageDenialKind::UnknownTailIntegrity
    );
    assert_eq!(
        non_adjacent_checkpoint.tail_posture(),
        WalTailIntegrityPosture::UnknownTailIntegrity
    );
    assert!(non_adjacent_checkpoint
        .checkpoint_adjacent_damage()
        .is_none());

    let unknown = inspect_denial(wal_payload("crc32c", 4, "unknown", b"DATA"));
    assert_eq!(
        unknown.kind(),
        WalFrameDamageDenialKind::UnknownTailIntegrity
    );
    assert_eq!(
        unknown.tail_posture(),
        WalTailIntegrityPosture::UnknownTailIntegrity
    );

    let precedence = inspect_denial(wal_payload(
        "crc32c",
        4,
        "recovery-precedence-required",
        b"DATA",
    ));
    assert_eq!(
        precedence.kind(),
        WalFrameDamageDenialKind::RecoveryPrecedenceRequired
    );
    assert_eq!(
        precedence.tail_posture(),
        WalTailIntegrityPosture::RecoveryPrecedenceRequired
    );
}

#[test]
fn checkpoint_adjacent_intact_input_has_distinct_checkpoint_report_without_replay() {
    let mut report = None;
    with_wal_frame_input(
        wal_payload("crc32c", 4, "ok", b"DATA"),
        CheckpointAdjacencyPosture::CheckpointAdjacent,
        |input| {
            let request =
                WalFrameIntegrityInspectionRequest::from_admitted_wal_frame(input).unwrap();
            report = Some(
                WalFrameIntegrityAuthority::s3()
                    .inspect_checkpoint_adjacent(request)
                    .unwrap(),
            );
        },
    );
    let report = report.unwrap();

    assert_eq!(report.tail_posture(), WalTailIntegrityPosture::IntactTail);
    assert_eq!(
        report.input_identity().checkpoint_adjacency(),
        CheckpointAdjacencyPosture::CheckpointAdjacent
    );
    assert_eq!(report.counters().skipped_replay_attempts(), 1);
}

#[test]
fn non_checkpoint_adjacent_wal_frame_cannot_mint_checkpoint_record_report() {
    let mut denial = None;
    with_wal_frame_input(
        wal_payload("crc32c", 4, "ok", b"DATA"),
        CheckpointAdjacencyPosture::NotCheckpointAdjacent,
        |input| {
            let request =
                WalFrameIntegrityInspectionRequest::from_admitted_wal_frame(input).unwrap();
            denial = Some(
                WalFrameIntegrityAuthority::s3()
                    .inspect_checkpoint_adjacent(request)
                    .unwrap_err(),
            );
        },
    );
    let denial = denial.unwrap();

    assert_eq!(
        denial.kind(),
        WalFrameDamageDenialKind::WrongCheckpointAdjacency
    );
    assert_eq!(denial.tail_posture(), WalTailIntegrityPosture::IntactTail);
    assert_eq!(denial.counters().skipped_replay_attempts(), 1);
}

fn inspect_intact_wal_frame() -> forge_store_physical_integrity::WalFrameIntegrityReport {
    let mut report = None;
    with_wal_frame_input(
        wal_payload("crc32c", 4, "ok", b"DATA"),
        CheckpointAdjacencyPosture::NotCheckpointAdjacent,
        |input| {
            let request =
                WalFrameIntegrityInspectionRequest::from_admitted_wal_frame(input).unwrap();
            report = Some(WalFrameIntegrityAuthority::s3().inspect(request).unwrap());
        },
    );
    report.unwrap()
}

pub(crate) fn inspect_denial(
    payload: Vec<u8>,
) -> forge_store_physical_integrity::WalFrameDamageDenial {
    inspect_denial_with_adjacency(payload, CheckpointAdjacencyPosture::NotCheckpointAdjacent)
}

fn inspect_checkpoint_denial(
    payload: Vec<u8>,
) -> forge_store_physical_integrity::WalFrameDamageDenial {
    inspect_denial_with_adjacency(payload, CheckpointAdjacencyPosture::CheckpointAdjacent)
}

fn inspect_denial_with_adjacency(
    payload: Vec<u8>,
    adjacency: CheckpointAdjacencyPosture,
) -> forge_store_physical_integrity::WalFrameDamageDenial {
    let mut denial = None;
    with_wal_frame_input(payload, adjacency, |input| {
        let request = WalFrameIntegrityInspectionRequest::from_admitted_wal_frame(input).unwrap();
        denial = Some(
            WalFrameIntegrityAuthority::s3()
                .inspect(request)
                .unwrap_err(),
        );
    });
    denial.unwrap()
}

fn with_wal_frame_input(
    payload: Vec<u8>,
    adjacency: CheckpointAdjacencyPosture,
    run: impl FnOnce(ScopedPhysicalValidatorInput<'_>),
) {
    let validation = validation(1, 2, 3, 7);
    let scope = PhysicalReferenceScope::wal_frame(validation);
    let root = root_with_slot(1, 2, 3, 7);
    let membership = scope_membership(&root, scope);
    with_checked_frame(&payload, validation, |checked| {
        let request = PhysicalScopeAdmissionRequest::frame(
            scope,
            membership,
            RootManifestIntegrityPosture::current_root_admitted(membership),
            adjacency,
            checked.gate_evidence().coverage_basis().clone(),
        );
        let admission = PhysicalScopeAdmission::admit_frame(checked, request).unwrap();
        run(ScopedPhysicalValidatorInput::wal_frame(admission).unwrap());
    });
}

pub(crate) fn wal_payload(
    algorithm: &str,
    declared_len: usize,
    status: &str,
    body: &[u8],
) -> Vec<u8> {
    let mut payload = format!("WALF|{algorithm}|{declared_len}|{status}|").into_bytes();
    payload.extend_from_slice(body);
    payload
}
