use super::*;
use proved_fixture::ProvedEvidence;

pub(super) fn proved_record(evidence: &ProvedEvidence<'_>) -> Vec<String> {
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
        "evidence_schema" => super::super::requirement_contract::EVIDENCE_SCHEMA,
        "baseline_digest" => "1500dfdd4dae40ae38aab3f951756d4d780b70792fea1d70c29e2917c9abca16",
        "scenario_delta" => "stale-predecessor",
        "generated_seed" => "not-applicable",
        "authority_provenance" => "worth_ui_runtime::mounting::presentation",
        "production_entry" => "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/authority/work.rs::issue_delta",
        "independent_oracle" => "workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/work_producer_tests.rs::one_replacement_carries_one_change_and_exact_predecessor_successor_damage",
        "mutation_control" => "family=affinity;case=stale-predecessor",
        "fault_injection_boundary" => "not-applicable",
        "retained_failure_artifact" => evidence.artifact,
        "teardown_result" => "terminal",
        "construction_cost" => "main-tests=1;hostile-controls=0;product-processes=0;compile-sessions=0;courtroom-worlds=0",
        "execution_cost" => "executed-tests=1;presentations=0",
        "structural_counters" => "work=3",
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
        "font_profile_digest" => super::super::requirement_contract::FONT_PROFILE_DIGEST,
        "native_profile_identity" => "worth-ui-windows-dx12-v1",
        "native_profile_digest" => super::super::requirement_contract::NATIVE_PROFILE_DIGEST,
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

pub(super) fn write_artifact(evidence: &ProvedEvidence<'_>, claim_digest: &str) {
    let path = source_digest::repository_root().join(evidence.artifact);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let artifact = json!({
        "schema_version": 5,
        "requirement": "P1-AFFINITY-01",
        "claim_digest": claim_digest,
        "package": "worth-ui-runtime",
        "target_kind": "lib",
        "target_name": "lib",
        "features": [],
        "test_name": TEST_NAME,
        "matched_test_count": 1,
        "declared_ignored_test_count": 0,
        "expected_declared_ignored": false,
        "executed_test_count": 1,
        "passed_test_count": 1,
        "ignored_test_count": 0,
        "exit_posture": "passed",
        "list_exit_code": 0,
        "test_exit_code": 0,
        "list_duration_ms": 1,
        "ignored_list_duration_ms": 1,
        "test_duration_ms": 1,
        "test_budget_ms": 60_000,
        "structural_counter": "work=3",
        "construction_cost": "main-tests=1;hostile-controls=0;product-processes=0;compile-sessions=0;courtroom-worlds=0",
        "execution_cost": "executed-tests=1;presentations=0",
        "source_revision": evidence.revision,
        "source_digest": evidence.digest,
        "source_state_digest": evidence.state_digest,
        "run_nonce": evidence.run_nonce,
        "source_identity": [PRODUCTION_SOURCE, ORACLE_SOURCE],
        "list_command": cargo_words(true),
        "ignored_list_command": ignored_cargo_words(),
        "test_command": cargo_words(false),
        "test_stdout": "WORTH_UI_LEDGER_COUNTERS={\"P1-AFFINITY-01\":3}\n",
        "hostile_control": Value::Null,
    });
    std::fs::write(path, serde_json::to_vec_pretty(&artifact).unwrap()).unwrap();
}

fn ignored_cargo_words() -> Vec<String> {
    let mut words = cargo_words(true);
    words.truncate(words.len() - 4);
    words.extend(["--", "--ignored", "--list", "--format", "terse"].map(str::to_owned));
    words
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

pub(super) fn assert_valid_fixture(record: &[String]) {
    parse(&ledger_with_first_record(record)).expect("proved fixture must satisfy every validator");
}

pub(super) fn record_claim_digest(record: &[String]) -> String {
    let row = HEADER
        .iter()
        .zip(record)
        .map(|(field, value)| ((*field).to_owned(), value.clone()))
        .collect();
    super::super::claim_digest::calculate(&row)
}

pub(super) fn ledger_with_first_record(record: &[String]) -> String {
    let bound = bound_record(record);
    ledger_with_record_at(&bound, 0)
}

pub(super) fn bound_record(record: &[String]) -> Vec<String> {
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

pub(super) fn ledger_with_record_at(record: &[String], target: usize) -> String {
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

pub(super) fn set(record: &mut [String], name: &str, value: &str) {
    record[column(name)] = value.to_owned();
}

pub(super) fn column(name: &str) -> usize {
    HEADER.iter().position(|column| *column == name).unwrap()
}
