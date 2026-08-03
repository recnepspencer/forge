use std::collections::BTreeSet;

pub(super) fn validate(contract: &toml::Value) -> Result<(), String> {
    let protocol = &contract["protocol"];
    require_exact(
        protocol,
        "identity",
        "worth-ui.platform-pulse.lifecycle-observation",
    )?;
    require_exact(protocol, "v2_posture", "InheritedLifecycleOnly")?;
    require_exact(
        protocol,
        "failure_artifact_identity",
        "worth-ui.platform-pulse.failure-artifact.v1",
    )?;
    let accepted = protocol["accepted_versions"]
        .as_array()
        .ok_or_else(|| "accepted protocol versions are not an array".to_owned())?
        .iter()
        .map(|version| version.as_integer())
        .collect::<Vec<_>>();
    if protocol["producer_version"].as_integer() != Some(3) || accepted != [Some(2), Some(3)] {
        return Err("v2/v3 coexistence contract drifted".to_owned());
    }
    require_exact(
        protocol,
        "v2_fixture_root",
        "workspaces/worth-ui/apps/platform-pulse/tests/fixtures/lifecycle_protocol/v2",
    )?;
    require_exact(
        protocol,
        "failure_artifact_posture",
        "separate normalized diagnostic bundle; never a raw lifecycle envelope",
    )?;
    validate_inspection(&contract["inspection"])
}

fn validate_inspection(inspection: &toml::Value) -> Result<(), String> {
    let supported = required_string_set(inspection, "supported")?;
    if supported
        != BTreeSet::from([
            "exact-key reference lookup",
            "rebind decision record",
            "summary",
            "visual snapshot comparison",
        ])
    {
        return Err("inspection support set drifted".to_owned());
    }
    let deferred = required_string_set(inspection, "deferred")?;
    if deferred
        != BTreeSet::from([
            "human and agent inspector",
            "materialized causal diagnostics",
            "replay",
        ])
    {
        return Err("deferred inspection set drifted".to_owned());
    }
    require_exact(
        inspection,
        "forbidden",
        "construct, execute, retry, amend, or republish runtime authority",
    )
}

fn require_exact(value: &toml::Value, field: &str, expected: &str) -> Result<(), String> {
    let actual = value[field]
        .as_str()
        .filter(|text| !text.trim().is_empty())
        .ok_or_else(|| format!("missing nonempty `{field}`"))?;
    if actual == expected {
        Ok(())
    } else {
        Err(format!("`{field}` expected `{expected}`, found `{actual}`"))
    }
}

fn required_string_set<'a>(
    value: &'a toml::Value,
    field: &str,
) -> Result<BTreeSet<&'a str>, String> {
    value[field]
        .as_array()
        .ok_or_else(|| format!("`{field}` is not an array"))?
        .iter()
        .map(|entry| {
            entry
                .as_str()
                .filter(|text| !text.trim().is_empty())
                .ok_or_else(|| format!("`{field}` contains a non-string or empty value"))
        })
        .collect()
}
