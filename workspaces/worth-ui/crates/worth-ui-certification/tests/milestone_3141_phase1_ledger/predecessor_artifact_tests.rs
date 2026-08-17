use serde_json::{json, Value};

use super::super::{
    claim_contract, execution_contract, predecessor_current_mapping, requirement_contract,
    result_artifact_binding, schema, source_digest,
};
use super::{calculate_mapping_digest, validate_value, validate_value_with_mapping};

#[test]
fn phase_three_predecessor_rows_use_the_future_requirement_contract_owner() {
    let row = json!({"structural_counter": "requirements=30"});
    assert!(super::validate_counter(&row, "P3-PREDECESSOR-01").is_ok());
}

#[test]
fn phase_three_predecessor_inventory_selects_by_phase_not_append_position() {
    let expected = super::super::predecessor_inventory::predecessor_requirements(47);
    assert_eq!(expected.len(), 47);
    assert!(expected.contains("P3-CLIPPED-DELTA-01"));
    assert!(!expected.contains("P4-BIDI-01"));
    assert!(!expected.contains("P5-PREDECESSOR-01"));
}

#[test]
fn stale_source_or_missing_row_is_rejected() {
    let revision = result_artifact_binding::current_revision().unwrap();
    let source_state = source_digest::calculate_source_state(&revision).unwrap();
    let lawful = fixture(&revision, &source_state);
    let mapping = lawful["mapping_digest"].as_str().unwrap().to_owned();
    validate_value(&lawful, &revision, &source_state).unwrap();

    let mut substituted = lawful.clone();
    substituted["rows"][0]["production_entry"] = substituted["rows"][1]["production_entry"].clone();
    assert_mapping_mutant(
        &mut substituted,
        &revision,
        &source_state,
        "P1-AFFINITY-01 has wrong production_entry",
    );

    let mut substituted_oracle = lawful.clone();
    substituted_oracle["rows"][0]["independent_oracle"] =
        substituted_oracle["rows"][1]["independent_oracle"].clone();
    assert_mapping_mutant(
        &mut substituted_oracle,
        &revision,
        &source_state,
        "P1-AFFINITY-01 has wrong independent_oracle",
    );

    let mut omitted_source = lawful.clone();
    omitted_source["rows"][5]["mapping_source_identity"]
        .as_array_mut()
        .unwrap()
        .pop();
    omitted_source["rows"][5]["source_identity"]
        .as_array_mut()
        .unwrap()
        .pop();
    let reduced_sources = omitted_source["rows"][5]["source_identity"]
        .as_array()
        .unwrap()
        .iter()
        .map(|source| source.as_str().unwrap())
        .collect::<Vec<_>>()
        .join(";");
    omitted_source["rows"][5]["source_digest"] =
        Value::from(source_digest::calculate(&reduced_sources).unwrap());
    assert_mapping_mutant(
        &mut omitted_source,
        &revision,
        &source_state,
        "P1-CONSUMERS-01 has wrong required source identities",
    );

    let mut stale = lawful.clone();
    stale["source_state_digest"] = Value::from("0".repeat(64));
    assert_eq!(
        validate_value_with_mapping(&stale, &revision, &source_state, &mapping),
        Err("predecessor artifact has wrong source_state_digest".to_owned())
    );

    let mut missing = lawful;
    missing["rows"].as_array_mut().unwrap().pop();
    refresh_derived_totals(&mut missing);
    missing["mapping_digest"] = Value::from(calculate_mapping_digest(&missing["rows"]));
    let missing_mapping = missing["mapping_digest"].as_str().unwrap().to_owned();
    assert_eq!(
        validate_value_with_mapping(&missing, &revision, &source_state, &missing_mapping),
        Err("predecessor artifact has the wrong row count".to_owned())
    );
    println!(
        "WORTH_UI_LEDGER_MUTATION_CONTROLS={{\"P3-PREDECESSOR-01\":\"stale-phase-two-source\"}}"
    );
}

fn refresh_derived_totals(artifact: &mut Value) {
    let (main, controls, costs) = {
        let rows = artifact["rows"].as_array().unwrap();
        let main = super::unique_execution_total(rows, "main-test").unwrap();
        let controls = super::unique_execution_total(rows, "control-test").unwrap();
        let costs = [
            (
                "product_processes",
                "construction_cost",
                "product-processes",
            ),
            ("courtroom_worlds", "construction_cost", "courtroom-worlds"),
            ("presentations", "execution_cost", "presentations"),
        ]
        .map(|(field, cost_field, name)| {
            let prefix = format!("{name}=");
            let total = rows
                .iter()
                .filter(|row| row.get("shared_main_artifact").is_none())
                .filter_map(|row| row[cost_field].as_str())
                .flat_map(|cost| cost.split(';'))
                .filter_map(|entry| entry.strip_prefix(&prefix))
                .filter_map(|amount| amount.parse::<u64>().ok())
                .sum::<u64>();
            (field, total)
        });
        (main, controls, costs)
    };
    artifact["main_test_executions"] = Value::from(main);
    artifact["hostile_control_executions"] = Value::from(controls);
    for (field, total) in costs {
        artifact[field] = Value::from(total);
    }
}

#[test]
fn phase_four_stale_source_or_missing_row_is_rejected() {
    let revision = result_artifact_binding::current_revision().unwrap();
    let source_state = source_digest::calculate_source_state(&revision).unwrap();
    let mut artifact = fixture(&revision, &source_state);
    artifact["through_phase"] = Value::from(3);
    artifact["verified_requirement_count"] = Value::from(47);
    artifact["mapping_digest"] = Value::from(super::EXPECTED_PHASE_THREE_MAPPING_DIGEST);

    let mut stale = artifact.clone();
    stale["source_state_digest"] = Value::from("0".repeat(64));
    assert_eq!(
        validate_value(&stale, &revision, &source_state),
        Err("predecessor artifact has wrong source_state_digest".to_owned())
    );
    assert_eq!(
        validate_value(&artifact, &revision, &source_state),
        Err("predecessor artifact has wrong mapping_digest".to_owned())
    );
    println!(
        "WORTH_UI_LEDGER_MUTATION_CONTROLS={{\"P4-PREDECESSOR-01\":\"stale-phase-three-source\"}}"
    );
}

#[test]
fn phase_five_stale_source_or_missing_row_is_rejected() {
    let revision = result_artifact_binding::current_revision().unwrap();
    let source_state = source_digest::calculate_source_state(&revision).unwrap();
    let mut artifact = fixture(&revision, &source_state);
    artifact["through_phase"] = Value::from(4);
    artifact["verified_requirement_count"] = Value::from(68);
    artifact["mapping_digest"] = Value::from("0".repeat(64));
    let mut stale = artifact.clone();
    stale["source_state_digest"] = Value::from("0".repeat(64));
    assert_eq!(
        validate_value(&stale, &revision, &source_state),
        Err("predecessor artifact has wrong source_state_digest".to_owned())
    );
    assert_eq!(
        validate_value(&artifact, &revision, &source_state),
        Err("predecessor artifact has wrong mapping_digest".to_owned())
    );
    println!(
        "WORTH_UI_LEDGER_MUTATION_CONTROLS={{\"P5-PREDECESSOR-01\":\"stale-phase-four-source\"}}"
    );
}

fn assert_mapping_mutant(
    artifact: &mut Value,
    revision: &str,
    source_state: &str,
    expected_error: &str,
) {
    artifact["mapping_digest"] = Value::from(calculate_mapping_digest(&artifact["rows"]));
    let mutant_mapping = artifact["mapping_digest"].as_str().unwrap().to_owned();
    assert_eq!(
        validate_value_with_mapping(artifact, revision, source_state, &mutant_mapping),
        Err(expected_error.to_owned())
    );
}

fn fixture(revision: &str, source_state: &str) -> Value {
    let context = FixtureContext {
        revision,
        source_state,
    };
    let rows = schema::EXPECTED_REQUIREMENTS[..30]
        .iter()
        .enumerate()
        .map(|(index, requirement)| fixture_row(requirement, index, &context))
        .collect::<Vec<_>>();
    let mut artifact = json!({
        "schema": "worth-ui-phase-predecessor-handoff-v1",
        "through_phase": 2,
        "source_revision": revision,
        "source_state_digest": source_state,
        "verified_requirement_count": 30,
        "main_test_executions": 30,
        "hostile_control_executions": 11,
        "closure_test_executions": 2,
        "compile_sessions": 2,
        "product_processes": 1,
        "courtroom_worlds": 2,
        "presentations": 8,
        "run_nonce": "f".repeat(32),
        "rows": rows,
    });
    artifact["mapping_digest"] = Value::from(calculate_mapping_digest(&artifact["rows"]));
    artifact
}

struct FixtureContext<'a> {
    revision: &'a str,
    source_state: &'a str,
}

fn fixture_row(requirement: &str, index: usize, context: &FixtureContext<'_>) -> Value {
    let main = execution_contract::current_predecessor_main_for(requirement).unwrap();
    let contract = requirement_contract::for_requirement(requirement).unwrap();
    let amount =
        super::execution_contract::current_predecessor_counter_amount(requirement).unwrap();
    let expected_ignored = execution_contract::expected_declared_ignored(requirement);
    let marginal_main = u64::from(!execution_contract::is_shared_main(requirement));
    let hostile_control = execution_contract::control_for(requirement).map(|control| {
        json!({
            "package": control.package,
            "target_kind": control.target_kind,
            "target_name": control.target_name,
            "features": control.features,
            "test_name": control.test_name,
            "matched_test_count": 1,
            "executed_test_count": 1,
            "passed_test_count": 1,
            "ignored_test_count": 0,
            "exit_posture": "passed",
        })
    });
    let mut execution_receipts = vec![json!({
        "role": "main-test",
        "key": format!("{:064x}", index + 1),
    })];
    if hostile_control.is_some() {
        execution_receipts.push(json!({
            "role": "control-test",
            "key": format!("{:064x}", index + 101),
        }));
    }
    let mapping = predecessor_current_mapping::expected(requirement).unwrap();
    let mapping_sources = mapping.source_identity.split(';').collect::<Vec<_>>();
    let selected_digest = source_digest::calculate(mapping.source_identity).unwrap();
    json!({
        "requirement": requirement,
        "production_entry": mapping.production_entry,
        "independent_oracle": mapping.independent_oracle,
        "package": main.package,
        "target_kind": main.target_kind,
        "target_name": main.target_name,
        "features": main.features,
        "test_name": main.test_name,
        "matched_test_count": 1,
        "declared_ignored_test_count": u64::from(expected_ignored),
        "expected_declared_ignored": expected_ignored,
        "executed_test_count": marginal_main,
        "passed_test_count": marginal_main,
        "ignored_test_count": 0,
        "exit_posture": "passed",
        "source_revision": context.revision,
        "source_identity": mapping_sources.clone(),
        "mapping_source_identity": mapping_sources,
        "source_rebindings": [],
        "source_digest": selected_digest,
        "source_state_digest": context.source_state,
        "run_nonce": format!("{index:032x}"),
        "artifact_sha256": format!("{index:064x}"),
        "structural_counter": format!("{}={amount}", contract.counter_family),
        "hostile_control": hostile_control,
        "execution_receipts": execution_receipts,
        "construction_cost": claim_contract::construction_cost(requirement),
        "execution_cost": claim_contract::execution_cost(requirement),
    })
}
