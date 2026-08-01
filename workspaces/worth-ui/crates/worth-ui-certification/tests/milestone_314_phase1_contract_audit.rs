use super::{milestone_314_ledger, repository_document, workspace_source_inventory};

#[path = "milestone_314_phase1_contract_audit/phase_ledger.rs"]
mod phase_ledger;

fn phase_1_inputs() -> (toml::Value, String) {
    let contract_text = repository_document("_docs/worth-ui/milestone-3.14-phase-1-contract.toml");
    let contract = toml::from_str(&contract_text).expect("Phase 1 contract should parse");
    let ledger = repository_document("_docs/worth-ui/milestone-3.14-proof-ledger.csv");
    (contract, ledger)
}

fn assert_rejected(contract: &toml::Value, ledger: &str, phase: i64, label: &str) {
    assert!(
        milestone_314_ledger::validate_at_phase(contract, ledger, phase).is_err(),
        "{label} mutation should be rejected"
    );
}

#[test]
fn milestone_314_contract_and_current_implementation_ledger_are_exact() {
    let (contract, ledger) = phase_1_inputs();
    milestone_314_ledger::validate_at_phase(
        &contract,
        &ledger,
        milestone_314_ledger::CURRENT_IMPLEMENTATION_PHASE,
    )
    .expect("the frozen portfolio and current milestone closures should agree");
}

#[test]
fn milestone_314_phase_1_rejects_any_closed_row() {
    let (contract, ledger) = phase_1_inputs();
    let mut rows = milestone_314_ledger::parse_ledger(&ledger).expect("ledger should parse");
    rows[1][8] = "PROVED".to_owned();
    rows[1][9] =
        "A deliberately substantial but premature Phase 2 proof may not close during Phase 1."
            .to_owned();
    assert_rejected(
        &contract,
        &milestone_314_ledger::render_ledger(&rows),
        1,
        "premature Phase 2 proof",
    );
}

#[test]
fn milestone_314_ledger_rejects_hostile_structure_mutations() {
    let (contract, ledger) = phase_1_inputs();
    let rows = milestone_314_ledger::parse_ledger(&ledger).expect("ledger should parse");

    let mut missing = rows.clone();
    missing.remove(4);
    assert_rejected(
        &contract,
        &milestone_314_ledger::render_ledger(&missing),
        1,
        "missing row",
    );

    let mut duplicate = rows.clone();
    duplicate[4] = duplicate[3].clone();
    assert_rejected(
        &contract,
        &milestone_314_ledger::render_ledger(&duplicate),
        1,
        "duplicate row",
    );

    let mut reordered = rows.clone();
    reordered.swap(4, 5);
    assert_rejected(
        &contract,
        &milestone_314_ledger::render_ledger(&reordered),
        1,
        "reordered row",
    );

    let mut fabricated = rows;
    fabricated[4][0] = "IA-99".to_owned();
    assert_rejected(
        &contract,
        &milestone_314_ledger::render_ledger(&fabricated),
        1,
        "fabricated row",
    );
}

#[test]
fn milestone_314_ledger_rejects_command_and_evidence_drift() {
    let (contract, ledger) = phase_1_inputs();
    let rows = milestone_314_ledger::parse_ledger(&ledger).expect("ledger should parse");

    let mut command_drift = rows.clone();
    command_drift[0][7].push_str(" --ignored");
    assert_rejected(
        &contract,
        &milestone_314_ledger::render_ledger(&command_drift),
        1,
        "command drift",
    );

    let mut fabricated_evidence = rows;
    fabricated_evidence[8][9] = "source=fiction; result=passed".to_owned();
    assert_rejected(
        &contract,
        &milestone_314_ledger::render_ledger(&fabricated_evidence),
        1,
        "evidence on an open row",
    );
}

#[test]
fn milestone_314_contract_freezes_native_and_query_boundaries() {
    let (contract, _) = phase_1_inputs();
    assert_eq!(
        contract["native_reachability"]["application_seam"].as_str(),
        Some("eframe::App::raw_input_hook")
    );
    assert_eq!(
        contract["native_reachability"]["eframe_version"].as_str(),
        Some("0.31.1")
    );
    assert_eq!(
        contract["ownership"]["raw_query_owner"].as_str(),
        Some("worth-ui-query-binding")
    );
    assert_eq!(
        contract["execution"]["ui_admission_is_query_admission"].as_bool(),
        Some(false)
    );
    assert_eq!(
        contract["public_contract"]["string_intent_authority"].as_bool(),
        Some(false)
    );
    assert_eq!(
        contract["public_contract"]["provider_create_phase"].as_integer(),
        Some(4)
    );
    assert_eq!(
        contract["public_contract"]["accepted_interactions_are_nonempty_canonical"].as_bool(),
        Some(true)
    );
}

#[test]
fn milestone_314_phase_1_owner_paths_exist_without_demanding_later_placeholders() {
    let (contract, _) = phase_1_inputs();
    let inventory = workspace_source_inventory();
    let required_paths = contract["required_path"]
        .as_array()
        .expect("required paths should be an array");

    for required in required_paths {
        let create_phase = required["create_phase"]
            .as_integer()
            .expect("required path should declare its creation phase");
        assert!(
            (1..=4).contains(&create_phase),
            "required path creation phase should be part of 3.14"
        );
        if create_phase == 1 {
            let repository_path = required["path"].as_str().expect("required path");
            let workspace_path = repository_path
                .strip_prefix("workspaces/worth-ui/")
                .expect("required path should belong to the Worth UI workspace");
            assert!(
                inventory.root().join(workspace_path).exists(),
                "Phase 1 owner should exist: {repository_path}"
            );
        }
    }
}

#[test]
fn milestone_314_native_dependency_versions_remain_exact_historical_evidence() {
    let (contract, _) = phase_1_inputs();
    assert_eq!(
        contract["native_reachability"]["egui_version"].as_str(),
        Some("0.31.1")
    );
    assert_eq!(
        contract["native_reachability"]["eframe_version"].as_str(),
        Some("0.31.1")
    );
}

#[test]
fn milestone_314_production_ingress_and_protocol_evolution_are_exact() {
    let (contract, _) = phase_1_inputs();
    assert_eq!(
        contract["native_reachability"]["lifecycle_schema_version"].as_integer(),
        Some(4)
    );
    assert_eq!(
        contract["native_reachability"]["inherited_lifecycle_schema_versions"]
            .as_array()
            .map(|versions| versions
                .iter()
                .filter_map(toml::Value::as_integer)
                .collect::<Vec<_>>()),
        Some(vec![2, 3])
    );
    assert_pulse_raw_input_successor();
    assert_installed_egui_translation();
}

fn assert_pulse_raw_input_successor() {
    let inventory = workspace_source_inventory();
    let native_frame = inventory.text("apps/platform-pulse/src/native_frame.rs");
    for required in ["fn raw_input_hook(", ".native_input", ".observe("] {
        assert!(
            native_frame.contains(required),
            "Pulse native ingress is missing `{required}`"
        );
    }
    let pulse_input = inventory.text("apps/platform-pulse/src/native_frame/input.rs");
    for required in [
        "host.observe_native_input(raw_input)",
        "UiEguiRawInputIngressOutcome::Stopped",
        "fn publish_discovered(",
        "PlatformPulseNativeInputIngressPosture::Retained",
        "PlatformPulseNativeInputIngressPosture::Stopped",
        ".native_input_reached(reached, posture)",
    ] {
        assert!(
            pulse_input.contains(required),
            "Pulse input module lost `{required}`"
        );
    }
    let first_frame = inventory.text("apps/platform-pulse/src/native_frame/first_frame.rs");
    assert!(first_frame.contains("self.native_input.arm_after_first_frame()"));
    let query = inventory.text("apps/platform-pulse/src/native_frame/query.rs");
    assert!(query.contains("self.publish_first_frame(&source, mounted)"));
}

fn assert_installed_egui_translation() {
    let inventory = workspace_source_inventory();
    let ingress =
        inventory.text("crates/worth-ui-host-egui/src/adapter/input_observation/reachability.rs");
    for required in [
        "egui::Event::PointerButton",
        "egui::Event::Key",
        "egui::Event::Text",
        "egui::ImeEvent::Preedit",
        "egui::ImeEvent::Commit",
        "text.is_empty()",
    ] {
        assert!(
            ingress.contains(required),
            "egui ingress does not expose `{required}`"
        );
    }
    let translation =
        inventory.text("crates/worth-ui-host-egui/src/adapter/input_observation/translation.rs");
    for required in [
        "struct UiEguiInstalledInputTranslators",
        "UiEguiRawInputIngressStopReason::UnsupportedEvent",
        "UiEguiRawInputIngressStopReason::TranslatorUnavailable",
    ] {
        assert!(
            translation.contains(required),
            "installed egui translation lost `{required}`"
        );
    }
}

#[test]
fn milestone_314_pulse_query_edges_remain_audience_safe() {
    let config: toml::Value = toml::from_str(&repository_document(
        "tools/boundary-check/config/road1.toml",
    ))
    .expect("boundary configuration should parse");
    let pulse = config["source_dependency_allowlists"]
        .as_array()
        .expect("source dependency allowlists")
        .iter()
        .find(|rule| {
            rule["sources"].as_array().is_some_and(|sources| {
                sources
                    .iter()
                    .any(|source| source.as_str() == Some("worth-ui-platform-pulse"))
            })
        })
        .expect("Pulse dependency allowlist");
    let allowed = pulse["allowed_targets"]
        .as_array()
        .expect("Pulse targets")
        .iter()
        .filter_map(toml::Value::as_str)
        .collect::<Vec<_>>();

    for required in ["worth-query-decl", "worth-query-host"] {
        assert!(allowed.contains(&required), "missing safe edge {required}");
    }
    for forbidden in [
        "worth-query",
        "worth-query-replay",
        "worth-ui-query-binding",
        "worth-ui-runtime",
        "worth-ui-dsl",
        "worth-ui-certification",
        "worth-ui-test-support",
    ] {
        assert!(
            !allowed.contains(&forbidden),
            "Pulse admits forbidden dependency {forbidden}"
        );
    }
}

#[test]
fn milestone_314_phase_1_has_no_placeholder_or_generic_host_authority_residue() {
    let inventory = workspace_source_inventory();
    let roots = [
        "crates/worth-ui-runtime/src",
        "crates/worth-ui-host-contract/src",
        "crates/worth-ui-host-egui/src",
        "crates/worth-ui-dsl/src",
        "apps/platform-pulse/src",
    ];
    let forbidden = [
        "CommandRuntimeIntentBinding",
        "CommandReadinessBinding",
        "WorthUiHostCapabilityReport::from_contract",
    ];
    for root in roots {
        for source in inventory.rust_files_under(root) {
            for token in forbidden {
                assert!(
                    !source.text().contains(token),
                    "{} retains forbidden `{token}`",
                    source.relative_path().display()
                );
            }
        }
    }
}

#[test]
fn milestone_314_runtime_host_and_dsl_have_no_production_query_authority_edge() {
    let inventory = workspace_source_inventory();
    for manifest_path in [
        "crates/worth-ui-runtime/Cargo.toml",
        "crates/worth-ui-host-contract/Cargo.toml",
        "crates/worth-ui-host-egui/Cargo.toml",
        "crates/worth-ui-dsl/Cargo.toml",
    ] {
        let manifest: toml::Value =
            toml::from_str(inventory.text(manifest_path)).expect("crate manifest should parse");
        let dependencies = manifest.get("dependencies").map(|dependencies| {
            dependencies
                .as_table()
                .expect("production dependencies should be a table")
        });
        for forbidden in [
            "worth-query",
            "worth-query-decl",
            "worth-query-host",
            "worth-query-replay",
        ] {
            assert!(
                dependencies.is_none_or(|dependencies| !dependencies.contains_key(forbidden)),
                "{manifest_path} imports forbidden production authority {forbidden}"
            );
        }
    }
}
