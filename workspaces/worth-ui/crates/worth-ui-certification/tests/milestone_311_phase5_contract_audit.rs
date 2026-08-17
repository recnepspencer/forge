use std::collections::BTreeSet;

use crate::{repository_document, workspace_source_inventory};
use worth_ui_certification::topology::WorkspaceSourceInventory;

#[path = "milestone_311_phase5_contract_audit/documentation.rs"]
mod documentation;
#[path = "milestone_311_phase5_contract_audit/ledger.rs"]
mod ledger;

#[test]
fn phase_5_contract_freezes_the_exact_ordered_batches_and_scenarios() {
    let contract = phase_5_contract();
    let status = contract["status"].as_str().expect("Phase 5 status");
    assert!(
        matches!(status, "implementation" | "closed"),
        "Phase 5 has an unknown status {status}"
    );

    let batches = contract["batch"]
        .as_array()
        .expect("Phase 5 batches are an array");
    assert_eq!(
        batches
            .iter()
            .map(|batch| batch["id"].as_str().expect("batch id"))
            .collect::<Vec<_>>(),
        ["5A", "5B", "5C", "5D"]
    );
    let expected_batch_statuses = if status == "closed" {
        ["closed", "closed", "closed", "closed"]
    } else {
        ["closed", "closed", "closed", "implementation"]
    };
    assert_eq!(
        batches
            .iter()
            .map(|batch| batch["status"].as_str().expect("batch status"))
            .collect::<Vec<_>>(),
        expected_batch_statuses
    );
    for batch in batches {
        assert_nonempty_fields(batch, &["status", "claim", "gate"]);
        if batch["status"].as_str() == Some("closed") {
            assert_nonempty_fields(batch, &["evidence"]);
        } else {
            assert!(
                batch.get("evidence").is_none(),
                "{} carries evidence before closure",
                batch["id"]
            );
        }
    }

    let scenarios = contract["scenario"]
        .as_array()
        .expect("Phase 5 scenarios are an array");
    let expected = (1..=9)
        .map(|number| format!("VS-{number:02}"))
        .collect::<BTreeSet<_>>();
    let actual = scenarios
        .iter()
        .map(|scenario| scenario["id"].as_str().expect("scenario id").to_owned())
        .collect::<BTreeSet<_>>();
    assert_eq!(actual, expected);
    for scenario in scenarios {
        assert_nonempty_fields(scenario, &["owner", "command"]);
    }
}

#[test]
fn phase_5_ledger_is_exact_and_never_claims_proof_without_evidence() {
    let contract = phase_5_contract();
    let ledger = repository_document("_docs/worth-ui/milestone-3.11-phase-5-proof-ledger.csv");
    ledger::validate(&contract, &ledger, false)
        .unwrap_or_else(|failure| panic!("Phase 5 ledger is invalid: {failure}"));
}

#[test]
fn reopened_or_empty_phase_5_evidence_turns_closure_red() {
    let contract = phase_5_contract();
    let ledger = repository_document("_docs/worth-ui/milestone-3.11-phase-5-proof-ledger.csv");
    assert!(
        ledger::validate(&contract, &ledger, true).is_ok(),
        "the checked-in ledger must first satisfy closed posture"
    );

    let reopened = ledger.replacen("\"PROVED\"", "\"OPEN\"", 1);
    assert!(ledger::validate(&contract, &reopened, true).is_err());

    let mut rows = ledger.lines().map(str::to_owned).collect::<Vec<_>>();
    let evidence = rows[1]
        .rfind(",\"")
        .expect("the first proved row has a quoted evidence field");
    rows[1].truncate(evidence);
    rows[1].push_str(",\"\"");
    let empty_evidence = rows.join("\n");
    assert!(ledger::validate(&contract, &empty_evidence, true).is_err());
}

#[test]
fn phase_5a_evidence_is_a_non_authoritative_curated_projection() {
    let inventory = workspace_source_inventory();
    let evidence = inventory.text("crates/worth-ui-inspection/src/receipt/snapshot/evidence.rs");
    for required in [
        "pub struct UiVisualSnapshotEvidence",
        "schema_version:",
        "affinity:",
        "coordinates:",
        "visible_index:",
        "hit_test_index:",
        "artifact:",
        "disclosure:",
        "query_budget:",
        "cost:",
    ] {
        assert!(evidence.contains(required), "evidence misses `{required}`");
    }
    for forbidden in [
        "WorthUiVisualInspectionAuthority",
        "UiVisualGeometryGrant",
        "UiVisualPixelCaptureGrant",
        "UiVisualOverlayGrant",
        "UiMountedVisualSnapshotLease",
        "UiVisualSnapshotResourceLease",
        "worth_ui_runtime",
        "worth_ui_host_contract",
    ] {
        assert!(
            !evidence.contains(forbidden),
            "immutable evidence contains authority or lifecycle field `{forbidden}`"
        );
    }

    let facade = inventory.text("crates/worth-ui/src/facade/inspection.rs");
    assert!(facade.contains("UiVisualSnapshotEvidence,"));
    assert!(
        !facade.contains("UiVisualSnapshotEvidenceInput"),
        "the curated facade must not expose the inter-crate sealing input"
    );
}

#[test]
fn phase_5a_disclosure_and_redaction_are_explicit_production_contracts() {
    let inventory = workspace_source_inventory();
    let disclosure =
        inventory.text("crates/worth-ui-inspection/src/query/visual_snapshot/disclosure.rs");
    for required in [
        "pub struct UiVisualInspectionDisclosure",
        "pub enum UiVisualPixelRedaction",
        "UnredactedSyntheticContent",
        "OpaqueBlack",
    ] {
        assert!(
            disclosure.contains(required),
            "disclosure misses `{required}`"
        );
    }
    let request = inventory.text("crates/worth-ui-inspection/src/query/visual_snapshot/request.rs");
    assert!(
        request.contains(
            "pub fn for_frame(target: Target, disclosure: super::UiVisualInspectionDisclosure)"
        ),
        "frame requests must declare disclosure at construction"
    );
    let entry = inventory.text("crates/worth-ui-runtime/src/facade/entry/visual_snapshot.rs");
    assert!(
        entry.find("grant_scope.disclosure() != request.disclosure()")
            < entry.find("reserve_visual_capture_identity()?"),
        "disclosure denial must precede capture identity and resource effects"
    );
    let pixels =
        inventory.text("crates/worth-ui-inspection/src/receipt/snapshot/pixel_artifact.rs");
    for required in [
        "RedactedNativePresentation",
        "pixel.copy_from_slice(&[0, 0, 0, u8::MAX])",
        "redaction: crate::UiVisualPixelRedaction",
        "retention: UiVisualPixelRetentionDisposition",
    ] {
        assert!(
            pixels.contains(required),
            "pixel posture misses `{required}`"
        );
    }
}

#[test]
fn phase_5b_resources_are_bounded_accounted_and_publicly_projected() {
    let inventory = workspace_source_inventory();
    assert_visual_resource_policy(inventory);
    assert_structural_admission_and_replacement(inventory);
    assert_overlay_and_shutdown_accounting(inventory);
    assert_native_capture_backend(inventory);
    assert_ordinary_and_spatial_costs(inventory);
}

#[test]
fn phase_5c_documentation_is_exact_executable_and_successor_honest() {
    documentation::assert_phase_5c_documentation();
}

#[test]
fn phase_5d_closes_the_spec_roadmap_and_successor_handoff() {
    documentation::assert_phase_5d_documentary_closure();
}

fn assert_visual_resource_policy(inventory: &WorkspaceSourceInventory) {
    let policy =
        inventory.text("crates/worth-ui-inspection/src/query/visual_snapshot/disclosure.rs");
    for required in [
        "pub struct UiVisualInspectionRegionCapacity",
        "maximum_retained_structural_bytes_per_receipt",
        "maximum_retained_structural_bytes_per_session",
        "maximum_visible_region_records",
        "maximum_hit_test_region_records",
    ] {
        assert!(policy.contains(required), "policy misses `{required}`");
    }
}

fn assert_structural_admission_and_replacement(inventory: &WorkspaceSourceInventory) {
    let entry = inventory.text("crates/worth-ui-runtime/src/facade/entry/visual_snapshot.rs");
    let reservation = entry
        .find("host_structural_reservation(")
        .expect("host admission computes structural reservation");
    let identity = entry
        .find("reserve_visual_capture_identity()?")
        .expect("host admission reserves one capture identity");
    let registration = entry
        .find(".register(identity, basis.host_surface, reservation)")
        .expect("host admission registers the exact resource reservation");
    assert!(
        reservation < identity && identity < registration,
        "structure must be bounded before identity and registry effects"
    );

    let lifecycle = inventory.text(
        "crates/worth-ui-runtime/src/inspection/visual_snapshot/registry/resource_lifecycle.rs",
    );
    for required in [
        "pub(crate) fn replace(",
        "replace_retained_total(",
        "prior.usage.pixel_bytes",
        "prior.usage.structural_bytes",
    ] {
        assert!(
            lifecycle.contains(required),
            "resource successor misses `{required}`"
        );
    }
}

fn assert_overlay_and_shutdown_accounting(inventory: &WorkspaceSourceInventory) {
    let overlay =
        inventory.text("crates/worth-ui-runtime/src/inspection/visual_snapshot/overlay/seal.rs");
    let normalized_overlay = overlay.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(overlay.contains("published_overlay_cost"));
    assert!(normalized_overlay.contains("4, 1, retained_structural_bytes"));
    assert!(overlay.contains("from_runtime_projection([0; 11])"));

    let shutdown = inventory.text("apps/platform-pulse/src/observation_contract/lifecycle.rs");
    assert!(shutdown.contains("disposed_visual_structural_bytes"));
    let cleanup = inventory
        .text("apps/platform-pulse/tests/executable_world/adjudication/lifecycle_cleanup.rs");
    assert!(cleanup.contains("shutdown.disposed_visual_structural_bytes()"));
}

fn assert_native_capture_backend(inventory: &WorkspaceSourceInventory) {
    let pulse_manifest = repository_document("workspaces/worth-ui/apps/platform-pulse/Cargo.toml");
    assert!(pulse_manifest
        .contains(r#"eframe = { workspace = true, features = ["wgpu_no_default_features"] }"#));
    assert!(pulse_manifest.contains(r#"xcap = { workspace = true, features = ["wgc"] }"#));
    let pulse_main = inventory.text("apps/platform-pulse/src/main.rs");
    assert!(pulse_main.contains("renderer: eframe::Renderer::Wgpu"));
    let native =
        inventory.text("apps/platform-pulse/tests/executable_world/native_platform/windows.rs");
    assert!(native.contains("capture_window: Window"));
    assert!(native.contains("client_capture::exact_window("));
    let client_capture = inventory.text(
        "apps/platform-pulse/tests/executable_world/native_platform/windows/client_capture.rs",
    );
    assert!(client_capture.contains("window.pid().ok() == Some(process_id)"));
    assert!(client_capture.contains("window.id().ok() == Some(window_id)"));
    assert!(client_capture.contains(".capture_image()"));
}

fn assert_ordinary_and_spatial_costs(inventory: &WorkspaceSourceInventory) {
    let ordinary = inventory.text(
        "crates/worth-ui-certification/tests/application_contracts/cross_lane_bundle_execution.rs",
    );
    assert!(ordinary.contains("for frame in 0..3"));
    assert!(ordinary.contains("visual_inspection_cost().counters()"));
    assert!(ordinary.contains("[0; 11]"));

    let spatial = inventory.text(
        "crates/worth-ui-runtime/src/inspection/visual_snapshot/spatial/tests/cost_bounds.rs",
    );
    assert!(spatial.contains("[1_usize, 1_024, 65_536]"));
}

fn phase_5_contract() -> toml::Value {
    let text = repository_document("_docs/worth-ui/milestone-3.11-phase-5-contract.toml");
    toml::from_str(&text).expect("Phase 5 contract is TOML")
}

fn assert_nonempty_fields(value: &toml::Value, fields: &[&str]) {
    for field in fields {
        assert!(
            value[*field]
                .as_str()
                .is_some_and(|text| !text.trim().is_empty()),
            "{} has no {field}",
            value["id"]
        );
    }
}
