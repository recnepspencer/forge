use serde_json::Value;

thread_local! {
    static ACTIVE_LEDGER: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) };
}

pub(super) struct ActiveLedger;

pub(super) fn install(ledger: &str) -> ActiveLedger {
    ACTIVE_LEDGER.with(|active| active.borrow_mut().push(ledger.to_owned()));
    ActiveLedger
}

impl Drop for ActiveLedger {
    fn drop(&mut self) {
        ACTIVE_LEDGER.with(|active| {
            active.borrow_mut().pop();
        });
    }
}

pub(super) fn require_proved_artifact(
    requirement: &str,
    identity: &str,
    digest: &str,
    artifact: &Value,
) -> Result<(), String> {
    let ledger = ACTIVE_LEDGER
        .with(|active| active.borrow().last().cloned())
        .unwrap_or_else(super::ledger_document);
    let row = producer_row(&ledger, requirement)?;
    validate_row(&row, identity, digest, artifact)
}

fn producer_row(
    ledger: &str,
    requirement: &str,
) -> Result<std::collections::BTreeMap<String, String>, String> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(ledger.as_bytes());
    let headers = reader.headers().map_err(|error| error.to_string())?.clone();
    let mut matched = None;
    for record in reader.records() {
        let record = record.map_err(|error| error.to_string())?;
        let row = headers
            .iter()
            .zip(record.iter())
            .map(|(field, value)| (field.to_owned(), value.to_owned()))
            .collect::<std::collections::BTreeMap<_, _>>();
        if row.get("requirement").map(String::as_str) == Some(requirement) {
            if matched.replace(row).is_some() {
                return Err("dependency ledger repeats its producer".to_owned());
            }
        }
    }
    matched.ok_or_else(|| "dependency ledger omits its producer".to_owned())
}

fn validate_row(
    row: &std::collections::BTreeMap<String, String>,
    identity: &str,
    digest: &str,
    artifact: &Value,
) -> Result<(), String> {
    require_column(&row, "result", "PROVED")?;
    require_column(&row, "final_source", "true")?;
    require_column(&row, "retained_result_artifact", identity)?;
    require_column(&row, "result_artifact_digest", digest)?;
    let claim_digest = super::claim_digest::calculate(&row);
    if artifact.get("claim_digest").and_then(Value::as_str) != Some(&claim_digest) {
        return Err("dependency artifact drifted claim_digest".to_owned());
    }
    for field in [
        "source_revision",
        "source_digest",
        "source_state_digest",
        "run_nonce",
    ] {
        let expected = row
            .get(field)
            .map(String::as_str)
            .ok_or_else(|| format!("dependency ledger omits {field}"))?;
        if artifact.get(field).and_then(Value::as_str) != Some(expected) {
            return Err(format!("dependency artifact drifted {field}"));
        }
    }
    let sources = artifact
        .get("source_identity")
        .and_then(Value::as_array)
        .ok_or_else(|| "dependency artifact omits source identity".to_owned())?;
    let expected_sources = row["source_identity"].split(';').collect::<Vec<_>>();
    if sources.iter().filter_map(Value::as_str).collect::<Vec<_>>() != expected_sources {
        return Err("dependency artifact drifted source identity".to_owned());
    }
    Ok(())
}

#[test]
fn open_digest_nonce_and_source_substitutions_cannot_authorize_a_dependency() {
    let ledger = super::ledger_document();
    let mut row = producer_row(&ledger, "P3-DELTA-SOURCE-01").unwrap();
    let identity = row["retained_result_artifact"].clone();
    let digest = "artifact-digest";
    let artifact_for = |row: &std::collections::BTreeMap<String, String>| {
        serde_json::json!({
            "claim_digest": super::claim_digest::calculate(row),
            "source_revision": row["source_revision"],
            "source_digest": row["source_digest"],
            "source_state_digest": row["source_state_digest"],
            "run_nonce": row["run_nonce"],
            "source_identity": row["source_identity"].split(';').collect::<Vec<_>>(),
        })
    };
    assert!(validate_row(&row, &identity, digest, &artifact_for(&row)).is_err());
    row.insert("result".to_owned(), "PROVED".to_owned());
    row.insert("final_source".to_owned(), "true".to_owned());
    row.insert("result_artifact_digest".to_owned(), digest.to_owned());
    let artifact = artifact_for(&row);
    validate_row(&row, &identity, digest, &artifact).unwrap();
    for field in [
        "run_nonce",
        "source_digest",
        "source_state_digest",
        "source_identity",
    ] {
        let mut mutant = artifact.clone();
        mutant[field] = Value::from("substitute");
        assert!(
            validate_row(&row, &identity, digest, &mutant).is_err(),
            "{field}"
        );
    }
    assert!(validate_row(&row, &identity, "substitute", &artifact).is_err());
}

fn require_column(
    row: &std::collections::BTreeMap<String, String>,
    field: &str,
    expected: &str,
) -> Result<(), String> {
    (row.get(field).map(String::as_str) == Some(expected))
        .then_some(())
        .ok_or_else(|| format!("dependency producer has wrong {field}"))
}
