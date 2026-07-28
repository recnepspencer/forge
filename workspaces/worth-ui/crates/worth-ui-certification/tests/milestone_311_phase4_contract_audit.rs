use std::collections::BTreeSet;

use crate::{repository_document, workspace_source_inventory};

#[test]
fn phase_4_scenario_contracts_have_every_required_evidence_field() {
    let contract = phase_4_contract();
    assert_eq!(contract["status"].as_str(), Some("closed"));
    let scenarios = contract["scenario"]
        .as_array()
        .expect("Phase 4 scenarios are an array");
    let ids = scenarios
        .iter()
        .map(|scenario| scenario["id"].as_str().expect("scenario id"))
        .collect::<BTreeSet<_>>();
    assert_eq!(ids, BTreeSet::from(["VS-01", "VS-06", "VS-07"]));
    for scenario in scenarios {
        for field in REQUIRED_SCENARIO_TEXT_FIELDS {
            assert!(
                scenario[field]
                    .as_str()
                    .is_some_and(|value| !value.trim().is_empty()),
                "{} has no {field}",
                scenario["id"]
            );
        }
        assert!(
            scenario["mutation_control"]
                .as_array()
                .is_some_and(|controls| controls.len() >= 4),
            "{} needs at least four mutation controls",
            scenario["id"]
        );
    }
}

#[test]
fn phase_4_ledger_has_one_complete_proved_row_set() {
    let ledger = repository_document("_docs/worth-ui/milestone-3.11-phase-4-proof-ledger.csv");
    let mut ids = BTreeSet::new();
    let mut rows = 0;
    for line in ledger.lines().skip(1) {
        let id = line.split(',').next().expect("ledger row id");
        assert!(ids.insert(id.to_owned()), "duplicate ledger row {id}");
        assert!(
            line.contains(",\"PROVED\","),
            "{id} is not closed as PROVED"
        );
        assert!(!line.ends_with(",\"\""), "{id} is proved without evidence");
        rows += 1;
    }
    assert_eq!(rows, 20);
    assert_eq!(ids.first().map(String::as_str), Some("P4-01"));
    assert_eq!(ids.last().map(String::as_str), Some("P4-20"));
    assert!(!ledger.contains(",\"OPEN\","));

    let final_row = ledger
        .lines()
        .find(|line| line.starts_with("P4-20,"))
        .expect("P4-20 closure row");
    for marker in REQUIRED_FINAL_EVIDENCE_MARKERS {
        assert!(
            final_row.contains(marker),
            "P4-20 is missing final evidence marker {marker}"
        );
    }
}

#[test]
fn executable_world_command_enables_its_manifest_required_feature() {
    let contract = phase_4_contract();
    let executable_scenario = contract["scenario"]
        .as_array()
        .expect("Phase 4 scenarios are an array")
        .iter()
        .find(|scenario| scenario["id"].as_str() == Some("VS-01"))
        .expect("VS-01 scenario exists");
    assert_eq!(
        executable_scenario["command"].as_str(),
        Some(EXECUTABLE_WORLD_COMMAND)
    );

    let manifest = repository_document("workspaces/worth-ui/apps/platform-pulse/Cargo.toml");
    let manifest: toml::Value = toml::from_str(&manifest).expect("platform-pulse manifest is TOML");
    let executable_target = manifest["test"]
        .as_array()
        .expect("platform-pulse tests are an array")
        .iter()
        .find(|target| target["name"].as_str() == Some("executable_world"))
        .expect("executable_world target exists");
    let required_features = executable_target["required-features"]
        .as_array()
        .expect("executable_world required features are an array");
    assert_eq!(required_features.len(), 1);
    assert_eq!(required_features[0].as_str(), Some("executable-world"));
}

#[test]
fn identity_overlay_drawing_is_owned_only_by_the_host_adapter() {
    let inventory = workspace_source_inventory();
    let adapter = inventory.text("crates/worth-ui-host-egui/src/adapter/identity_overlay.rs");
    for required in [
        "UiMountedDiagnosticProjection::IdentityOverlay",
        "mechanic.coordinate_basis()",
        ".layer_painter(",
        "painter.rect_filled(",
    ] {
        assert!(
            adapter.contains(required),
            "egui identity-overlay adapter must retain `{required}`"
        );
    }
    for forbidden_authority in [
        "UiVisualOverlayGrant",
        "UiPendingVisualOverlay",
        "UiPublishedVisualOverlay",
        "UiVisualSnapshotReceipt",
        "worth_ui_runtime",
        "worth_ui_inspection",
    ] {
        assert!(
            !adapter.contains(forbidden_authority),
            "adapter must not own lifecycle or snapshot authority `{forbidden_authority}`"
        );
    }
    for root in [
        "apps/platform-pulse/src",
        "crates/worth-ui-runtime/src",
        "crates/worth-ui/src",
    ] {
        for source in inventory.rust_files_under(root) {
            for forbidden_draw in [".layer_painter(", "painter.rect_filled("] {
                assert!(
                    !source.text().contains(forbidden_draw),
                    "{} draws adapter pixels through `{forbidden_draw}`",
                    source.relative_path().display()
                );
            }
        }
    }
}

fn phase_4_contract() -> toml::Value {
    let text = repository_document("_docs/worth-ui/milestone-3.11-phase-4-contract.toml");
    toml::from_str(&text).expect("Phase 4 contract is TOML")
}

const REQUIRED_SCENARIO_TEXT_FIELDS: &[&str] = &[
    "production_claim",
    "plausible_defect",
    "world",
    "semantic_handles",
    "production_entry",
    "authority_boundary",
    "action_schedule",
    "typed_expected_outcome",
    "consequential_state",
    "independent_oracle",
    "cleanup_disposition",
    "structural_cost",
    "owning_cost_lane",
    "command",
];

const EXECUTABLE_WORLD_COMMAND: &str = "cargo test --manifest-path workspaces/worth-ui/Cargo.toml \
    -p worth-ui-platform-pulse --features executable-world --test executable_world";

const REQUIRED_FINAL_EVIDENCE_MARKERS: &[&str] = &[
    "857 runtime tests",
    "17 focused visual identity application contracts",
    "6 protocol tests",
    "9 executable-world tests",
    "126 topology contracts",
    "35 fail targets",
    "13 pass targets",
    "2 Cargo sessions",
    "workspace all-target all-feature clippy -D warnings",
    "340 Rust files",
    "3417-file WORTH UI Rust line-cap audit",
    "boundary-check",
    "agent-context check",
    "closed 20-row Phase 4 ledger audit",
];
