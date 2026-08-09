use std::process::Command;
use std::sync::OnceLock;

use serde_json::Value;

use super::command_binding::CommandBinding;
use super::source_digest;

pub(super) struct LedgerResult<'a> {
    pub(super) matched_test_count: &'a str,
    pub(super) command_result: &'a str,
    pub(super) artifact: &'a str,
    pub(super) source_revision: &'a str,
    pub(super) source_digest: &'a str,
    pub(super) source_state_digest: &'a str,
    pub(super) run_nonce: &'a str,
    pub(super) source_identity: &'a str,
    pub(super) result_artifact_digest: &'a str,
    pub(super) claim_digest: &'a str,
}

pub(super) fn validate(ledger: LedgerResult<'_>, command: &CommandBinding) -> Result<(), String> {
    validate_ledger_fields(&ledger, command)?;
    let artifact = read_artifact(ledger.artifact)?;
    require_u64(&artifact, "schema_version", 3)?;
    require_str(&artifact, "package", &command.package)?;
    require_str(&artifact, "target_kind", &command.target_kind)?;
    require_str(&artifact, "target_name", &command.target_name)?;
    require_str(&artifact, "test_name", &command.test_name)?;
    require_str(&artifact, "requirement", &command.requirement)?;
    require_str(&artifact, "claim_digest", ledger.claim_digest)?;
    require_u64(&artifact, "matched_test_count", 1)?;
    require_u64(&artifact, "executed_test_count", 1)?;
    require_u64(&artifact, "passed_test_count", 1)?;
    require_u64(&artifact, "ignored_test_count", 0)?;
    require_str(&artifact, "exit_posture", "passed")?;
    require_i64(&artifact, "list_exit_code", 0)?;
    require_i64(&artifact, "test_exit_code", 0)?;
    require_str(&artifact, "source_revision", ledger.source_revision)?;
    require_str(&artifact, "source_digest", ledger.source_digest)?;
    require_str(&artifact, "source_state_digest", ledger.source_state_digest)?;
    require_str(&artifact, "run_nonce", ledger.run_nonce)?;
    require_array(&artifact, "source_identity", &command.sources)?;
    require_array(&artifact, "list_command", &cargo_command(command, true))?;
    require_array(&artifact, "test_command", &cargo_command(command, false))?;
    if source_digest::file_digest(ledger.artifact)? != ledger.result_artifact_digest {
        return Err("result artifact digest is stale".to_owned());
    }
    Ok(())
}

fn validate_ledger_fields(
    ledger: &LedgerResult<'_>,
    command: &CommandBinding,
) -> Result<(), String> {
    if ledger.matched_test_count != "1" || ledger.command_result != "passed" {
        return Err("proved command must pass after exactly one match".to_owned());
    }
    if ledger.artifact != command.artifact {
        return Err("retained result artifact is not command-bound".to_owned());
    }
    if ledger.source_revision != current_revision()? {
        return Err("result artifact does not name the current source revision".to_owned());
    }
    if ledger.source_digest != source_digest::calculate(ledger.source_identity)? {
        return Err("result artifact source digest is stale".to_owned());
    }
    if ledger.source_state_digest != source_digest::calculate_source_state(ledger.source_revision)?
    {
        return Err("result artifact source-state digest is stale".to_owned());
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

fn read_artifact(identity: &str) -> Result<Value, String> {
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

fn cargo_command(command: &CommandBinding, list_only: bool) -> Vec<String> {
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
    if list_only {
        words.extend(["--", "--list", "--format", "terse"].map(str::to_owned));
    } else {
        words.push(command.test_name.clone());
        words.extend(["--", "--exact", "--include-ignored", "--nocapture"].map(str::to_owned));
    }
    words
}

fn require_str(value: &Value, field: &str, expected: &str) -> Result<(), String> {
    (value.get(field).and_then(Value::as_str) == Some(expected))
        .then_some(())
        .ok_or_else(|| format!("result artifact has wrong {field}"))
}

fn require_u64(value: &Value, field: &str, expected: u64) -> Result<(), String> {
    (value.get(field).and_then(Value::as_u64) == Some(expected))
        .then_some(())
        .ok_or_else(|| format!("result artifact has wrong {field}"))
}

fn require_i64(value: &Value, field: &str, expected: i64) -> Result<(), String> {
    (value.get(field).and_then(Value::as_i64) == Some(expected))
        .then_some(())
        .ok_or_else(|| format!("result artifact has wrong {field}"))
}

fn require_array(value: &Value, field: &str, expected: &[String]) -> Result<(), String> {
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
