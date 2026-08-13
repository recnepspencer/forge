use serde_json::Value;

use super::command_binding::CommandBinding;
use super::result_artifact_binding::{read_artifact, require_i64, require_str, require_u64};

const HP02: &str = "P3-HP02-WORLD-01";
const MIXED_REQUIREMENT: &str = "P3-DELTA-SOURCE-01";
const MIXED_TEST: &str =
    "host_platform::mixed_carrier_successors_are_local_at_the_4096_command_ceiling";

pub(super) fn validate(
    artifact: &Value,
    command: &CommandBinding,
    source_revision: &str,
    source_state_digest: &str,
) -> Result<(), String> {
    if command.requirement != HP02 {
        return artifact
            .get("supporting_world")
            .is_none_or(Value::is_null)
            .then_some(())
            .ok_or_else(|| "non-HP-02 row carries a supporting world".to_owned());
    }
    let binding = artifact
        .get("supporting_world")
        .ok_or_else(|| "HP-02 omits its mixed-carrier supporting world".to_owned())?;
    let identity = binding
        .get("artifact")
        .and_then(Value::as_str)
        .ok_or_else(|| "HP-02 supporting world omits its artifact identity".to_owned())?;
    require_command_bound_source(command, identity)?;
    require_str(binding, "requirement", MIXED_REQUIREMENT)?;
    require_u64(binding, "worlds", 1)?;
    require_u64(binding, "presentations", 5)?;
    let expected_digest = super::source_digest::file_digest(identity)?;
    require_str(binding, "artifact_digest", &expected_digest)?;
    let supporting = read_artifact(identity)?;
    super::dependency_row::require_proved_artifact(
        MIXED_REQUIREMENT,
        identity,
        &expected_digest,
        &supporting,
    )?;
    validate_content(&supporting, source_revision, source_state_digest)
}

fn require_command_bound_source(command: &CommandBinding, identity: &str) -> Result<(), String> {
    command
        .sources
        .iter()
        .any(|source| source == identity)
        .then_some(())
        .ok_or_else(|| "HP-02 supporting world is not a command-bound source".to_owned())
}

fn validate_content(
    supporting: &Value,
    source_revision: &str,
    source_state_digest: &str,
) -> Result<(), String> {
    require_u64(supporting, "schema_version", 5)?;
    require_str(supporting, "requirement", MIXED_REQUIREMENT)?;
    require_str(supporting, "package", "worth-ui-certification")?;
    require_str(supporting, "target_kind", "test")?;
    require_str(supporting, "target_name", "application_contracts")?;
    require_str(supporting, "test_name", MIXED_TEST)?;
    require_u64(supporting, "matched_test_count", 1)?;
    require_u64(supporting, "declared_ignored_test_count", 1)?;
    (supporting.get("expected_declared_ignored") == Some(&Value::Bool(true)))
        .then_some(())
        .ok_or_else(|| "HP-02 supporting world has wrong ignored posture".to_owned())?;
    require_u64(supporting, "executed_test_count", 1)?;
    require_u64(supporting, "passed_test_count", 1)?;
    require_u64(supporting, "ignored_test_count", 0)?;
    require_str(supporting, "exit_posture", "passed")?;
    require_i64(supporting, "test_exit_code", 0)?;
    require_str(supporting, "source_revision", source_revision)?;
    require_str(supporting, "source_state_digest", source_state_digest)?;
    require_str(supporting, "structural_counter", "source-rows=1")?;
    require_str(
        supporting,
        "construction_cost",
        "main-tests=1;hostile-controls=1;product-processes=0;compile-sessions=0;courtroom-worlds=1",
    )?;
    require_str(
        supporting,
        "execution_cost",
        "executed-tests=2;presentations=5",
    )?;
    let stdout = supporting
        .get("test_stdout")
        .and_then(Value::as_str)
        .ok_or_else(|| "HP-02 supporting world omits test stdout".to_owned())?;
    stdout
        .lines()
        .any(|line| line == "WORTH_UI_LEDGER_WORLD=1")
        .then_some(())
        .ok_or_else(|| "HP-02 supporting world omits its executed world observation".to_owned())
}

#[test]
fn hp02_support_rejects_substituted_world_identity_and_incomplete_execution() {
    let mut supporting = fixture();
    validate_content(&supporting, "revision", "state").unwrap();
    for (field, value) in [
        ("requirement", Value::from("P3-HP02-WORLD-01")),
        ("test_name", Value::from("cooperative_substitute")),
        ("passed_test_count", Value::from(0)),
        ("structural_counter", Value::from("source-rows=0")),
        ("test_stdout", Value::from("test result: ok")),
    ] {
        let original = supporting[field].clone();
        supporting[field] = value;
        assert!(validate_content(&supporting, "revision", "state").is_err());
        supporting[field] = original;
    }
}

#[test]
fn hp02_support_must_name_the_exact_command_bound_artifact() {
    let fresh = "workspaces/worth-ui/target/worth-ui-3141-verify-test/mixed.json";
    let command = CommandBinding {
        shared_main: false,
        requirement: HP02.to_owned(),
        package: "worth-ui-platform-pulse".to_owned(),
        target_kind: "test".to_owned(),
        target_name: "executable_world".to_owned(),
        features: vec!["executable-world".to_owned()],
        test_name: "courtroom::native_phase3::world".to_owned(),
        sources: vec![fresh.to_owned()],
        artifact: "result.json".to_owned(),
        control: None,
    };
    require_command_bound_source(&command, fresh).unwrap();
    assert!(require_command_bound_source(&command, "cooperative.json").is_err());
}

fn fixture() -> Value {
    serde_json::json!({
        "schema_version": 5,
        "requirement": MIXED_REQUIREMENT,
        "package": "worth-ui-certification",
        "target_kind": "test",
        "target_name": "application_contracts",
        "test_name": MIXED_TEST,
        "matched_test_count": 1,
        "declared_ignored_test_count": 1,
        "expected_declared_ignored": true,
        "executed_test_count": 1,
        "passed_test_count": 1,
        "ignored_test_count": 0,
        "exit_posture": "passed",
        "test_exit_code": 0,
        "source_revision": "revision",
        "source_state_digest": "state",
        "structural_counter": "source-rows=1",
        "construction_cost": "main-tests=1;hostile-controls=1;product-processes=0;compile-sessions=0;courtroom-worlds=1",
        "execution_cost": "executed-tests=2;presentations=5",
        "test_stdout": "WORTH_UI_LEDGER_WORLD=1\n"
    })
}
