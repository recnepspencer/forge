use super::{milestone_314_ledger, repository_document};

const PHASE_LEDGER: &str = "_docs/worth-ui/milestone-3.14-phase-1-proof-ledger.csv";
const IDS: [&str; 9] = [
    "P1-01", "P1-02", "P1-03", "P1-04", "P1-05", "P1-06", "P1-07", "P1-08", "P1-09",
];

fn inputs() -> (toml::Value, String) {
    let contract: toml::Value = toml::from_str(&repository_document(
        "_docs/worth-ui/milestone-3.14-phase-1-contract.toml",
    ))
    .expect("Phase 1 contract should parse");
    (contract, repository_document(PHASE_LEDGER))
}

fn validate(contract: &toml::Value, ledger: &str) -> Result<(), String> {
    if contract["phase_ledger"].as_str() != Some(PHASE_LEDGER) {
        return Err("contract phase-ledger path drifted".to_owned());
    }
    let rows = milestone_314_ledger::parse_ledger(ledger)?;
    if rows.len() != IDS.len() {
        return Err("Phase 1 proof-ledger row count drifted".to_owned());
    }
    for (row, expected_id) in rows.iter().zip(IDS) {
        if row[0] != expected_id {
            return Err(format!("expected {expected_id}, observed {}", row[0]));
        }
        if row[8] != "PROVED" {
            return Err(format!("{expected_id} is not proved"));
        }
        if row[1..8].iter().any(String::is_empty) {
            return Err(format!("{expected_id} has an empty proof obligation"));
        }
        if !row[9].contains("source=") || !row[9].contains("result=") || !row[9].contains("owner=")
        {
            return Err(format!("{expected_id} evidence is not auditable"));
        }
    }
    Ok(())
}

#[test]
fn phase_1_closure_ledger_is_exact_and_independent_from_the_open_ia_portfolio() {
    let (contract, ledger) = inputs();
    validate(&contract, &ledger).expect("Phase 1 closure ledger should be exact");
    let milestone = repository_document("_docs/worth-ui/milestone-3.14-proof-ledger.csv");
    assert!(milestone
        .lines()
        .skip(1)
        .all(|row| row.contains("\"OPEN\"")));
}

#[test]
fn phase_1_closure_ledger_rejects_hostile_status_and_structure_mutations() {
    let (contract, ledger) = inputs();
    let rows = milestone_314_ledger::parse_ledger(&ledger).expect("ledger should parse");

    let mut open = rows.clone();
    open[4][8] = "OPEN".to_owned();
    assert!(validate(&contract, &milestone_314_ledger::render_ledger(&open)).is_err());

    let mut missing = rows.clone();
    missing.remove(4);
    assert!(validate(&contract, &milestone_314_ledger::render_ledger(&missing)).is_err());

    let mut duplicate = rows.clone();
    duplicate[4] = duplicate[3].clone();
    assert!(validate(&contract, &milestone_314_ledger::render_ledger(&duplicate)).is_err());

    let mut unauditable = rows;
    unauditable[4][9] = "tests passed".to_owned();
    assert!(validate(
        &contract,
        &milestone_314_ledger::render_ledger(&unauditable)
    )
    .is_err());
}
