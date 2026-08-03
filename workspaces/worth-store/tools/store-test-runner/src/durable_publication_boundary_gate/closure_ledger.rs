use std::collections::{BTreeMap, BTreeSet};

use self::source_identity::{
    phase_four_data_source_identity, phase_one_source_identity, phase_three_wal_source_identity,
    phase_two_authority_source_identity, phase_two_mutation_source_identity,
};
use super::read_repository_document;

mod phase_eight;
mod phase_five;
mod phase_nine;
mod phase_seven;
mod phase_six;
mod phase_ten;
mod source_identity;

const LEDGER: &str = "_docs/worth-store/physical-reconstruction-c7-closure-ledger.md";
const REQUIRED_FAMILIES: &[&str] = &[
    "AUTHORITY",
    "API",
    "DESTINATION",
    "GRAPH",
    "LEDGER",
    "MECHANISM",
    "VOCABULARY",
    "IDEMPOTENCY",
    "WAL",
    "BARRIER",
    "DATA",
    "GROUP",
    "CHECKPOINT",
    "RETENTION",
    "ROOT",
    "FAILURE",
    "LIFECYCLE",
    "SIGNAL",
    "FOUNDATIONAL",
    "PERFORMANCE",
    "CLEANUP",
    "DOCUMENTATION",
    "COURTROOM",
    "HANDOFF",
];
const PHASE_FOUR_GUARANTEES: &[&str] = &[
    "C7-BARRIER-01",
    "C7-DATA-01",
    "C7-DATA-02",
    "C7-DATA-03",
    "C7-DATA-04",
    "C7-FAILURE-01",
];

#[test]
fn living_ledger_covers_the_complete_causal_claim_and_phase_one_controls() {
    let document = read_repository_document(LEDGER).expect("read C.7 closure ledger");
    let rows = parse_ledger(&document).expect("parse C.7 closure ledger");
    validate_ledger(&rows).unwrap_or_else(|denial| panic!("{denial}"));
}

#[test]
fn phase_one_guarantees_are_resolved_before_phase_two_can_begin() {
    let document = read_repository_document(LEDGER).expect("read C.7 closure ledger");
    let rows = parse_ledger(&document).expect("parse C.7 closure ledger");
    let source_identity = phase_one_source_identity().expect("hash Phase 1 source closure");
    validate_phase_one_source_identity(&rows, &source_identity)
        .unwrap_or_else(|denial| panic!("{denial}"));
    for row in rows.iter().filter(|row| row.phase == "1") {
        assert!(
            row.status == LedgerStatus::Proved,
            "Phase 1 guarantee `{}` remains unresolved",
            row.id
        );
    }
    assert!(
        rows.iter().any(|row| row.phase == "1"),
        "C.7 ledger contains no Phase 1 guarantees"
    );
}

#[test]
fn proved_phase_two_authority_tracks_its_exact_source_closure() {
    let document = read_repository_document(LEDGER).expect("read C.7 closure ledger");
    let rows = parse_ledger(&document).expect("parse C.7 closure ledger");
    let source_identity =
        phase_two_authority_source_identity().expect("hash Phase 2 authority closure");
    validate_exact_source_identity(&rows, "C7-AUTHORITY-02", &source_identity)
        .unwrap_or_else(|denial| panic!("{denial}"));
}

#[test]
fn proved_phase_two_mutation_guarantees_track_their_exact_source_closure() {
    let document = read_repository_document(LEDGER).expect("read C.7 closure ledger");
    let rows = parse_ledger(&document).expect("parse C.7 closure ledger");
    let source_identity =
        phase_two_mutation_source_identity().expect("hash Phase 2 mutation closure");
    for guarantee in [
        "C7-IDEMPOTENCY-01",
        "C7-IDEMPOTENCY-02",
        "C7-SIGNAL-01",
        "C7-FOUNDATIONAL-01",
    ] {
        validate_exact_source_identity(&rows, guarantee, &source_identity)
            .unwrap_or_else(|denial| panic!("{denial}"));
    }
}

#[test]
fn proved_phase_three_wal_guarantees_track_their_exact_source_closure() {
    let document = read_repository_document(LEDGER).expect("read C.7 closure ledger");
    let rows = parse_ledger(&document).expect("parse C.7 closure ledger");
    let source_identity =
        phase_three_wal_source_identity().expect("hash Phase 3 WAL source closure");
    for guarantee in ["C7-WAL-01", "C7-WAL-02", "C7-WAL-03", "C7-WAL-04"] {
        validate_exact_source_identity(&rows, guarantee, &source_identity)
            .unwrap_or_else(|denial| panic!("{denial}"));
    }
}

#[test]
fn proved_phase_four_barrier_and_data_guarantees_track_their_exact_source_closure() {
    let document = read_repository_document(LEDGER).expect("read C.7 closure ledger");
    let rows = parse_ledger(&document).expect("parse C.7 closure ledger");
    let source_identity =
        phase_four_data_source_identity().expect("hash Phase 4 barrier and data source closure");
    for guarantee in PHASE_FOUR_GUARANTEES {
        validate_exact_source_identity(&rows, guarantee, &source_identity)
            .unwrap_or_else(|denial| panic!("{denial}"));
    }
}

fn parse_ledger(document: &str) -> Result<Vec<LedgerRow>, String> {
    let mut rows = Vec::new();
    let mut in_guarantee_table = false;
    for (line_number, line) in document.lines().enumerate() {
        if line.starts_with("| ID | Phase | Guarantee |") {
            in_guarantee_table = true;
            continue;
        }
        if in_guarantee_table && line.starts_with("## ") {
            break;
        }
        if !in_guarantee_table {
            continue;
        }
        if !line.starts_with("| C7-") {
            continue;
        }
        let columns = line
            .trim_matches('|')
            .split('|')
            .map(str::trim)
            .collect::<Vec<_>>();
        if columns.len() != 7 || columns.iter().any(|column| column.is_empty()) {
            return Err(format!(
                "invalid C.7 closure row at line {}",
                line_number + 1
            ));
        }
        rows.push(LedgerRow {
            id: columns[0].to_owned(),
            phase: columns[1].to_owned(),
            guarantee: columns[2].to_owned(),
            required_evidence: columns[3].to_owned(),
            current_evidence: columns[4].to_owned(),
            source_identity: columns[5].to_owned(),
            status: LedgerStatus::parse(columns[6])?,
        });
    }
    if rows.is_empty() {
        return Err("C.7 closure ledger contains no guarantee rows".to_owned());
    }
    Ok(rows)
}

fn validate_ledger(rows: &[LedgerRow]) -> Result<(), String> {
    let mut identities = BTreeSet::new();
    let mut families = BTreeMap::<String, usize>::new();
    for row in rows {
        if !identities.insert(row.id.clone()) {
            return Err(format!("duplicate C.7 closure identity `{}`", row.id));
        }
        let family = row
            .id
            .strip_prefix("C7-")
            .and_then(|rest| rest.split('-').next())
            .ok_or_else(|| format!("invalid C.7 closure identity `{}`", row.id))?;
        *families.entry(family.to_owned()).or_default() += 1;
        if row.guarantee.len() < 24 || row.required_evidence.len() < 12 {
            return Err(format!(
                "C.7 closure row `{}` is not a semantic guarantee",
                row.id
            ));
        }
        if row.status == LedgerStatus::Proved
            && (row.current_evidence == "pending" || row.source_identity == "pending")
        {
            return Err(format!("C.7 closure row `{}` claims stale proof", row.id));
        }
        if row.phase == "1" && row.status == LedgerStatus::NotApplicable {
            return Err(format!("Phase 1 control `{}` cannot be N/A", row.id));
        }
    }
    for family in REQUIRED_FAMILIES {
        if !families.contains_key(*family) {
            return Err(format!("C.7 closure ledger omits `{family}` guarantees"));
        }
    }
    for guarantee in PHASE_FOUR_GUARANTEES {
        let row = rows
            .iter()
            .find(|row| row.id == *guarantee)
            .ok_or_else(|| format!("C.7 closure ledger omits Phase 4 guarantee `{guarantee}`"))?;
        if row.phase != "4" {
            return Err(format!(
                "Phase 4 guarantee `{guarantee}` is assigned to phase {}",
                row.phase
            ));
        }
    }
    Ok(())
}

fn validate_phase_one_source_identity(rows: &[LedgerRow], expected: &str) -> Result<(), String> {
    for row in rows
        .iter()
        .filter(|row| row.phase == "1" && row.status == LedgerStatus::Proved)
    {
        if row.source_identity != expected {
            return Err(format!(
                "Phase 1 guarantee `{}` has stale source identity `{}`; expected `{expected}`",
                row.id, row.source_identity
            ));
        }
    }
    Ok(())
}

fn validate_exact_source_identity(
    rows: &[LedgerRow],
    guarantee: &str,
    expected: &str,
) -> Result<(), String> {
    let row = rows
        .iter()
        .find(|row| row.id == guarantee)
        .ok_or_else(|| format!("C.7 closure ledger omits `{guarantee}`"))?;
    if row.status == LedgerStatus::Proved && row.source_identity != expected {
        return Err(format!(
            "Guarantee `{guarantee}` has stale source identity `{}`; expected `{expected}`",
            row.source_identity
        ));
    }
    Ok(())
}

#[test]
fn ledger_validator_rejects_omitted_family_duplicate_and_stale_proof() {
    let document = read_repository_document(LEDGER).expect("read C.7 closure ledger");
    let rows = parse_ledger(&document).expect("parse C.7 closure ledger");

    let omitted = rows
        .iter()
        .filter(|row| !row.id.starts_with("C7-HANDOFF-"))
        .cloned()
        .collect::<Vec<_>>();
    assert!(validate_ledger(&omitted).is_err());

    let omitted_retention = rows
        .iter()
        .filter(|row| !row.id.starts_with("C7-RETENTION-"))
        .cloned()
        .collect::<Vec<_>>();
    assert!(validate_ledger(&omitted_retention).is_err());

    let mut duplicate = rows.clone();
    duplicate.push(rows[0].clone());
    assert!(validate_ledger(&duplicate).is_err());

    let omitted_phase_four = rows
        .iter()
        .filter(|row| row.id != "C7-DATA-03")
        .cloned()
        .collect::<Vec<_>>();
    assert!(validate_ledger(&omitted_phase_four).is_err());

    let mut stale = rows.clone();
    stale[0].status = LedgerStatus::Proved;
    stale[0].current_evidence = "pending".to_owned();
    assert!(validate_ledger(&stale).is_err());
}

#[test]
fn source_identity_validators_reject_stale_phase_closures() {
    let document = read_repository_document(LEDGER).expect("read C.7 closure ledger");
    let rows = parse_ledger(&document).expect("parse C.7 closure ledger");

    let source_identity = phase_one_source_identity().expect("hash Phase 1 source closure");
    let mut stale_identity = rows.clone();
    stale_identity[0].source_identity = "P1 closure deadbeefdead".to_owned();
    assert!(validate_phase_one_source_identity(&stale_identity, &source_identity).is_err());

    let source_identity =
        phase_two_authority_source_identity().expect("hash Phase 2 authority closure");
    assert_stale_exact_identity_rejected(
        &rows,
        "C7-AUTHORITY-02",
        &source_identity,
        "P2 authority deadbeefdead",
    );

    let source_identity =
        phase_two_mutation_source_identity().expect("hash Phase 2 mutation closure");
    assert_stale_exact_identity_rejected(
        &rows,
        "C7-IDEMPOTENCY-01",
        &source_identity,
        "P2 mutation deadbeefdead",
    );

    let source_identity = phase_three_wal_source_identity().expect("hash Phase 3 WAL closure");
    assert_stale_exact_identity_rejected(
        &rows,
        "C7-WAL-01",
        &source_identity,
        "P3 WAL deadbeefdead",
    );

    let source_identity =
        phase_four_data_source_identity().expect("hash Phase 4 barrier and data closure");
    assert_stale_exact_identity_rejected(
        &rows,
        "C7-DATA-01",
        &source_identity,
        "P4 data deadbeefdead",
    );
}

fn assert_stale_exact_identity_rejected(
    rows: &[LedgerRow],
    guarantee: &str,
    expected: &str,
    stale: &str,
) {
    let mut rows = rows.to_vec();
    let row = rows
        .iter_mut()
        .find(|row| row.id == guarantee)
        .unwrap_or_else(|| panic!("missing controlled guarantee `{guarantee}`"));
    row.status = LedgerStatus::Proved;
    row.source_identity = stale.to_owned();
    assert!(validate_exact_source_identity(&rows, guarantee, expected).is_err());
}

#[derive(Clone)]
struct LedgerRow {
    id: String,
    phase: String,
    guarantee: String,
    required_evidence: String,
    current_evidence: String,
    source_identity: String,
    status: LedgerStatus,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum LedgerStatus {
    Open,
    Proved,
    Defect,
    Blocked,
    NotApplicable,
}

impl LedgerStatus {
    fn parse(value: &str) -> Result<Self, String> {
        match value {
            "OPEN" => Ok(Self::Open),
            "PROVED" => Ok(Self::Proved),
            "DEFECT" => Ok(Self::Defect),
            "BLOCKED" => Ok(Self::Blocked),
            "N/A" => Ok(Self::NotApplicable),
            _ => Err(format!("invalid C.7 closure status `{value}`")),
        }
    }
}
