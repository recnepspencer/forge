use super::source_identity::phase_five_group_source_identity;
use super::{parse_ledger, read_repository_document, validate_exact_source_identity, LEDGER};

#[test]
fn proved_phase_five_group_commit_tracks_its_exact_source_closure() {
    let document = read_repository_document(LEDGER).expect("read C.7 closure ledger");
    let rows = parse_ledger(&document).expect("parse C.7 closure ledger");
    let source_identity =
        phase_five_group_source_identity().expect("hash Phase 5 group-commit source closure");
    validate_exact_source_identity(&rows, "C7-GROUP-01", &source_identity)
        .unwrap_or_else(|denial| panic!("{denial}"));
}

#[test]
fn phase_five_source_validator_rejects_stale_identity() {
    let document = read_repository_document(LEDGER).expect("read C.7 closure ledger");
    let mut rows = parse_ledger(&document).expect("parse C.7 closure ledger");
    let source_identity =
        phase_five_group_source_identity().expect("hash Phase 5 group-commit source closure");
    let row = rows
        .iter_mut()
        .find(|row| row.id == "C7-GROUP-01")
        .expect("controlled Phase 5 guarantee exists");
    row.source_identity = "P5 group deadbeefdead".to_owned();
    assert!(validate_exact_source_identity(&rows, "C7-GROUP-01", &source_identity).is_err());
}
