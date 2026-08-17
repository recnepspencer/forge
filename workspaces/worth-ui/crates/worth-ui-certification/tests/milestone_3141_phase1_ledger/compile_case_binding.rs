use serde_json::Value;

const COMPILE_ARTIFACT: &str = "_docs/worth-ui/milestone-3.14.1-evidence/compile-contracts.json";

pub(super) fn validate(
    requirement: &str,
    sources: &[String],
    result_artifact: &Value,
) -> Result<(), String> {
    let expected = super::execution_contract::compile_cases_for(requirement);
    if expected.is_empty() {
        return Ok(());
    }
    let identity = governed_compile_identity(sources, result_artifact)?;
    let path = super::source_digest::repository_file(identity)?;
    let bytes = std::fs::read(path).map_err(|error| error.to_string())?;
    let artifact: Value = serde_json::from_slice(&bytes)
        .map_err(|error| format!("invalid compile artifact: {error}"))?;
    validate_cases(&artifact, expected)
}

fn governed_compile_identity<'a>(
    sources: &'a [String],
    artifact: &Value,
) -> Result<&'a str, String> {
    let matches = sources
        .iter()
        .enumerate()
        .filter(|(_, source)| source.ends_with("compile-contracts.json"))
        .collect::<Vec<_>>();
    let [(index, identity)] = matches.as_slice() else {
        return Err("compile-backed row omits the governed compile artifact".to_owned());
    };
    if identity.as_str() == COMPILE_ARTIFACT {
        return Ok(identity);
    }
    let normalized = identity.replace('\\', "/");
    if !normalized.starts_with("workspaces/worth-ui/target/worth-ui-3141-verify-") {
        return Err("compile artifact rebind is not verifier-owned".to_owned());
    }
    let mapped = artifact["mapping_source_identity"]
        .as_array()
        .and_then(|sources| sources.get(*index))
        .and_then(Value::as_str);
    if mapped != Some(COMPILE_ARTIFACT) {
        return Err("compile artifact rebind lost its canonical mapping".to_owned());
    }
    let digest = super::source_digest::file_digest(identity)?;
    let exact = artifact["source_rebindings"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|record| {
            record["canonical"] == COMPILE_ARTIFACT
                && record["executed"] == identity.as_str()
                && record["sha256"] == digest
        })
        .count();
    (exact == 1)
        .then_some(identity.as_str())
        .ok_or_else(|| "compile artifact rebind is not hash-exact".to_owned())
}

fn validate_cases(
    artifact: &Value,
    expected: &[super::execution_contract::CompileCase],
) -> Result<(), String> {
    let cases = artifact["cases"]
        .as_array()
        .ok_or_else(|| "compile artifact omits cases".to_owned())?;
    for required in expected {
        let matches = cases
            .iter()
            .filter(|case| {
                case["owner"] == required.owner
                    && case["kind"] == required.kind
                    && case["target"] == required.target
            })
            .count();
        if matches != 1 {
            return Err(format!(
                "compile artifact must contain exactly one {}:{}:{} case",
                required.owner, required.kind, required.target
            ));
        }
    }
    Ok(())
}

#[test]
fn compile_case_deletion_reopens_its_own_requirement() {
    let required = super::execution_contract::compile_cases_for("P1-ORDER-SOURCE-01");
    let cases = required
        .iter()
        .map(|case| {
            serde_json::json!({
                "owner": case.owner,
                "kind": case.kind,
                "target": case.target,
            })
        })
        .collect::<Vec<_>>();
    let lawful = serde_json::json!({"cases": cases});
    validate_cases(&lawful, required).unwrap();
    let deleted = serde_json::json!({"cases": []});
    assert!(validate_cases(&deleted, required).is_err());
}
