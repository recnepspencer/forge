use std::collections::BTreeMap;
use std::process::{Command, Stdio};
use std::sync::{Mutex, OnceLock};

use serde_json::Value;

use super::command_binding::CommandBinding;
use super::result_artifact::{LedgerResult, SourceValidationPosture};
use super::source_digest;

pub(super) fn validate_ledger_fields(
    ledger: &LedgerResult<'_>,
    command: &CommandBinding,
) -> Result<(), String> {
    if ledger.matched_test_count != "1" || ledger.command_result != "passed" {
        return Err("proved command must pass after exactly one match".to_owned());
    }
    if ledger.artifact != command.artifact {
        return Err("retained result artifact is not command-bound".to_owned());
    }
    validate_source_revision(ledger.source_revision)?;
    if !is_lower_hex(ledger.source_digest, 64) || !is_lower_hex(ledger.source_state_digest, 64) {
        return Err("result artifact source metadata is invalid".to_owned());
    }
    match ledger.source_validation {
        SourceValidationPosture::CurrentSource => validate_current_source(ledger)?,
        SourceValidationPosture::HistoricalArtifactOnly => {}
    }
    if ledger.run_nonce.len() != 32 || !is_lower_hex(ledger.run_nonce, 32) {
        return Err("result artifact run nonce is invalid".to_owned());
    }
    if ledger.result_artifact_digest.len() != 64 || !is_lower_hex(ledger.result_artifact_digest, 64)
    {
        return Err("result artifact digest is invalid".to_owned());
    }
    Ok(())
}

fn validate_current_source(ledger: &LedgerResult<'_>) -> Result<(), String> {
    if ledger.source_digest != source_digest::calculate(ledger.source_identity)? {
        return Err("result artifact source digest is stale".to_owned());
    }
    if ledger.source_state_digest != source_digest::calculate_source_state(ledger.source_revision)?
    {
        return Err("result artifact source-state digest is stale".to_owned());
    }
    Ok(())
}

fn validate_source_revision(revision: &str) -> Result<(), String> {
    if !is_lower_hex(revision, 40) {
        return Err("result artifact source revision is invalid".to_owned());
    }
    static VALIDATED: OnceLock<Mutex<BTreeMap<String, Result<(), String>>>> = OnceLock::new();
    let validated = VALIDATED.get_or_init(|| Mutex::new(BTreeMap::new()));
    if let Some(result) = validated
        .lock()
        .map_err(|_| "source revision cache is poisoned".to_owned())?
        .get(revision)
        .cloned()
    {
        return result;
    }
    let status = Command::new("git")
        .args(["cat-file", "-e", &format!("{revision}^{{commit}}")])
        .current_dir(source_digest::repository_root())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|error| format!("cannot resolve source revision: {error}"))?;
    let result = status
        .success()
        .then_some(())
        .ok_or_else(|| "result artifact source revision is not repository lineage".to_owned());
    validated
        .lock()
        .map_err(|_| "source revision cache is poisoned".to_owned())?
        .insert(revision.to_owned(), result.clone());
    result
}

pub(super) fn read_artifact(identity: &str) -> Result<Value, String> {
    let path = source_digest::repository_file(identity)?;
    let content = std::fs::read_to_string(&path)
        .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
    serde_json::from_str(&content)
        .map_err(|error| format!("invalid result artifact {}: {error}", path.display()))
}

pub(super) fn current_revision() -> Result<String, String> {
    static REVISION: OnceLock<Result<String, String>> = OnceLock::new();
    REVISION.get_or_init(resolve_current_revision).clone()
}

fn resolve_current_revision() -> Result<String, String> {
    let output = Command::new("git")
        .args(["rev-parse", "--verify", "HEAD"])
        .current_dir(source_digest::repository_root())
        .output()
        .map_err(|error| format!("cannot resolve source revision: {error}"))?;
    let revision = String::from_utf8(output.stdout)
        .map_err(|error| format!("source revision is not UTF-8: {error}"))?
        .trim()
        .to_owned();
    if !output.status.success() || !is_lower_hex(&revision, 40) {
        return Err("source revision is not one exact Git commit".to_owned());
    }
    Ok(revision)
}

pub(super) fn cargo_command(command: &CommandBinding, list_only: bool) -> Vec<String> {
    let mut words = vec![
        "cargo".to_owned(),
        "test".to_owned(),
        "--manifest-path".to_owned(),
        "workspaces/worth-ui/Cargo.toml".to_owned(),
        "-p".to_owned(),
        command.package.clone(),
    ];
    words.push(format!("--{}", command.target_kind));
    if command.target_kind == "test" {
        words.push(command.target_name.clone());
    }
    for feature in &command.features {
        words.extend(["--features".to_owned(), feature.clone()]);
    }
    if list_only {
        words.extend(["--", "--list", "--format", "terse"].map(str::to_owned));
    } else {
        words.push(command.test_name.clone());
        words.extend(["--", "--exact", "--include-ignored", "--nocapture"].map(str::to_owned));
    }
    words
}

pub(super) fn ignored_list_command(command: &CommandBinding) -> Vec<String> {
    let mut words = cargo_command(command, true);
    words.truncate(words.len() - 4);
    words.extend(["--", "--ignored", "--list", "--format", "terse"].map(str::to_owned));
    words
}

pub(super) fn require_str(value: &Value, field: &str, expected: &str) -> Result<(), String> {
    (value.get(field).and_then(Value::as_str) == Some(expected))
        .then_some(())
        .ok_or_else(|| format!("result artifact has wrong {field}"))
}

pub(super) fn require_u64(value: &Value, field: &str, expected: u64) -> Result<(), String> {
    (value.get(field).and_then(Value::as_u64) == Some(expected))
        .then_some(())
        .ok_or_else(|| format!("result artifact has wrong {field}"))
}

pub(super) fn require_duration_within(
    value: &Value,
    field: &str,
    maximum: u64,
) -> Result<(), String> {
    value
        .get(field)
        .and_then(Value::as_u64)
        .filter(|duration| *duration > 0 && *duration <= maximum)
        .map(|_| ())
        .ok_or_else(|| format!("result artifact has out-of-budget {field}"))
}

pub(super) fn require_i64(value: &Value, field: &str, expected: i64) -> Result<(), String> {
    (value.get(field).and_then(Value::as_i64) == Some(expected))
        .then_some(())
        .ok_or_else(|| format!("result artifact has wrong {field}"))
}

pub(super) fn require_array(value: &Value, field: &str, expected: &[String]) -> Result<(), String> {
    let observed = value
        .get(field)
        .and_then(Value::as_array)
        .and_then(|items| items.iter().map(Value::as_str).collect::<Option<Vec<_>>>());
    (observed.as_deref()
        == Some(
            expected
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>()
                .as_slice(),
        ))
    .then_some(())
    .ok_or_else(|| format!("result artifact has wrong {field}"))
}

fn is_lower_hex(value: &str, length: usize) -> bool {
    value.len() == length
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}
