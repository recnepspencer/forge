use std::collections::BTreeSet;

use crate::{repository_document, workspace_source_inventory};

#[test]
fn phase_1_contract_names_existing_homes_and_honest_dependency_denials() {
    let contract = phase_1_contract();
    assert_module_homes(&contract);
    assert_scenario_contracts(&contract);
    assert_dependency_denials();
    assert_single_executable_world();
}

fn phase_1_contract() -> toml::Value {
    let text = repository_document("_docs/worth-ui/milestone-3.11-phase-1-contract.toml");
    toml::from_str(&text).expect("Phase 1 contract is TOML")
}

fn assert_module_homes(contract: &toml::Value) {
    let homes = contract["module_home"]
        .as_array()
        .expect("module_home is an array");
    let actual_homes = homes
        .iter()
        .map(|home| {
            (
                home["owner"].as_str().expect("module home owner"),
                home["path"].as_str().expect("module home path"),
            )
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(actual_homes, expected_module_homes());
    for home in homes {
        let path = home["path"].as_str().expect("module home path");
        let workspace_path = path
            .strip_prefix("workspaces/worth-ui/")
            .expect("Phase 1 module homes remain inside the WORTH UI workspace");
        assert!(
            workspace_source_inventory().contains(workspace_path),
            "missing Phase 1 home {path}"
        );
    }
}

fn expected_module_homes() -> BTreeSet<(&'static str, &'static str)> {
    [
        (
            "host mechanics",
            "workspaces/worth-ui/crates/worth-ui-host-contract/src/visual_snapshot/mod.rs",
        ),
        (
            "inspection query meaning",
            "workspaces/worth-ui/crates/worth-ui-inspection/src/query/visual_snapshot/mod.rs",
        ),
        (
            "inspection immutable evidence",
            "workspaces/worth-ui/crates/worth-ui-inspection/src/receipt/snapshot/mod.rs",
        ),
        (
            "runtime authority",
            "workspaces/worth-ui/crates/worth-ui-runtime/src/inspection/visual_snapshot/mod.rs",
        ),
        (
            "egui mechanics",
            "workspaces/worth-ui/crates/worth-ui-host-egui/src/adapter/visual_snapshot/mod.rs",
        ),
        (
            "public workflow",
            "workspaces/worth-ui/crates/worth-ui-runtime/src/facade/entry/visual_snapshot.rs",
        ),
        (
            "public overlay workflow",
            "workspaces/worth-ui/crates/worth-ui-runtime/src/facade/entry/visual_overlay.rs",
        ),
        (
            "permanent pulse scenario",
            "workspaces/worth-ui/apps/platform-pulse/src/visual_identity_pulse.rs",
        ),
        (
            "executable-world progression",
            "workspaces/worth-ui/apps/platform-pulse/tests/executable_world/product_process/visual_snapshot_progression.rs",
        ),
    ]
    .into_iter()
    .collect()
}

fn assert_scenario_contracts(contract: &toml::Value) {
    let scenarios = contract["scenario"]
        .as_array()
        .expect("scenario is an array");
    let ids = scenarios
        .iter()
        .map(|scenario| scenario["id"].as_str().expect("scenario id"))
        .collect::<BTreeSet<_>>();
    assert_eq!(ids, BTreeSet::from(["VS-01", "VS-09"]));
    for scenario in scenarios {
        for field in REQUIRED_SCENARIO_TEXT_FIELDS {
            assert!(
                scenario[field]
                    .as_str()
                    .is_some_and(|value| !value.is_empty()),
                "{} has no {field}",
                scenario["id"]
            );
        }
        assert!(
            scenario["mutation_control"]
                .as_array()
                .is_some_and(|controls| !controls.is_empty()),
            "{} has no mutation controls",
            scenario["id"]
        );
    }
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

fn assert_dependency_denials() {
    assert_manifest_excludes(
        "workspaces/worth-ui/crates/worth-ui-host-contract/Cargo.toml",
        &["worth-ui-runtime", "worth-ui-inspection", "worth-query"],
    );
    assert_manifest_excludes(
        "workspaces/worth-ui/crates/worth-ui-inspection/Cargo.toml",
        &["worth-ui-runtime", "worth-ui-host-egui", "worth-query"],
    );
    assert_manifest_excludes(
        "workspaces/worth-ui/apps/platform-pulse/Cargo.toml",
        &[
            "worth-ui-certification",
            "worth-ui-test-support",
            "worth-query",
        ],
    );
}

fn assert_single_executable_world() {
    let pulse_manifest = workspace_source_inventory().text("apps/platform-pulse/Cargo.toml");
    assert_eq!(pulse_manifest.matches("[[bin]]").count(), 1);
    assert_eq!(pulse_manifest.matches("[[test]]").count(), 1);
    assert_eq!(
        pulse_manifest
            .matches("name = \"executable_world\"")
            .count(),
        1
    );
}

#[test]
fn phase_1_ledger_has_one_complete_proved_row_set() {
    let ledger = repository_document("_docs/worth-ui/milestone-3.11-phase-1-proof-ledger.csv");
    let mut ids = BTreeSet::new();
    let mut rows = 0;
    for line in ledger.lines().skip(1) {
        let id = line.split(',').next().expect("ledger row id");
        assert!(ids.insert(id.to_owned()), "duplicate ledger row {id}");
        assert!(
            line.contains(",\"PROVED\","),
            "Phase 1 ledger row is not proved: {line}"
        );
        rows += 1;
    }
    assert_eq!(rows, 20);
    assert_eq!(ids.first().map(String::as_str), Some("P1-01"));
    assert_eq!(ids.last().map(String::as_str), Some("P1-20"));
    assert!(ledger.contains("P1-20,\"All focused, broad"));
    assert!(!ledger.contains(",\"OPEN\","));
    for required_evidence in [
        "118 topology",
        "161 application",
        "28 fail and 13 pass",
        "6 executable-world",
        "warning-denied clippy",
        "400-line cap",
        "boundary-check",
        "agent-context",
    ] {
        assert!(
            ledger.contains(required_evidence),
            "P1-20 omits closing evidence `{required_evidence}`"
        );
    }
}

fn assert_manifest_excludes(manifest: &str, forbidden: &[&str]) {
    let workspace_manifest = manifest
        .strip_prefix("workspaces/worth-ui/")
        .expect("governed manifests remain inside the WORTH UI workspace");
    let text = workspace_source_inventory().text(workspace_manifest);
    for dependency in forbidden {
        assert!(
            !text.lines().any(|line| {
                line.trim_start().starts_with(&format!("{dependency} "))
                    || line.trim_start().starts_with(&format!("{dependency}="))
            }),
            "{manifest} must not depend on {dependency}"
        );
    }
}
