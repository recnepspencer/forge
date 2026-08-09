use serde_json::Value;

use super::command_binding::ControlBinding;
use super::result_artifact_binding::{require_array, require_i64, require_str, require_u64};

pub(super) fn validate(artifact: &Value, expected: Option<&ControlBinding>) -> Result<(), String> {
    let observed = artifact.get("hostile_control");
    let Some(expected) = expected else {
        return observed
            .is_some_and(Value::is_null)
            .then_some(())
            .ok_or_else(|| "Phase 1 artifact unexpectedly carries a hostile control".to_owned());
    };
    let control = observed
        .filter(|value| value.is_object())
        .ok_or_else(|| "Phase 2 artifact omits its hostile control".to_owned())?;
    require_str(control, "package", &expected.package)?;
    require_str(control, "target_kind", &expected.target_kind)?;
    require_str(control, "target_name", &expected.target_name)?;
    require_array(control, "features", &expected.features)?;
    require_str(control, "test_name", &expected.test_name)?;
    require_u64(control, "matched_test_count", 1)?;
    require_u64(control, "executed_test_count", 1)?;
    require_u64(control, "passed_test_count", 1)?;
    require_u64(control, "ignored_test_count", 0)?;
    require_str(control, "exit_posture", "passed")?;
    require_i64(control, "list_exit_code", 0)?;
    require_i64(control, "test_exit_code", 0)?;
    require_u64(control, "test_budget_ms", 10_000)?;
    require_duration(control, "list_duration_ms", 300_000)?;
    require_duration(control, "test_duration_ms", 10_000)?;
    require_array(control, "list_command", &cargo_command(expected, true))?;
    require_array(control, "test_command", &cargo_command(expected, false))
}

fn require_duration(value: &Value, field: &str, maximum: u64) -> Result<(), String> {
    value
        .get(field)
        .and_then(Value::as_u64)
        .filter(|duration| *duration > 0 && *duration <= maximum)
        .map(|_| ())
        .ok_or_else(|| format!("hostile control has out-of-budget {field}"))
}

pub(super) fn cargo_command(control: &ControlBinding, list_only: bool) -> Vec<String> {
    let mut words = vec![
        "cargo".to_owned(),
        "test".to_owned(),
        "--manifest-path".to_owned(),
        "workspaces/worth-ui/Cargo.toml".to_owned(),
        "-p".to_owned(),
        control.package.clone(),
        format!("--{}", control.target_kind),
    ];
    if control.target_kind == "test" {
        words.push(control.target_name.clone());
    }
    for feature in &control.features {
        words.extend(["--features".to_owned(), feature.clone()]);
    }
    if list_only {
        words.extend(["--", "--list", "--format", "terse"].map(str::to_owned));
    } else {
        words.push(control.test_name.clone());
        words.extend(["--", "--exact", "--nocapture"].map(str::to_owned));
    }
    words
}
