use std::collections::BTreeSet;
use std::sync::atomic::{AtomicU64, Ordering};

use serde_json::{json, Value};

#[path = "ledger_mutants.rs"]
mod ledger_mutants;

use ledger_mutants::{duplicate_first_data_row, remove_first_data_row, swap_first_data_rows};

use super::{
    parse, repository_document, result_artifact, source_digest, validate_phase_progression,
    EXPECTED_REQUIREMENTS, HEADER, LEDGER,
};

const PRODUCTION_SOURCE: &str =
    "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/authority/work.rs";
const ORACLE_SOURCE: &str =
    "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/work_producer_tests.rs";
const TEST_NAME: &str = "mounting::presentation::work_producer_tests::one_replacement_carries_one_change_and_exact_predecessor_successor_damage";

static FIXTURE_SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[test]
fn milestone_ledger_has_exact_schema_inventory_and_honest_posture() {
    let rows = parse(&repository_document(LEDGER)).expect("the milestone ledger should parse");
    let observed = rows.keys().map(String::as_str).collect::<BTreeSet<_>>();
    assert_eq!(observed, EXPECTED_REQUIREMENTS.into_iter().collect());
    assert!(rows.values().all(|row| row["result"] == "OPEN"));
    assert!(rows.values().all(|row| row["final_source"] == "false"));
}

#[test]
fn validator_rejects_inventory_schema_and_premature_proof_mutants() {
    let ledger = repository_document(LEDGER);
    assert!(parse(&ledger).is_ok());
    assert!(parse(&remove_first_data_row(&ledger)).is_err());
    assert!(parse(&duplicate_first_data_row(&ledger)).is_err());
    assert!(parse(&swap_first_data_rows(&ledger)).is_err());
    assert!(parse(&ledger.replacen("phase,requirement", "requirement,phase", 1)).is_err());
    assert!(parse(&ledger.replacen(",OPEN,,false", ",PROVED,,false", 1)).is_err());
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
    assert!(super::validate_named_entry(&format!("{ORACLE_SOURCE}::missing_oracle")).is_err());
    let mut zero_match = bound_record(&fixture.record);
    let command = zero_match[column("exact_command")].replace(TEST_NAME, "missing::test_name");
    set(&mut zero_match, "exact_command", &command);
    assert!(parse(&ledger_with_first_record(&zero_match)).is_err());
}

#[test]
fn retained_artifact_rejects_zero_match_failure_and_stale_source_mutants() {
    for (field, value) in [
        ("matched_test_count", json!(0)),
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

fn proved_record(evidence: &ProvedEvidence<'_>) -> Vec<String> {
    HEADER
        .iter()
        .map(|name| proved_value(name, evidence).to_owned())
        .collect()
}

fn proved_value<'a>(name: &str, evidence: &'a ProvedEvidence<'a>) -> &'a str {
    match name {
        "phase" => "1",
        "requirement" => "P1-AFFINITY-01",
        "owner" => "worth-ui-runtime",
        "production_boundary" => "initial delta unchanged affinity",
        "world_identity" => "mounted-presentation-world",
        "world_version" => "1",
        "proof_kind" => "runtime-model",
        "evidence_schema" => super::requirement_contract::EVIDENCE_SCHEMA,
        "baseline_digest" => "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "scenario_delta" => "initial-to-delta",
        "generated_seed" => "seed=1",
        "authority_provenance" => "worth_ui_runtime::mounting::presentation",
        "production_entry" => "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/authority/work.rs::issue_delta",
        "independent_oracle" => "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/work_producer_tests.rs::one_replacement_carries_one_change_and_exact_predecessor_successor_damage",
        "mutation_control" => "affinity-drop-predecessor",
        "fault_injection_boundary" => "before-effects",
        "retained_failure_artifact" => "typed-denial:stale-predecessor",
        "teardown_result" => "terminal",
        "construction_cost" | "execution_cost" => "rows=1",
        "structural_counters" => "work=1",
        "exact_command" => "dynamic",
        "matched_test_count" => "1",
        "command_result" => "passed",
        "retained_result_artifact" => evidence.artifact,
        "source_revision" => evidence.revision,
        "source_digest" => evidence.digest,
        "source_state_digest" => evidence.state_digest,
        "run_nonce" => evidence.run_nonce,
        "source_identity" => evidence.sources,
        "font_profile_identity" => "worth-ui-body-default-v1",
        "font_profile_digest" => super::requirement_contract::FONT_PROFILE_DIGEST,
        "native_profile_identity" => "worth-ui-windows-dx12-v1",
        "native_profile_digest" => super::requirement_contract::NATIVE_PROFILE_DIGEST,
        "platform_versions" => "protocol=4",
        "presented_source_readback" | "client_area_observation" => "not-applicable",
        "result" => "PROVED",
        "reopen_lineage" => "none",
        "final_source" => "true",
        "result_artifact_digest" => {
            "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc"
        }
        _ => unreachable!("every proved ledger column has explicit fixture evidence"),
    }
}

fn write_artifact(evidence: &ProvedEvidence<'_>, claim_digest: &str) {
    let path = source_digest::repository_root().join(evidence.artifact);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let artifact = json!({
        "schema_version": 3,
        "requirement": "P1-AFFINITY-01",
        "claim_digest": claim_digest,
        "package": "worth-ui-runtime",
        "target_kind": "lib",
        "target_name": "lib",
        "test_name": TEST_NAME,
        "matched_test_count": 1,
        "executed_test_count": 1,
        "passed_test_count": 1,
        "ignored_test_count": 0,
        "exit_posture": "passed",
        "list_exit_code": 0,
        "test_exit_code": 0,
        "source_revision": evidence.revision,
        "source_digest": evidence.digest,
        "source_state_digest": evidence.state_digest,
        "run_nonce": evidence.run_nonce,
        "source_identity": [PRODUCTION_SOURCE, ORACLE_SOURCE],
        "list_command": cargo_words(true),
        "test_command": cargo_words(false),
    });
    std::fs::write(path, serde_json::to_vec_pretty(&artifact).unwrap()).unwrap();
}

fn cargo_words(list_only: bool) -> Vec<String> {
    let mut words =
        "cargo test --manifest-path workspaces/worth-ui/Cargo.toml -p worth-ui-runtime --lib"
            .split_whitespace()
            .map(str::to_owned)
            .collect::<Vec<_>>();
    if list_only {
        words.extend(["--", "--list", "--format", "terse"].map(str::to_owned));
    } else {
        words.push(TEST_NAME.to_owned());
        words.extend(["--", "--exact", "--include-ignored", "--nocapture"].map(str::to_owned));
    }
    words
}

fn assert_valid_fixture(record: &[String]) {
    parse(&ledger_with_first_record(record)).expect("proved fixture must satisfy every validator");
}

fn record_claim_digest(record: &[String]) -> String {
    let row = HEADER
        .iter()
        .zip(record)
        .map(|(field, value)| ((*field).to_owned(), value.clone()))
        .collect();
    super::claim_digest::calculate(&row)
}

fn ledger_with_first_record(record: &[String]) -> String {
    let bound = bound_record(record);
    ledger_with_record_at(&bound, 0)
}

fn bound_record(record: &[String]) -> Vec<String> {
    let mut bound = record.to_vec();
    if bound[column("exact_command")] != "dynamic" {
        return bound;
    }
    let command = format!(
        "python scripts/ci/run_worth_ui_ledger_test.py --manifest-path workspaces/worth-ui/Cargo.toml --package worth-ui-runtime --lib --test-name {TEST_NAME} --requirement P1-AFFINITY-01 --source {PRODUCTION_SOURCE} --source {ORACLE_SOURCE} --artifact {}",
        bound[column("retained_result_artifact")]
    );
    set(&mut bound, "exact_command", &command);
    bound
}

fn ledger_with_record_at(record: &[String], target: usize) -> String {
    let mut writer = csv::Writer::from_writer(Vec::new());
    writer.write_record(HEADER).unwrap();
    let ledger = repository_document(LEDGER);
    let mut reader = csv::Reader::from_reader(ledger.as_bytes());
    for (index, original) in reader.records().enumerate() {
        if index == target {
            writer.write_record(record).unwrap();
        } else {
            writer.write_record(&original.unwrap()).unwrap();
        }
    }
    String::from_utf8(writer.into_inner().unwrap()).unwrap()
}

fn set(record: &mut [String], name: &str, value: &str) {
    record[column(name)] = value.to_owned();
}

fn column(name: &str) -> usize {
    HEADER.iter().position(|column| *column == name).unwrap()
}
