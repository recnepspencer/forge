use super::source_identity::source_identity;
use super::{
    parse_ledger, read_repository_document, validate_exact_source_identity, LedgerRow,
    LedgerStatus, LEDGER,
};

const PHASE_TEN_SOURCE_FILES: &[&str] = &[
    ".cargo/config.toml",
    "Cargo.toml",
    "_docs/worth-store/physical-durability-and-checkpoints.md",
    "_docs/worth-store/physical-foundation-reconstruction-roadmap.md",
    "_docs/worth-store/physical-reality-audit.csv",
    "_docs/worth-store/physical-reconstruction-c7-authority-trace.csv",
    "_docs/worth-store/physical-reconstruction-c7-durable-publication-join.md",
    "_docs/worth-store/physical-reconstruction-c7-public-api.csv",
    "_docs/worth-store/physical-reconstruction-c7-removal-ledger.csv",
    "_docs/worth-store/storage-foundation-aspect-native-gate.md",
    "workspaces/worth-store/Cargo.lock",
    "workspaces/worth-store/Cargo.toml",
    "workspaces/worth-store/.cargo/config.toml",
];
const PHASE_TEN_SOURCE_TREES: &[&str] = &[
    "crates/worth-foundational",
    "crates/worth-proof",
    "crates/worth-signal",
    "workspaces/worth-store/crates",
    "workspaces/worth-store/tools/store-test-runner",
];

const PHASE_TEN_GUARANTEES: &[&str] = &[
    "C7-PERFORMANCE-01",
    "C7-CLEANUP-03",
    "C7-COURTROOM-01",
    "C7-HANDOFF-01",
    "C7-LEDGER-02",
];

fn phase_ten_source_identity() -> Result<String, String> {
    source_identity(
        "P10 closure",
        PHASE_TEN_SOURCE_FILES,
        PHASE_TEN_SOURCE_TREES,
        &[],
    )
}

#[test]
fn phase_ten_guarantees_close_the_final_source_and_entire_ledger() {
    let document = read_repository_document(LEDGER).expect("read C.7 closure ledger");
    let rows = parse_ledger(&document).expect("parse C.7 closure ledger");
    let source_identity = phase_ten_source_identity().expect("hash final C.7 source closure");
    validate_phase_ten_closure(&rows, &source_identity).unwrap_or_else(|denial| panic!("{denial}"));
}

#[test]
fn phase_ten_validator_rejects_omission_wrong_phase_stale_identity_and_reopened_history() {
    let document = read_repository_document(LEDGER).expect("read C.7 closure ledger");
    let rows = parse_ledger(&document).expect("parse C.7 closure ledger");
    let source_identity = phase_ten_source_identity().expect("hash final C.7 source closure");
    let rows = proved_phase_ten_fixture(rows, &source_identity);
    validate_phase_ten_closure(&rows, &source_identity)
        .expect("controlled Phase 10 ledger fixture must begin valid");

    let omitted = rows
        .iter()
        .filter(|row| row.id != "C7-CLEANUP-03")
        .cloned()
        .collect::<Vec<_>>();
    assert!(validate_phase_ten_closure(&omitted, &source_identity).is_err());

    let mut wrong_phase = rows.clone();
    phase_ten_row_mut(&mut wrong_phase, "C7-HANDOFF-01").phase = "9".to_owned();
    assert!(validate_phase_ten_closure(&wrong_phase, &source_identity).is_err());

    let mut stale = rows.clone();
    let stale_row = phase_ten_row_mut(&mut stale, "C7-COURTROOM-01");
    stale_row.status = LedgerStatus::Proved;
    stale_row.source_identity = "P10 closure deadbeefdead".to_owned();
    assert!(validate_phase_ten_closure(&stale, &source_identity).is_err());

    let mut reopened = rows;
    reopened
        .iter_mut()
        .find(|row| row.id == "C7-ROOT-01")
        .expect("ledger retains root guarantee")
        .status = LedgerStatus::Open;
    assert!(
        validate_phase_ten_closure(&reopened, &source_identity).is_err(),
        "MUTANT_PREDICATE:phase-ten-ledger-reopened-history-accepted"
    );
}

#[test]
fn phase_ten_guarantee_set_is_exact() {
    assert_eq!(
        PHASE_TEN_GUARANTEES,
        [
            "C7-PERFORMANCE-01",
            "C7-CLEANUP-03",
            "C7-COURTROOM-01",
            "C7-HANDOFF-01",
            "C7-LEDGER-02",
        ],
        "MUTANT_PREDICATE:phase-ten-ledger-guarantee-set-truncated"
    );
}

fn validate_phase_ten_closure(rows: &[LedgerRow], source_identity: &str) -> Result<(), String> {
    for guarantee in PHASE_TEN_GUARANTEES {
        let row = rows
            .iter()
            .find(|row| row.id == *guarantee)
            .ok_or_else(|| format!("C.7 ledger omits Phase 10 guarantee `{guarantee}`"))?;
        if row.phase != "10" {
            return Err(format!(
                "Phase 10 guarantee `{guarantee}` is assigned to phase {}",
                row.phase
            ));
        }
        if row.status != LedgerStatus::Proved {
            return Err(format!(
                "Phase 10 guarantee `{guarantee}` remains unresolved; current source is {source_identity}"
            ));
        }
        validate_exact_source_identity(rows, guarantee, source_identity)?;
    }

    if let Some(row) = rows.iter().find(|row| row.status != LedgerStatus::Proved) {
        return Err(format!(
            "C.7 ledger row `{}` remains unresolved at final closure",
            row.id
        ));
    }
    Ok(())
}

fn phase_ten_row_mut<'a>(rows: &'a mut [LedgerRow], identity: &str) -> &'a mut LedgerRow {
    rows.iter_mut()
        .find(|row| row.id == identity)
        .unwrap_or_else(|| panic!("missing controlled Phase 10 guarantee `{identity}`"))
}

fn proved_phase_ten_fixture(mut rows: Vec<LedgerRow>, source_identity: &str) -> Vec<LedgerRow> {
    for guarantee in PHASE_TEN_GUARANTEES {
        let row = phase_ten_row_mut(&mut rows, guarantee);
        row.phase = "10".to_owned();
        row.status = LedgerStatus::Proved;
        row.source_identity = source_identity.to_owned();
    }
    rows
}
