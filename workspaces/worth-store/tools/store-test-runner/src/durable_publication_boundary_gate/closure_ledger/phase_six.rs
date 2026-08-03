use super::source_identity::{
    phase_six_checkpoint_source_identity, phase_six_idempotency_source_identity,
    phase_six_reclamation_source_identity, phase_six_wal_source_identity,
};
use super::{
    parse_ledger, read_repository_document, validate_exact_source_identity, LedgerStatus, LEDGER,
};

const PHASE_SIX_WAL_GUARANTEE: &str = "C7-WAL-05";
const PHASE_SIX_IDEMPOTENCY_GUARANTEE: &str = "C7-IDEMPOTENCY-03";
const PHASE_SIX_RECLAMATION_GUARANTEE: &str = "C7-RETENTION-02";
const PHASE_SIX_CHECKPOINT_GUARANTEES: &[&str] = &[
    "C7-CHECKPOINT-01",
    "C7-CHECKPOINT-02",
    "C7-CHECKPOINT-03",
    "C7-RETENTION-01",
];

#[test]
fn proved_phase_six_wal_segment_lifecycle_tracks_exact_source_closure() {
    let document = read_repository_document(LEDGER).expect("read C.7 closure ledger");
    let rows = parse_ledger(&document).expect("parse C.7 closure ledger");
    let source_identity = phase_six_wal_source_identity().expect("hash Phase 6 WAL closure");
    let row = rows
        .iter()
        .find(|row| row.id == PHASE_SIX_WAL_GUARANTEE)
        .expect("C.7 ledger must contain the Phase 6 WAL guarantee");
    assert!(
        row.status == LedgerStatus::Proved,
        "Phase 6 WAL guarantee remains unresolved"
    );
    validate_exact_source_identity(&rows, PHASE_SIX_WAL_GUARANTEE, &source_identity)
        .unwrap_or_else(|denial| {
            panic!(
                "MUTANT_PREDICATE:phase-six-wal-evidence-source-closure-omission-accepted: {denial}"
            )
        });
}

#[test]
fn phase_six_wal_source_validator_rejects_stale_identity() {
    let document = read_repository_document(LEDGER).expect("read C.7 closure ledger");
    let mut rows = parse_ledger(&document).expect("parse C.7 closure ledger");
    let row = rows
        .iter_mut()
        .find(|row| row.id == PHASE_SIX_WAL_GUARANTEE)
        .expect("C.7 ledger must contain the Phase 6 WAL guarantee");
    row.status = LedgerStatus::Proved;
    row.source_identity = "P6 WAL deadbeefdead".to_owned();

    let source_identity = phase_six_wal_source_identity().expect("hash Phase 6 WAL closure");
    assert!(
        validate_exact_source_identity(&rows, PHASE_SIX_WAL_GUARANTEE, &source_identity).is_err(),
        "Phase 6 source closure validator accepted a stale identity"
    );
}

#[test]
fn proved_phase_six_checkpoint_guarantees_track_exact_source_closure() {
    let document = read_repository_document(LEDGER).expect("read C.7 closure ledger");
    let rows = parse_ledger(&document).expect("parse C.7 closure ledger");
    let source_identity =
        phase_six_checkpoint_source_identity().expect("hash Phase 6 checkpoint closure");
    for guarantee in PHASE_SIX_CHECKPOINT_GUARANTEES {
        let row = rows
            .iter()
            .find(|row| row.id == *guarantee)
            .expect("C.7 ledger must contain every Phase 6 checkpoint guarantee");
        assert!(
            row.status == LedgerStatus::Proved,
            "Phase 6 checkpoint guarantee `{guarantee}` remains unresolved"
        );
        validate_exact_source_identity(&rows, guarantee, &source_identity)
            .unwrap_or_else(|denial| panic!("{denial}"));
    }
}

#[test]
fn phase_six_checkpoint_source_validator_rejects_stale_identity() {
    let document = read_repository_document(LEDGER).expect("read C.7 closure ledger");
    let mut rows = parse_ledger(&document).expect("parse C.7 closure ledger");
    let row = rows
        .iter_mut()
        .find(|row| row.id == "C7-RETENTION-01")
        .expect("C.7 ledger must contain the Phase 6 retained-tail guarantee");
    row.status = LedgerStatus::Proved;
    row.source_identity = "P6 checkpoint deadbeefdead".to_owned();

    let source_identity =
        phase_six_checkpoint_source_identity().expect("hash Phase 6 checkpoint closure");
    assert!(
        validate_exact_source_identity(&rows, "C7-RETENTION-01", &source_identity,).is_err(),
        "Phase 6 checkpoint source closure validator accepted a stale identity"
    );
}

#[test]
fn proved_phase_six_idempotency_reopen_tracks_exact_source_closure() {
    let document = read_repository_document(LEDGER).expect("read C.7 closure ledger");
    let rows = parse_ledger(&document).expect("parse C.7 closure ledger");
    let source_identity =
        phase_six_idempotency_source_identity().expect("hash Phase 6 idempotency closure");
    let row = rows
        .iter()
        .find(|row| row.id == PHASE_SIX_IDEMPOTENCY_GUARANTEE)
        .expect("C.7 ledger must contain the Phase 6 idempotency guarantee");
    assert!(
        row.status == LedgerStatus::Proved,
        "Phase 6 idempotency guarantee remains unresolved"
    );
    validate_exact_source_identity(&rows, PHASE_SIX_IDEMPOTENCY_GUARANTEE, &source_identity)
        .unwrap_or_else(|denial| panic!("{denial}"));
}

#[test]
fn phase_six_idempotency_source_validator_rejects_stale_identity() {
    let document = read_repository_document(LEDGER).expect("read C.7 closure ledger");
    let mut rows = parse_ledger(&document).expect("parse C.7 closure ledger");
    let row = rows
        .iter_mut()
        .find(|row| row.id == PHASE_SIX_IDEMPOTENCY_GUARANTEE)
        .expect("C.7 ledger must contain the Phase 6 idempotency guarantee");
    row.status = LedgerStatus::Proved;
    row.source_identity = "P6 idempotency deadbeefdead".to_owned();

    let source_identity =
        phase_six_idempotency_source_identity().expect("hash Phase 6 idempotency closure");
    assert!(
        validate_exact_source_identity(&rows, PHASE_SIX_IDEMPOTENCY_GUARANTEE, &source_identity,)
            .is_err(),
        "Phase 6 idempotency source closure validator accepted a stale identity"
    );
}

#[test]
fn proved_phase_six_wal_reclamation_tracks_exact_source_closure() {
    let document = read_repository_document(LEDGER).expect("read C.7 closure ledger");
    let rows = parse_ledger(&document).expect("parse C.7 closure ledger");
    let source_identity =
        phase_six_reclamation_source_identity().expect("hash Phase 6 WAL reclamation closure");
    let row = rows
        .iter()
        .find(|row| row.id == PHASE_SIX_RECLAMATION_GUARANTEE)
        .expect("C.7 ledger must contain the Phase 6 WAL reclamation guarantee");
    assert!(
        row.status == LedgerStatus::Proved,
        "Phase 6 WAL reclamation guarantee remains unresolved"
    );
    validate_exact_source_identity(&rows, PHASE_SIX_RECLAMATION_GUARANTEE, &source_identity)
        .unwrap_or_else(|denial| panic!("{denial}"));
}

#[test]
fn phase_six_reclamation_source_validator_rejects_stale_identity() {
    let document = read_repository_document(LEDGER).expect("read C.7 closure ledger");
    let mut rows = parse_ledger(&document).expect("parse C.7 closure ledger");
    let row = rows
        .iter_mut()
        .find(|row| row.id == PHASE_SIX_RECLAMATION_GUARANTEE)
        .expect("C.7 ledger must contain the Phase 6 WAL reclamation guarantee");
    row.status = LedgerStatus::Proved;
    row.source_identity = "P6 reclamation deadbeefdead".to_owned();

    let source_identity =
        phase_six_reclamation_source_identity().expect("hash Phase 6 WAL reclamation closure");
    assert!(
        validate_exact_source_identity(&rows, PHASE_SIX_RECLAMATION_GUARANTEE, &source_identity,)
            .is_err(),
        "Phase 6 reclamation source closure validator accepted a stale identity"
    );
}
