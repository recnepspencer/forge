use std::collections::BTreeSet;

use serde_json::Value;

use super::{execution_observation, require_hex, require_str, require_string};

pub(super) fn validate(row: &Value) -> Result<(), String> {
    let Some(reuse) = row.get("causal_reuse") else {
        return Ok(());
    };
    require_str(reuse, "schema", "worth-ui-ledger-causal-reuse-v2")?;
    for field in ["predecessor_artifact_sha256", "predecessor_source_digest"] {
        require_hex(reuse, field, 64)?;
    }
    require_hex(reuse, "predecessor_run_nonce", 32)?;
    require_str(reuse, "claim_digest", require_string(row, "claim_digest")?)?;
    require_str(
        reuse,
        "exact_command",
        require_string(row, "executed_exact_command")?,
    )?;
    let requirement = require_string(row, "requirement")?;
    let observed = validate_references(row, requirement)?;
    let inherited = string_set(reuse, "execution_observation_ids")?;
    if inherited.is_empty() || !inherited.is_subset(&observed) {
        return Err(
            "predecessor causal reuse references an unlisted execution observation".to_owned(),
        );
    }
    Ok(())
}

fn validate_references(row: &Value, requirement: &str) -> Result<BTreeSet<String>, String> {
    row["execution_receipts"]
        .as_array()
        .filter(|references| !references.is_empty())
        .ok_or_else(|| "predecessor row omits execution references".to_owned())?
        .iter()
        .map(|reference| {
            execution_observation::validate(reference, requirement)
                .map(|observation| observation.observation_sha256)
        })
        .collect()
}

fn string_set(value: &Value, field: &str) -> Result<BTreeSet<String>, String> {
    let identities = value[field]
        .as_array()
        .ok_or_else(|| format!("predecessor causal reuse omits {field}"))?;
    let mut result = BTreeSet::new();
    for identity in identities {
        let identity = identity
            .as_str()
            .ok_or_else(|| format!("predecessor causal reuse has invalid {field}"))?;
        if identity.len() != 64
            || !identity
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err(format!("predecessor causal reuse has invalid {field}"));
        }
        result.insert(identity.to_owned());
    }
    Ok(result)
}
