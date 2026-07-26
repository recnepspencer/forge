use super::{
    audit, audit_insertions, audit_proof_ledger_text, reject_forbidden_fragments,
    require_exact_fenced_source, require_headings, require_marker,
};
use crate::topology::WorkspaceSourceInventory;

fn repository_root() -> std::path::PathBuf {
    let workspace_root =
        std::fs::canonicalize(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."))
            .expect("workspace root");
    WorkspaceSourceInventory::capture(workspace_root)
        .root()
        .parent()
        .and_then(std::path::Path::parent)
        .expect("repository root")
        .to_path_buf()
}

fn closeout() -> toml::Value {
    super::ledger::load(
        &repository_root().join("_docs/worth-ui/milestone-3.10.1-phase-8-closeout.toml"),
    )
    .expect("Phase 8 closeout manifest")
}

#[test]
fn phase8_closeout_is_complete_and_exact() {
    audit(&closeout(), &repository_root()).expect("Phase 8 closeout");
}

#[test]
fn phase8_rejects_a_stale_public_route() {
    let error = reject_forbidden_fragments(
        "docs/application-lifecycle.md",
        "call execute_framework_turn directly",
        false,
        &["execute_framework_turn"],
    )
    .expect_err("stale route should fail");
    assert!(error.contains("stale route"));
}

#[test]
fn phase8_rejects_a_missing_required_section() {
    let error = require_headings(
        "docs/inspection.md",
        "# Application Inspection\n",
        &["# Application Inspection", "## Anti-Patterns"],
    )
    .expect_err("missing heading should fail");
    assert!(error.contains("## Anti-Patterns"));
}

#[test]
fn phase8_rejects_a_wrong_future_insertion_owner() {
    let mut closing = closeout();
    closing
        .get_mut("future_insertion")
        .and_then(toml::Value::as_array_mut)
        .and_then(|rows| rows.first_mut())
        .and_then(toml::Value::as_table_mut)
        .expect("first insertion")
        .insert(
            "owner".to_owned(),
            toml::Value::String("session".to_owned()),
        );
    let phase4 = super::ledger::load(
        &repository_root().join("_docs/worth-ui/milestone-3.10.1-phase-4-runtime-subsystems.toml"),
    )
    .expect("Phase 4 authority");
    let roadmap =
        std::fs::read_to_string(repository_root().join("_docs/worth-ui/worth_ui_roadmap.md"))
            .expect("roadmap");
    let error = audit_insertions(&closing, &phase4, &roadmap).expect_err("wrong owner should fail");
    assert!(error.contains("owner"));
}

#[test]
fn phase8_rejects_a_stale_future_insertion_heading() {
    let mut phase4 = super::ledger::load(
        &repository_root().join("_docs/worth-ui/milestone-3.10.1-phase-4-runtime-subsystems.toml"),
    )
    .expect("Phase 4 authority");
    let mut closing = closeout();
    for document in [&mut phase4, &mut closing] {
        document
            .get_mut("future_insertion")
            .and_then(toml::Value::as_array_mut)
            .and_then(|rows| rows.get_mut(2))
            .and_then(toml::Value::as_table_mut)
            .expect("3.17 insertion")
            .insert(
                "roadmap_heading".to_owned(),
                toml::Value::String("### Milestone 3.17: Service Planning".to_owned()),
            );
    }
    let roadmap =
        std::fs::read_to_string(repository_root().join("_docs/worth-ui/worth_ui_roadmap.md"))
            .expect("roadmap");
    let error = audit_insertions(&closing, &phase4, &roadmap)
        .expect_err("matching stale manifests should still fail");
    assert!(error.contains("roadmap heading"));
}

#[test]
fn phase8_rejects_an_open_proof_row() {
    let ledger = concat!(
        "\"claim\",\"phase\",\"closure_claim\",\"required_evidence\",",
        "\"current_evidence\",\"risk_lenses\",\"status\"\n",
        "\"claim\",\"8\",\"closed\",\"proof\",\"not yet evaluated\",\"risk\",",
        "\"NOT_EVALUATED\"\n"
    );
    let error = audit_proof_ledger_text("phase-8.csv", ledger).expect_err("open row should fail");
    assert!(error.contains("empty evidence") || error.contains("open status"));
}

#[test]
fn phase8_rejects_a_missing_completion_marker() {
    let error = require_marker("roadmap", "Milestone 3.10.1", "Status: Complete")
        .expect_err("missing marker should fail");
    assert!(error.contains("completion marker"));
}

#[test]
fn phase8_rejects_documented_source_drift() {
    let document = "<!-- example -->\n```rust\nfn main() {}\n```\n";
    let error = require_exact_fenced_source(
        "ordinary-mounted-frame",
        document,
        "<!-- example -->",
        "fn main() { panic!() }",
    )
    .expect_err("documentation drift should fail");
    assert!(error.contains("differs from its compiled source"));
}
