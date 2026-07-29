use worth_ui_certification::topology::WorkspaceSourceInventory;

const FIXTURE: &str =
    "apps/platform-pulse/tests/fixtures/lifecycle_protocol/v2/inherited_lifecycle_envelopes.jsonl";
const PREFIX: &str = "WORTH_UI_PLATFORM_PULSE_EVENT ";

pub(super) fn validate_raw_v2_fixture(fixture: &str) -> Result<(), String> {
    validate_raw_v2_text(fixture)
}

fn validate_raw_v2_text(fixture: &str) -> Result<(), String> {
    if fixture.len() > 1_048_576 {
        return Err("raw v2 fixture exceeds the production protocol byte limit".to_owned());
    }
    let mut expected_sequence = 1_u64;
    let mut run = None::<String>;
    let mut count = 0;
    for line in fixture.lines() {
        let json = line
            .strip_prefix(PREFIX)
            .ok_or_else(|| "raw v2 fixture line has no production prefix".to_owned())?;
        let value: serde_json::Value =
            serde_json::from_str(json).map_err(|error| format!("invalid fixture JSON: {error}"))?;
        let decoded = worth_ui_platform_pulse::observation_contract::
            PlatformPulseLifecycleObservationEnvelope::decode_compatible_prefixed_line(line)
            .map_err(|denial| format!("production v2 decoder rejected raw fixture: {denial:?}"))?;
        let protocol = &value["protocol"];
        if protocol["identity"].as_str() != Some("worth-ui.platform-pulse.lifecycle-observation")
            || protocol["schema_version"].as_u64() != Some(2)
        {
            return Err("raw lifecycle fixture is not governed protocol v2".to_owned());
        }
        if value.get("rebind").is_some()
            || value.get("comparison").is_some()
            || value["outcome"]["payload"].get("rebind").is_some()
            || value["outcome"]["payload"].get("comparison").is_some()
        {
            return Err("raw v2 fixture invents Milestone 3.12 evidence".to_owned());
        }
        if !matches!(
            decoded,
            worth_ui_platform_pulse::observation_contract::
                PlatformPulseDecodedLifecycleObservation::InheritedLifecycleOnly(_)
        ) {
            return Err("production decoder did not preserve inherited v2 identity".to_owned());
        }
        let current_run = value["run"]["value"]
            .as_str()
            .ok_or_else(|| "raw lifecycle fixture has no run identity".to_owned())?;
        match run.as_deref() {
            Some(expected) if expected != current_run => {
                return Err("raw lifecycle fixture crosses run identities".to_owned());
            }
            None => run = Some(current_run.to_owned()),
            Some(_) => {}
        }
        if value["sequence"]["value"].as_u64() != Some(expected_sequence) {
            return Err("raw lifecycle fixture sequence is not contiguous".to_owned());
        }
        if value["outcome"]["kind"].as_str().is_none() {
            return Err("raw lifecycle fixture has no typed outcome family".to_owned());
        }
        expected_sequence += 1;
        count += 1;
    }
    if count < 2 {
        return Err("raw v2 fixture does not prove lifecycle succession".to_owned());
    }
    Ok(())
}

#[test]
fn raw_v2_fixture_rejects_cross_run_inheritance() {
    let fixture = crate::repository_document(&format!("workspaces/worth-ui/{FIXTURE}"));
    let marker = "\"run\":{\"value\":\"governed-v2-fixture-0001\"}";
    let first = fixture.find(marker).expect("first governed run marker");
    let suffix_start = first + marker.len();
    let second = suffix_start
        + fixture[suffix_start..]
            .find(marker)
            .expect("second governed run marker");
    let mut hostile = fixture;
    hostile.replace_range(
        second..second + marker.len(),
        "\"run\":{\"value\":\"foreign-v2-run\"}",
    );
    assert!(validate_raw_v2_text(&hostile).is_err());
}

#[test]
fn raw_v2_fixture_rejects_prefix_identity_version_and_invented_fields() {
    let fixture = crate::repository_document(&format!("workspaces/worth-ui/{FIXTURE}"));
    let hostile = [
        fixture.replacen(PREFIX, "", 1),
        fixture.replacen(
            "worth-ui.platform-pulse.lifecycle-observation",
            "foreign.lifecycle-observation",
            1,
        ),
        fixture.replacen("\"schema_version\":2", "\"schema_version\":3", 1),
        fixture.replacen("\"run\":", "\"rebind\":{},\"run\":", 1),
        fixture.replacen("\"payload\":{}", "\"payload\":{\"comparison\":{}}", 1),
    ];
    for mutation in hostile {
        assert!(validate_raw_v2_text(&mutation).is_err());
    }
}

pub(super) fn validate_failure_artifact_separation(
    inventory: &WorkspaceSourceInventory,
    fixture: &str,
) -> Result<(), String> {
    let artifact = inventory
        .text("apps/platform-pulse/tests/executable_world/failure_teardown/retained_artifact.rs");
    for required in [
        "worth-ui.platform-pulse.failure-artifact.v1",
        "trace: snapshot.trace()",
        "retained_by_default: true",
    ] {
        if !artifact.contains(required) {
            return Err(format!("failure artifact contract omits `{required}`"));
        }
    }
    for forbidden in [
        "failure-artifact.v1",
        "\"primary\"",
        "\"teardown\"",
        "\"trace\"",
    ] {
        if fixture.contains(forbidden) {
            return Err(format!("raw v2 fixture was relabeled with `{forbidden}`"));
        }
    }
    Ok(())
}

#[test]
fn raw_v2_fixture_rejects_failure_artifact_relabeling() {
    let fixture = crate::repository_document(&format!("workspaces/worth-ui/{FIXTURE}"));
    let relabeled = format!("{fixture}\"worth-ui.platform-pulse.failure-artifact.v1\"");
    assert!(
        validate_failure_artifact_separation(crate::workspace_source_inventory(), &relabeled)
            .is_err()
    );
}
