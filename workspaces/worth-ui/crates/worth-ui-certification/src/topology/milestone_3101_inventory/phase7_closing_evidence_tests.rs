use serde_json::Value;

use super::{
    belongs_to_phase7_inventory, load_json, validate_closing_evidence, CLOSING_PATH, OPENING_PATH,
};
use crate::topology::WorkspaceSourceInventory;

fn documents() -> (WorkspaceSourceInventory, Value, Value) {
    let workspace = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("Worth UI workspace root");
    let inventory = WorkspaceSourceInventory::capture(workspace);
    let root = inventory
        .root()
        .parent()
        .and_then(std::path::Path::parent)
        .expect("repository root");
    let opening = load_json(&root.join(OPENING_PATH)).expect("opening baseline parses");
    let closing = load_json(&root.join(CLOSING_PATH)).expect("closing evidence parses");
    (inventory, opening, closing)
}

fn audit(closing: &Value) -> Result<(), String> {
    let (inventory, opening, _) = documents();
    let root = inventory
        .root()
        .parent()
        .and_then(std::path::Path::parent)
        .expect("repository root");
    validate_closing_evidence(&inventory, root, &opening, closing)
}

#[test]
fn phase7_closing_cost_evidence_is_comparable_and_complete() {
    let (_, _, closing) = documents();
    audit(&closing).expect("real closing evidence should remain closed");
}

#[test]
fn phase7_closing_rejects_missing_operation_category() {
    let (_, _, mut closing) = documents();
    closing["operation_costs"]
        .as_array_mut()
        .expect("operation rows")
        .pop();
    assert!(audit(&closing)
        .expect_err("missing operation category must fail")
        .contains("operation categories differ"));
}

#[test]
fn phase7_closing_rejects_methodology_drift() {
    let (_, _, mut closing) = documents();
    closing["comparison_methodology"] = Value::String("faster local method".to_owned());
    assert!(audit(&closing)
        .expect_err("methodology drift must fail")
        .contains("methodology differs"));
}

#[test]
fn phase7_closing_rejects_failed_measurement() {
    let (_, _, mut closing) = documents();
    closing["measurements"][0]["exit_code"] = Value::from(1);
    assert!(audit(&closing)
        .expect_err("failed measurement must fail")
        .contains("not comparable"));
}

#[test]
fn phase7_closing_rejects_empty_regression_adjudication() {
    let (_, _, mut closing) = documents();
    closing["measurements"][0]["adjudication"] = Value::String(String::new());
    assert!(audit(&closing)
        .expect_err("empty adjudication must fail")
        .contains("not comparable"));
}

#[test]
fn phase7_closing_rejects_shared_paired_targets() {
    let (_, _, mut closing) = documents();
    closing["holistic_qa_paired_measurement"]["targets"]["shared_artifacts"] = Value::Bool(true);
    assert!(audit(&closing)
        .expect_err("paired measurements must not share artifacts")
        .contains("distinct isolated targets"));
}

#[test]
fn phase7_closing_rejects_unadjudicated_paired_regression() {
    let (_, _, mut closing) = documents();
    closing["holistic_qa_paired_measurement"]["measurements"][2]["adjudication"] =
        Value::String(String::new());
    assert!(audit(&closing)
        .expect_err("a paired regression needs adjudication")
        .contains("not comparable"));
}

#[test]
fn phase7_inventory_excludes_only_named_successor_sources() {
    assert!(!belongs_to_phase7_inventory(
        "crates/worth-ui-certification/src/topology/milestone_3101_inventory/phase8_closeout.rs"
    ));
    assert!(!belongs_to_phase7_inventory(
        "crates/worth-ui-certification/src/topology/milestone_3102_pulse_seed/mod.rs"
    ));
    assert!(!belongs_to_phase7_inventory(
        "crates/worth-ui-certification/src/scenario/application_authority_closure/visual_identity_application.rs"
    ));
    assert!(!belongs_to_phase7_inventory(
        "crates/worth-ui-certification/src/scenario/filesystem_application_lifecycle/visual_identity.rs"
    ));
    assert!(!belongs_to_phase7_inventory(
        "crates/worth-ui-certification/src/intent_execution_provider.rs"
    ));
    assert!(!belongs_to_phase7_inventory(
        "crates/worth-ui-certification/src/scenario/filesystem_application_lifecycle/scaled_canvas.rs"
    ));
    assert!(belongs_to_phase7_inventory(
        "crates/worth-ui-certification/src/intent_execution_provider_neighbor.rs"
    ));
    assert!(belongs_to_phase7_inventory(
        "crates/worth-ui-certification/src/topology/unadjudicated_future.rs"
    ));
}
