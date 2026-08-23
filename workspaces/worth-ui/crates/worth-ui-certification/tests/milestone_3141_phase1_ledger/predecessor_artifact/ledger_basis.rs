use std::collections::BTreeMap;

use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};

use super::{claim_digest, ledger_document};

const SCHEMA: &str = "worth-ui-ledger-candidate-basis-v1";

pub(super) fn validate(
    observed: &Value,
    through_phase: u64,
    handoff_rows: &Value,
) -> Result<(), String> {
    let expected = expected(through_phase)?;
    if observed != &expected {
        return Err("predecessor verification basis differs from candidate ledger".to_owned());
    }
    let handoff_claims = handoff_rows
        .as_array()
        .ok_or_else(|| "predecessor rows are not an array".to_owned())?
        .iter()
        .map(|row| {
            Ok(json!({
                "requirement": row["requirement"].as_str()
                    .ok_or_else(|| "handoff row omits requirement".to_owned())?,
                "claim_digest": row["claim_digest"].as_str()
                    .ok_or_else(|| "handoff row omits claim digest".to_owned())?,
            }))
        })
        .collect::<Result<Vec<_>, String>>()?;
    if observed["claim_inventory"] != Value::Array(handoff_claims) {
        return Err("predecessor claim inventory differs from handoff rows".to_owned());
    }
    Ok(())
}

pub(super) fn expected(through_phase: u64) -> Result<Value, String> {
    calculate(&ledger_document(), through_phase)
}

fn calculate(document: &str, through_phase: u64) -> Result<Value, String> {
    let mut reader = csv::Reader::from_reader(document.as_bytes());
    let headers = reader.headers().map_err(|error| error.to_string())?.clone();
    let mut rows = Vec::new();
    let mut claims = Vec::new();
    for record in reader.records() {
        let record = record.map_err(|error| error.to_string())?;
        let row = headers
            .iter()
            .zip(record.iter())
            .map(|(field, value)| (field.to_owned(), value.to_owned()))
            .collect::<BTreeMap<_, _>>();
        if row["phase"]
            .parse::<u64>()
            .map_err(|error| error.to_string())?
            > through_phase
        {
            continue;
        }
        claims.push(json!({
            "requirement": row["requirement"],
            "claim_digest": claim_digest::calculate(&row),
        }));
        rows.push(Value::Object(
            headers
                .iter()
                .map(|field| (field.to_owned(), Value::String(row[field].clone())))
                .collect::<Map<_, _>>(),
        ));
    }
    let prefix = json!({
        "schema": SCHEMA,
        "through_phase": through_phase,
        "fields": headers.iter().collect::<Vec<_>>(),
        "rows": rows,
    });
    let inventory = json!({
        "schema": format!("{SCHEMA}-claims"),
        "through_phase": through_phase,
        "claims": claims,
    });
    Ok(json!({
        "schema": SCHEMA,
        "through_phase": through_phase,
        "candidate_prefix_digest": digest_json(&prefix),
        "claim_inventory": inventory["claims"],
        "claim_inventory_digest": digest_json(&inventory),
    }))
}

fn digest_json(value: &Value) -> String {
    let mut digest = Sha256::new();
    digest.update(serde_json::to_vec(value).expect("candidate basis should serialize"));
    format!("{:x}", digest.finalize())
}
