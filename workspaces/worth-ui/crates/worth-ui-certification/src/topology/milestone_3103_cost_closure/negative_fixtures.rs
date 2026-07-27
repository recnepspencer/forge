use std::path::PathBuf;

use serde_json::Value;

use super::closing_evidence;

#[test]
fn live_phase5_cost_and_handoff_evidence_is_closed() {
    let root = repository_root();
    let evidence = evidence(&root);
    closing_evidence::audit(&root, &evidence).expect("live Phase 5 closing evidence");
}

#[test]
fn budget_overrun_and_lane_blurring_are_rejected() {
    let root = repository_root();
    let evidence = evidence(&root);

    let mut overrun = evidence.clone();
    overrun["build"]["clean_seconds"] = Value::from(241);
    let error = closing_evidence::audit(&root, &overrun).expect_err("overrun must fail");
    assert!(error.contains("clean_seconds"));

    let mut blurred = evidence;
    blurred["proof_lanes"][1]["claim_boundary"] =
        Value::String("real executable product world".to_owned());
    let error = closing_evidence::audit(&root, &blurred).expect_err("blurred lane must fail");
    assert!(error.contains("blurred"));
}

#[test]
fn platform_overclaim_and_passing_residue_are_rejected() {
    let root = repository_root();
    let evidence = evidence(&root);

    let mut overclaim = evidence.clone();
    overclaim["native_platforms"][1]["posture"] = Value::String("certified_executable".to_owned());
    let error =
        closing_evidence::audit(&root, &overclaim).expect_err("platform overclaim must fail");
    assert!(error.contains("overclaims"));

    let mut residue = evidence;
    residue["failure_artifacts"]["passing_child_processes"] = Value::from(1);
    let error = closing_evidence::audit(&root, &residue).expect_err("residue must fail");
    assert!(error.contains("residue"));
}

#[test]
fn missing_successor_handoff_and_immature_324_are_rejected() {
    let root = repository_root();
    let evidence = evidence(&root);

    let mut missing = evidence.clone();
    missing["successor_handoffs"]
        .as_array_mut()
        .expect("handoffs")
        .pop();
    let error = closing_evidence::audit(&root, &missing).expect_err("missing handoff must fail");
    assert!(error.contains("handoff count"));

    let mut immature = evidence;
    immature["mature_world"]["product_entry_already_mature"] = Value::Bool(false);
    let error = closing_evidence::audit(&root, &immature).expect_err("immature 3.24 must fail");
    assert!(error.contains("3.24"));
}

fn evidence(repository_root: &std::path::Path) -> Value {
    closing_evidence::load(
        &repository_root.join("_docs/worth-ui/milestone-3.10.3-phase-5-closing-evidence.json"),
    )
    .expect("closing evidence")
}

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates")
        .parent()
        .expect("workspace")
        .parent()
        .expect("worth-ui")
        .parent()
        .expect("forge workspace")
        .to_path_buf()
}
