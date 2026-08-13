use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::sync::atomic::{AtomicU64, Ordering};
#[path = "ledger_mutants.rs"]
mod ledger_mutants;
#[path = "proved_fixture_values.rs"]
mod proved_fixture_values;
use super::{
    parse, repository_document, result_artifact, source_digest, validate_phase_closure,
    validate_phase_progression, validate_row, EXPECTED_REQUIREMENTS, HEADER, LEDGER,
};
use ledger_mutants::{duplicate_first_data_row, remove_first_data_row, swap_first_data_rows};
use proved_fixture_values::*;

const PRODUCTION_SOURCE: &str =
    "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/authority/work.rs";
const ORACLE_SOURCE: &str =
    "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/work_producer_tests.rs";
const TEST_NAME: &str = "mounting::presentation::work_producer_tests::one_replacement_carries_one_change_and_exact_predecessor_successor_damage";

static FIXTURE_SEQUENCE: AtomicU64 = AtomicU64::new(0);
#[test]
fn milestone_ledger_has_exact_schema_inventory_and_honest_posture() {
    let rows = parse(&super::ledger_document()).expect("the milestone ledger should parse");
    let observed = rows.keys().map(String::as_str).collect::<BTreeSet<_>>();
    assert_eq!(observed, EXPECTED_REQUIREMENTS.into_iter().collect());
    assert!(rows.values().all(|row| matches!(
        (row["result"].as_str(), row["final_source"].as_str()),
        ("OPEN", "false") | ("PROVED", "true")
    )));
}

#[test]
fn validator_rejects_inventory_schema_and_premature_proof_mutants() {
    let ledger = repository_document(LEDGER);
    assert!(parse(&ledger).is_ok());
    assert!(parse(&remove_first_data_row(&ledger)).is_err());
    assert!(parse(&duplicate_first_data_row(&ledger)).is_err());
    assert!(parse(&swap_first_data_rows(&ledger)).is_err());
    assert!(parse(&ledger.replacen("phase,requirement", "requirement,phase", 1)).is_err());
    let fixture = ProvedFixture::new();
    let mut nonfinal = fixture.record.clone();
    set(&mut nonfinal, "final_source", "false");
    assert!(parse(&ledger_with_first_record(&nonfinal)).is_err());
    assert!(parse(&ledger.replacen("protocol=4", "protocol=3", 1)).is_err());
}

#[test]
fn proved_rows_cannot_hide_blank_stale_or_unbound_evidence() {
    let fixture = ProvedFixture::new();
    assert_valid_fixture(&fixture.record);
    for column in HEADER {
        if matches!(column, "phase" | "requirement" | "result" | "final_source") {
            continue;
        }
        let mut mutant = fixture.record.clone();
        set(&mut mutant, column, "");
        assert!(
            parse(&ledger_with_first_record(&mutant)).is_err(),
            "blank {column} must fail"
        );
    }
    for (column, replacement) in [
        ("baseline_digest", "stale"),
        ("exact_command", "cargo test -p worth-ui-host-contract"),
        ("matched_test_count", "0"),
        ("command_result", "failed"),
        ("retained_result_artifact", "missing/result.json"),
        (
            "source_revision",
            "0000000000000000000000000000000000000000",
        ),
        (
            "source_digest",
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        ),
        (
            "source_state_digest",
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        ),
        ("run_nonce", "not-one-governed-run"),
        ("source_identity", "missing/source.rs"),
        ("world_version", "zero"),
        ("final_source", "false"),
        (
            "result_artifact_digest",
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        ),
    ] {
        let mut mutant = fixture.record.clone();
        set(&mut mutant, column, replacement);
        assert!(
            parse(&ledger_with_first_record(&mutant)).is_err(),
            "stale {column} must fail"
        );
    }
}

#[test]
fn named_entries_and_runner_filter_must_resolve_to_the_bound_oracle() {
    let fixture = ProvedFixture::new();
    for (column, replacement) in [
        (
            "production_entry",
            format!("{PRODUCTION_SOURCE}::missing_issue_delta"),
        ),
        (
            "independent_oracle",
            format!("{ORACLE_SOURCE}::missing_oracle"),
        ),
    ] {
        let mut mutant = fixture.record.clone();
        set(&mut mutant, column, &replacement);
        assert!(parse(&ledger_with_first_record(&mutant)).is_err());
    }
    assert!(
        super::row_evidence::validate_named_entry(&format!("{ORACLE_SOURCE}::missing_oracle"))
            .is_err()
    );
    let mut zero_match = bound_record(&fixture.record);
    let command = zero_match[column("exact_command")].replace(TEST_NAME, "missing::test_name");
    set(&mut zero_match, "exact_command", &command);
    assert!(parse(&ledger_with_first_record(&zero_match)).is_err());
}

#[test]
fn retained_artifact_rejects_zero_match_failure_and_stale_source_mutants() {
    for (field, value) in [
        ("matched_test_count", json!(0)),
        ("declared_ignored_test_count", json!(1)),
        ("expected_declared_ignored", json!(true)),
        ("executed_test_count", json!(0)),
        ("passed_test_count", json!(0)),
        ("ignored_test_count", json!(1)),
        ("exit_posture", json!("test-failed")),
        ("list_exit_code", json!(1)),
        ("test_exit_code", Value::Null),
        (
            "source_revision",
            json!("0000000000000000000000000000000000000000"),
        ),
        ("source_digest", json!("stale")),
        ("source_state_digest", json!("stale")),
        ("run_nonce", json!("00000000000000000000000000000000")),
        ("test_name", json!("missing::test_name")),
        ("claim_digest", json!("stale")),
        ("construction_cost", json!("main-tests=999")),
        ("execution_cost", json!("executed-tests=999")),
        (
            "test_stdout",
            json!("WORTH_UI_LEDGER_COUNTERS={\"P1-AFFINITY-01\":0}\n"),
        ),
    ] {
        let mut fixture = ProvedFixture::new();
        fixture.mutate_artifact(field, value);
        assert!(
            parse(&ledger_with_first_record(&fixture.record)).is_err(),
            "artifact mutant {field} must fail"
        );
    }
}

#[test]
fn phase_two_proof_requires_complete_phase_one_closure() {
    let fixture = ProvedFixture::new();
    let mut phase_two = bound_record(&fixture.record);
    set(&mut phase_two, "phase", "2");
    set(&mut phase_two, "requirement", "P2-APPLICATION-01");
    assert!(parse(&ledger_with_record_at(&phase_two, 20)).is_err());
    let mut lawful_progression = parse(&repository_document(LEDGER)).unwrap();
    for row in lawful_progression
        .values_mut()
        .filter(|row| row["phase"] == "1")
    {
        row.insert("result".to_owned(), "PROVED".to_owned());
        row.insert("final_source".to_owned(), "true".to_owned());
    }
    lawful_progression
        .get_mut("P2-APPLICATION-01")
        .unwrap()
        .insert("result".to_owned(), "PROVED".to_owned());
    assert!(validate_phase_progression(&lawful_progression).is_ok());
}

#[test]
fn future_proof_requires_every_predecessor_and_a_qualified_text_profile() {
    let mut rows = parse(&repository_document(LEDGER)).unwrap();
    rows.get_mut("P3-PREDECESSOR-01")
        .unwrap()
        .insert("result".to_owned(), "OPEN".to_owned());
    rows.get_mut("P3-DRAW-LIST-01")
        .unwrap()
        .insert("result".to_owned(), "PROVED".to_owned());
    assert!(validate_phase_progression(&rows).is_err());

    rows.get_mut("P3-PREDECESSOR-01")
        .unwrap()
        .insert("result".to_owned(), "PROVED".to_owned());
    assert!(validate_phase_progression(&rows).is_ok());

    for row in rows.values_mut().filter(|row| row["phase"] == "3") {
        row.insert("result".to_owned(), "PROVED".to_owned());
    }
    rows.get_mut("P4-BIDI-01")
        .unwrap()
        .insert("result".to_owned(), "PROVED".to_owned());
    rows.get_mut("P4-TEXT-PROFILE-01")
        .unwrap()
        .insert("result".to_owned(), "OPEN".to_owned());
    assert!(validate_phase_progression(&rows).is_err());

    let digest = super::source_digest::file_digest(
        "workspaces/worth-ui/profiles/worth-ui-global-text-v2/manifest.toml",
    )
    .unwrap();
    assert!(
        super::text_profile_gate::validate(super::text_profile_gate::ProfileClaim {
            result: "PROVED",
            identity: "worth-ui-global-text-v2",
            digest: &digest,
            platform_versions: super::claim_contract::TEXT_PLATFORM_VERSIONS,
        })
        .is_ok()
    );
    let qualification =
        super::text_profile_gate::validate(super::text_profile_gate::ProfileClaim {
            result: "PROVED",
            identity: "worth-ui-global-text-v2",
            digest: "0000000000000000000000000000000000000000000000000000000000000000",
            platform_versions: super::claim_contract::TEXT_PLATFORM_VERSIONS,
        });
    assert_eq!(
        qualification.unwrap_err(),
        "qualified text profile digest does not match canonical bytes"
    );
}

#[test]
fn open_future_claims_reject_identity_mutation_before_execution_exists() {
    let rows = parse(&repository_document(LEDGER)).unwrap();
    for (requirement, field, mutant) in [
        ("P3-DAMAGE-INDEX-01", "scenario_delta", "cooperative-case"),
        (
            "P3-TRANSACTION-01",
            "fault_injection_boundary",
            "before-effects",
        ),
        ("P3-PRODUCER-SLOPE-01", "structural_counters", "rows=open"),
        (
            "P4-FALLBACK-01",
            "mutation_control",
            "family=fallback;case=ascii",
        ),
        ("P4-TEXT-PROFILE-01", "font_profile_digest", "provisional"),
    ] {
        let mut row = rows[requirement].clone();
        row.insert(field.to_owned(), mutant.to_owned());
        assert!(
            validate_row(&row).is_err(),
            "{requirement} accepted mutated {field}"
        );
    }
}

#[test]
fn phase_closure_mode_rejects_open_rows_at_or_before_its_gate() {
    let mut rows = EXPECTED_REQUIREMENTS
        .iter()
        .map(|requirement| {
            let phase = requirement
                .strip_prefix('P')
                .and_then(|suffix| suffix.split('-').next())
                .expect("requirement owns a phase");
            (
                (*requirement).to_owned(),
                std::collections::BTreeMap::from([
                    ("requirement".to_owned(), (*requirement).to_owned()),
                    ("phase".to_owned(), phase.to_owned()),
                    ("result".to_owned(), "PROVED".to_owned()),
                    ("final_source".to_owned(), "true".to_owned()),
                ]),
            )
        })
        .collect();
    assert!(validate_phase_closure(&rows, 1).is_ok());
    assert!(validate_phase_closure(&rows, 2).is_ok());
    assert!(validate_phase_closure(&rows, 3).is_ok());
    assert!(validate_phase_closure(&rows, 4).is_ok());
    let phase_one = rows.get_mut("P1-AFFINITY-01").unwrap();
    phase_one.insert("result".to_owned(), "OPEN".to_owned());
    phase_one.insert("final_source".to_owned(), "false".to_owned());
    assert!(validate_phase_closure(&rows, 1).is_err());
    assert!(validate_phase_closure(&rows, 2).is_err());
    assert!(validate_phase_closure(&rows, 3).is_err());
    assert!(validate_phase_closure(&rows, 4).is_err());
    let phase_one = rows.get_mut("P1-AFFINITY-01").unwrap();
    phase_one.insert("result".to_owned(), "PROVED".to_owned());
    phase_one.insert("final_source".to_owned(), "true".to_owned());
    let phase_two = rows.get_mut("P2-APPLICATION-01").unwrap();
    phase_two.insert("result".to_owned(), "OPEN".to_owned());
    phase_two.insert("final_source".to_owned(), "false".to_owned());
    assert!(validate_phase_closure(&rows, 1).is_ok());
    assert!(validate_phase_closure(&rows, 2).is_err());
    assert!(validate_phase_closure(&rows, 3).is_err());
    assert!(validate_phase_closure(&rows, 4).is_err());
    let phase_two = rows.get_mut("P2-APPLICATION-01").unwrap();
    phase_two.insert("result".to_owned(), "PROVED".to_owned());
    phase_two.insert("final_source".to_owned(), "true".to_owned());
    let phase_three = rows.get_mut("P3-BASELINE-REPLAY-01").unwrap();
    phase_three.insert("result".to_owned(), "OPEN".to_owned());
    phase_three.insert("final_source".to_owned(), "false".to_owned());
    assert!(validate_phase_closure(&rows, 2).is_ok());
    assert!(validate_phase_closure(&rows, 3).is_err());
    assert!(validate_phase_closure(&rows, 4).is_err());
    let phase_three = rows.get_mut("P3-BASELINE-REPLAY-01").unwrap();
    phase_three.insert("result".to_owned(), "PROVED".to_owned());
    phase_three.insert("final_source".to_owned(), "true".to_owned());
    let phase_four = rows.get_mut("P4-BIDI-01").unwrap();
    phase_four.insert("result".to_owned(), "OPEN".to_owned());
    phase_four.insert("final_source".to_owned(), "false".to_owned());
    assert!(validate_phase_closure(&rows, 3).is_ok());
    assert!(validate_phase_closure(&rows, 4).is_err());
    println!(
        "WORTH_UI_LEDGER_MUTATION_CONTROLS={{\"P3-CLOSE-01\":\"open-requirement\",\"P4-CLOSE-01\":\"open-requirement\"}}"
    );
}

struct ProvedFixture {
    record: Vec<String>,
    artifact_identity: String,
}

impl ProvedFixture {
    fn new() -> Self {
        let sequence = FIXTURE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let artifact_identity = format!(
            "workspaces/worth-ui/target/milestone-3141-ledger-fixtures/{}-{sequence}.json",
            std::process::id()
        );
        let revision = result_artifact::current_revision().unwrap();
        let sources = format!("{PRODUCTION_SOURCE};{ORACLE_SOURCE}");
        let digest = source_digest::calculate(&sources).unwrap();
        let state_digest = source_digest::calculate_source_state(&revision).unwrap();
        let run_nonce = format!("{:032x}", sequence + 1);
        let evidence = ProvedEvidence {
            artifact: &artifact_identity,
            revision: &revision,
            digest: &digest,
            state_digest: &state_digest,
            run_nonce: &run_nonce,
            sources: &sources,
        };
        let mut record = proved_record(&evidence);
        let claim_digest = record_claim_digest(&record);
        write_artifact(&evidence, &claim_digest);
        let artifact_digest = source_digest::file_digest(&artifact_identity).unwrap();
        set(&mut record, "result_artifact_digest", &artifact_digest);
        Self {
            record,
            artifact_identity,
        }
    }

    fn mutate_artifact(&mut self, field: &str, value: Value) {
        let path = source_digest::repository_file(&self.artifact_identity).unwrap();
        let mut artifact: Value = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        artifact[field] = value;
        std::fs::write(path, serde_json::to_vec_pretty(&artifact).unwrap()).unwrap();
        let artifact_digest = source_digest::file_digest(&self.artifact_identity).unwrap();
        set(&mut self.record, "result_artifact_digest", &artifact_digest);
    }
}

impl Drop for ProvedFixture {
    fn drop(&mut self) {
        let _ =
            std::fs::remove_file(source_digest::repository_root().join(&self.artifact_identity));
    }
}

struct ProvedEvidence<'a> {
    artifact: &'a str,
    revision: &'a str,
    digest: &'a str,
    state_digest: &'a str,
    run_nonce: &'a str,
    sources: &'a str,
}
