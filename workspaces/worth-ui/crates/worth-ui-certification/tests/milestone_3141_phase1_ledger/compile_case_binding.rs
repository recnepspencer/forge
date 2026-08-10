use serde_json::Value;

const COMPILE_ARTIFACT: &str = "_docs/worth-ui/milestone-3.14.1-evidence/compile-contracts.json";

pub(super) fn validate(requirement: &str, sources: &[String]) -> Result<(), String> {
    let expected = super::execution_contract::compile_cases_for(requirement);
    if expected.is_empty() {
        return Ok(());
    }
    if !sources.iter().any(|source| source == COMPILE_ARTIFACT) {
        return Err("compile-backed row omits the governed compile artifact".to_owned());
    }
    let path = super::source_digest::repository_file(COMPILE_ARTIFACT)?;
    let bytes = std::fs::read(path).map_err(|error| error.to_string())?;
    let artifact: Value = serde_json::from_slice(&bytes)
        .map_err(|error| format!("invalid compile artifact: {error}"))?;
    validate_cases(&artifact, expected)
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
