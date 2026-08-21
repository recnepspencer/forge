use serde_json::Value;

use super::{require_hex, require_str};
use crate::milestone_3141_phase1_ledger::{runner_artifact_authentication, source_digest};

pub(super) fn validate(row: &Value) -> Result<(), String> {
    let Some(reuse) = row.get("causal_reuse") else {
        return Ok(());
    };
    let receipt_keys = validate_receipts(row)?;
    require_str(reuse, "schema", "worth-ui-ledger-causal-reuse-v1")?;
    for field in ["predecessor_artifact_sha256", "predecessor_source_digest"] {
        require_hex(reuse, field, 64)?;
    }
    require_hex(reuse, "predecessor_run_nonce", 32)?;
    require_str(
        reuse,
        "claim_digest",
        row["claim_digest"]
            .as_str()
            .ok_or_else(|| "predecessor row omits claim digest".to_owned())?,
    )?;
    require_str(
        reuse,
        "exact_command",
        row["executed_exact_command"]
            .as_str()
            .ok_or_else(|| "predecessor row omits executed command".to_owned())?,
    )?;
    let reused_keys = reuse["execution_receipt_keys"]
        .as_array()
        .ok_or_else(|| "predecessor causal reuse omits execution receipts".to_owned())?
        .iter()
        .map(Value::as_str)
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| "predecessor causal reuse has invalid execution receipt".to_owned())?;
    let mut reused_keys = reused_keys;
    reused_keys.sort_unstable();
    (receipt_keys == reused_keys)
        .then_some(())
        .ok_or_else(|| "predecessor causal reuse substituted an execution receipt".to_owned())
}

fn validate_receipts(row: &Value) -> Result<Vec<&str>, String> {
    let receipts = row["execution_receipts"]
        .as_array()
        .filter(|receipts| !receipts.is_empty())
        .ok_or_else(|| "predecessor row omits execution receipts".to_owned())?;
    let mut keys = Vec::new();
    for receipt in receipts {
        let key = receipt["key"]
            .as_str()
            .ok_or_else(|| "predecessor row has invalid execution receipt".to_owned())?;
        require_hex(receipt, "key", 64)?;
        let identity = format!(
            "_docs/worth-ui/milestone-3.14.1-evidence/executions/{}/{}.json",
            &key[..2],
            key
        );
        let envelope: Value = serde_json::from_str(
            &std::fs::read_to_string(source_digest::repository_file(&identity)?)
                .map_err(|error| format!("cannot read predecessor receipt: {error}"))?,
        )
        .map_err(|error| format!("invalid predecessor receipt: {error}"))?;
        let record = envelope
            .get("record")
            .ok_or_else(|| "predecessor receipt omits its record".to_owned())?;
        let tag = envelope["runner_authentication"]
            .as_str()
            .ok_or_else(|| "predecessor receipt omits runner authentication".to_owned())?;
        runner_artifact_authentication::validate_tagged(record, tag)?;
        require_str(
            &envelope,
            "receipt_sha256",
            &runner_artifact_authentication::canonical_digest(record),
        )?;
        require_str(record, "key", key)?;
        require_str(
            receipt,
            "command_sha256",
            &runner_artifact_authentication::canonical_digest(&record["command"]),
        )?;
        if record["duration_ms"] != receipt["duration_ms"]
            || record["returncode"].as_u64() != Some(0)
        {
            return Err("predecessor receipt differs from its execution".to_owned());
        }
        let binding = serde_json::json!({
            "schema": record["schema"],
            "command": record["command"],
            "source_revision": record["source_revision"],
            "source_state_digest": record["source_state_digest"],
            "artifact_bindings": record["artifact_bindings"],
        });
        if runner_artifact_authentication::canonical_digest(&binding) != key {
            return Err("predecessor receipt identity is substituted".to_owned());
        }
        keys.push(key);
    }
    keys.sort_unstable();
    Ok(keys)
}
