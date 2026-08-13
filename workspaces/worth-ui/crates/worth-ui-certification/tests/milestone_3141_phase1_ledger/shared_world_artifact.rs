use serde_json::Value;

use super::command_binding::CommandBinding;
use super::result_artifact_binding::{read_artifact, require_i64, require_str, require_u64};

struct SharedWorldBinding<'a> {
    source_revision: &'a str,
    source_state_digest: &'a str,
    artifact_digest: &'a str,
}

pub(super) fn validate(
    artifact: &Value,
    command: &CommandBinding,
    source_revision: &str,
    source_state_digest: &str,
) -> Result<(), String> {
    let shared_requirement = shared_requirement(&command.requirement)?;
    let identity = artifact
        .get("shared_main_artifact")
        .and_then(Value::as_str)
        .ok_or_else(|| "shared row omits its native world artifact".to_owned())?;
    require_str(artifact, "shared_main_requirement", shared_requirement)?;
    let expected_digest = super::source_digest::file_digest(identity)?;
    let shared = read_artifact(identity)?;
    super::dependency_row::require_proved_artifact(
        shared_requirement,
        identity,
        &expected_digest,
        &shared,
    )?;
    validate_content(
        artifact,
        &shared,
        command,
        SharedWorldBinding {
            source_revision,
            source_state_digest,
            artifact_digest: &expected_digest,
        },
    )
}

fn validate_content(
    artifact: &Value,
    shared: &Value,
    command: &CommandBinding,
    binding: SharedWorldBinding<'_>,
) -> Result<(), String> {
    let shared_requirement = shared_requirement(&command.requirement)?;
    let identity = artifact
        .get("shared_main_artifact")
        .and_then(Value::as_str)
        .ok_or_else(|| "shared row omits its native world artifact".to_owned())?;
    if !command.sources.iter().any(|source| source == identity) {
        return Err("shared native world is not a command-bound source".to_owned());
    }
    require_str(
        artifact,
        "shared_main_artifact_digest",
        binding.artifact_digest,
    )?;
    require_u64(&shared, "schema_version", 5)?;
    require_str(&shared, "requirement", shared_requirement)?;
    require_str(&shared, "exit_posture", "passed")?;
    require_u64(&shared, "executed_test_count", 1)?;
    require_u64(&shared, "passed_test_count", 1)?;
    require_u64(&shared, "ignored_test_count", 0)?;
    require_i64(&shared, "test_exit_code", 0)?;
    require_str(&shared, "source_revision", binding.source_revision)?;
    require_str(&shared, "source_state_digest", binding.source_state_digest)?;
    for field in [
        "list_command",
        "ignored_list_command",
        "test_command",
        "list_stdout",
        "list_stderr",
        "ignored_list_stdout",
        "ignored_list_stderr",
        "test_stdout",
        "test_stderr",
        "boundary_observation",
    ] {
        if artifact.get(field) != shared.get(field) {
            return Err(format!("shared row substituted native world {field}"));
        }
    }
    Ok(())
}

fn shared_requirement(requirement: &str) -> Result<&'static str, String> {
    if requirement == "P1-HEADLESS-COST-01" {
        Ok("P1-WORLDS-01")
    } else if matches!(requirement, "P3-HEADLESS-COST-01" | "P3-PRODUCER-SLOPE-01") {
        Ok("P3-DELTA-SOURCE-01")
    } else if requirement.starts_with("P2-") && requirement != "P2-WORLD-01" {
        Ok("P2-WORLD-01")
    } else if matches!(
        requirement,
        "P3-BASELINE-REPLAY-01"
            | "P3-DAMAGE-REPLAY-01"
            | "P3-DRAW-LIST-01"
            | "P3-PHYSICAL-AMPLIFICATION-01"
            | "P3-TRANSACTION-01"
            | "P3-UNCHANGED-01"
    ) {
        Ok("P3-HP02-WORLD-01")
    } else {
        Err("requirement has no governed shared world".to_owned())
    }
}

#[test]
fn shared_native_world_rejects_digest_observation_and_source_substitution() {
    let mut command = CommandBinding {
        shared_main: true,
        requirement: "P2-CLOSE-01".to_owned(),
        package: "worth-ui-platform-pulse".to_owned(),
        target_kind: "test".to_owned(),
        target_name: "executable_world".to_owned(),
        features: vec!["executable-world".to_owned()],
        test_name: "courtroom::native_phase2::world".to_owned(),
        sources: vec!["world.json".to_owned()],
        artifact: "row.json".to_owned(),
        control: None,
    };
    let shared = fixture_shared();
    let mut row = shared.clone();
    row["shared_main_artifact"] = Value::from("world.json");
    row["shared_main_artifact_digest"] = Value::from("digest");
    row["shared_main_requirement"] = Value::from("P2-WORLD-01");
    let binding = || SharedWorldBinding {
        source_revision: "revision",
        source_state_digest: "state",
        artifact_digest: "digest",
    };
    validate_content(&row, &shared, &command, binding()).unwrap();
    for (field, value) in [
        ("shared_main_artifact_digest", Value::from("substitute")),
        ("boundary_observation", serde_json::json!({"world": 2})),
        ("test_stdout", Value::from("substituted output")),
    ] {
        let mut mutant = row.clone();
        mutant[field] = value;
        assert!(validate_content(&mutant, &shared, &command, binding()).is_err());
    }
    command.sources.clear();
    assert!(validate_content(&row, &shared, &command, binding()).is_err());
}

#[test]
fn shared_world_entrypoint_rejects_an_open_producer_before_reuse() {
    let identity = "workspaces/worth-ui/target/milestone-3141-open-producer.json";
    let destination = super::source_digest::repository_root().join(identity);
    let mut shared = fixture_shared();
    shared["requirement"] = Value::from("P3-DELTA-SOURCE-01");
    std::fs::write(&destination, serde_json::to_vec(&shared).unwrap()).unwrap();
    let digest = super::source_digest::file_digest(identity).unwrap();
    let mut row = shared.clone();
    row["shared_main_artifact"] = Value::from(identity);
    row["shared_main_artifact_digest"] = Value::from(digest);
    row["shared_main_requirement"] = Value::from("P3-DELTA-SOURCE-01");
    let command = CommandBinding {
        shared_main: true,
        requirement: "P3-HEADLESS-COST-01".to_owned(),
        package: "worth-ui-certification".to_owned(),
        target_kind: "test".to_owned(),
        target_name: "application_contracts".to_owned(),
        features: Vec::new(),
        test_name: "host_platform::mixed_carrier_successors_are_local_at_the_4096_command_ceiling"
            .to_owned(),
        sources: vec![identity.to_owned()],
        artifact: "row.json".to_owned(),
        control: None,
    };
    let ledger = format!(
        "requirement,result,final_source,retained_result_artifact,result_artifact_digest\nP3-DELTA-SOURCE-01,OPEN,false,{identity},unused\n"
    );
    let _active = super::dependency_row::install(&ledger);
    let denial = validate(&row, &command, "revision", "state").unwrap_err();
    assert!(denial.contains("wrong result"));
    std::fs::remove_file(destination).unwrap();
}

fn fixture_shared() -> Value {
    serde_json::json!({
        "schema_version": 5,
        "requirement": "P2-WORLD-01",
        "exit_posture": "passed",
        "executed_test_count": 1,
        "passed_test_count": 1,
        "ignored_test_count": 0,
        "test_exit_code": 0,
        "source_revision": "revision",
        "source_state_digest": "state",
        "list_command": ["list"],
        "ignored_list_command": ["ignored-list"],
        "test_command": ["test"],
        "list_stdout": "listed",
        "list_stderr": "",
        "ignored_list_stdout": "ignored",
        "ignored_list_stderr": "",
        "test_stdout": "passed",
        "test_stderr": "",
        "boundary_observation": {"world": 1}
    })
}
