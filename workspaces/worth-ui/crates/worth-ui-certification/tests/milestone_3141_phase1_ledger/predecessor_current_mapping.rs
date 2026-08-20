use std::collections::BTreeSet;
use std::sync::OnceLock;

use serde_json::Value;
use sha2::{Digest, Sha256};

use super::{row_evidence, schema};

const CURRENT_MAPPING: &str = r#"P1-AFFINITY-01|workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/work_producer.rs::issue_successor|workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/work_producer_tests.rs::one_replacement_carries_one_change_and_exact_predecessor_successor_damage|workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/work_producer.rs;workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/work_producer_tests.rs
P1-AUTHORITY-01|workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/application.rs::complete|workspaces/worth-ui/crates/worth-ui-certification/tests/milestone_3141_phase1_topology/compile_contract_artifact.rs::phase_one_compile_contract_artifact_matches_every_executed_case|workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/application.rs;workspaces/worth-ui/crates/worth-ui-certification/tests/milestone_3141_phase1_topology/compile_contract_artifact.rs;_docs/worth-ui/milestone-3.14.1-evidence/compile-contracts.json;scripts/ci/run_worth_ui_compile_contracts.py
P1-BACKEND-FEATURES-01|workspaces/worth-ui/crates/worth-ui-host-native/src/qualification_tests/qualified_dependencies.rs::assert_qualified_dependencies|workspaces/worth-ui/crates/worth-ui-certification/tests/milestone_3141_phase1_topology/resolved_graphs.rs::default_all_feature_and_windows_resolved_graphs_are_exact_and_mutation_sensitive|workspaces/worth-ui/crates/worth-ui-host-native/src/qualification_tests/qualified_dependencies.rs;workspaces/worth-ui/crates/worth-ui-certification/tests/milestone_3141_phase1_topology/resolved_graphs.rs
P1-BASELINE-01|workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/coordinator/admission.rs::baseline_requirement_denial|workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/coordinator/admission.rs::actual_baseline_registration_gates_the_presentation_admission_transition|workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/coordinator/admission.rs
P1-CLOSE-01|workspaces/worth-ui/crates/worth-ui-certification/tests/milestone_3141_phase1_ledger.rs::validate_phase_closure|workspaces/worth-ui/crates/worth-ui-certification/tests/milestone_3141_phase1_ledger.rs::phase_one_closure_prerequisites_are_final_source|workspaces/worth-ui/crates/worth-ui-certification/tests/milestone_3141_phase1_ledger.rs
P1-CONSUMERS-01|workspaces/worth-ui/crates/worth-ui-host-headless/src/headless_translation/static_paint.rs::validate_protocol|workspaces/worth-ui/crates/worth-ui-host-headless/src/headless_static_paint_tests.rs::validated_agreement_static_paint_consumes_and_mixed_contract_stops_before_consumer|workspaces/worth-ui/crates/worth-ui-host-headless/src/headless_translation/static_paint.rs;workspaces/worth-ui/crates/worth-ui-host-headless/src/headless_static_paint_tests.rs;workspaces/worth-ui/crates/worth-ui-host-contract/src/mounted_frame/protocol.rs;workspaces/worth-ui/crates/worth-ui-host-egui/src/adapter/native_paint.rs;workspaces/worth-ui/crates/worth-ui-host-egui/src/adapter/semantic_text_tests.rs
P1-DAMAGE-01|workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/work_producer/delta_diff.rs::append_damage|workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/work_producer_tests/damage_bounds.rs::replacement_damage_is_clipped_to_predecessor_and_successor_visibility|workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/work_producer/delta_diff.rs;workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/work_producer_tests/damage_bounds.rs
P1-HEADLESS-01|workspaces/worth-ui/crates/worth-ui-host-headless/src/headless_recorder/presentation.rs::apply_work|workspaces/worth-ui/crates/worth-ui-certification/tests/application_contracts/mounted_headless_recorder.rs::real_cross_lane_recording_preserves_exact_unperformed_external_mechanics|workspaces/worth-ui/crates/worth-ui-host-headless/src/headless_recorder/presentation.rs;workspaces/worth-ui/crates/worth-ui-certification/tests/application_contracts/mounted_headless_recorder.rs
P1-HEADLESS-COST-01|workspaces/worth-ui/crates/worth-ui-host-headless/src/headless_recorder/presentation.rs::work_cost|workspaces/worth-ui/crates/worth-ui-certification/tests/application_contracts/host_platform/mod.rs::maximum_overlap_removals_cross_public_runtime_and_headless_with_exact_work|workspaces/worth-ui/crates/worth-ui-host-headless/src/headless_recorder/presentation.rs;workspaces/worth-ui/crates/worth-ui-certification/tests/application_contracts/host_platform/mod.rs;_docs/worth-ui/milestone-3.14.1-evidence/p1-worlds-01.json;scripts/ci/run_worth_ui_shared_ledger_control.py
P1-ORDER-01|workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/projection/frame_storage/presentation_sources.rs::compile|workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/work_producer_tests.rs::equal_layer_total_order_follows_authored_node_order_not_command_identity|workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/projection/frame_storage/presentation_sources.rs;workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/work_producer_tests.rs
P1-ORDER-SOURCE-01|workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/projection/frame_storage/presentation_sources.rs::compile|workspaces/worth-ui/crates/worth-ui-certification/tests/milestone_3141_phase1_topology/compile_contract_artifact.rs::phase_one_compile_contract_artifact_matches_every_executed_case|workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/projection/frame_storage/presentation_sources.rs;workspaces/worth-ui/crates/worth-ui-certification/tests/milestone_3141_phase1_topology/compile_contract_artifact.rs;_docs/worth-ui/milestone-3.14.1-evidence/compile-contracts.json;scripts/ci/run_worth_ui_compile_contracts.py
P1-PLATFORM-AUTHORITY-01|workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/native_platform_binding.rs::issue|workspaces/worth-ui/crates/worth-ui-certification/tests/milestone_3141_phase1_topology/compile_contract_artifact.rs::phase_one_compile_contract_artifact_matches_every_executed_case|workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/native_platform_binding.rs;workspaces/worth-ui/crates/worth-ui-certification/tests/milestone_3141_phase1_topology/compile_contract_artifact.rs;_docs/worth-ui/milestone-3.14.1-evidence/compile-contracts.json;scripts/ci/run_worth_ui_compile_contracts.py
P1-PREPARATION-LIFECYCLE-01|workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/platform/preparation.rs::prepare|workspaces/worth-ui/crates/worth-ui-certification/tests/milestone_3141_phase1_topology.rs::phase_one_product_preparation_is_effect_free_and_host_neutral|workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/platform/preparation.rs;workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/profile.rs;workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/application.rs;workspaces/worth-ui/crates/worth-ui-host-native/src/native_profile.rs;workspaces/worth-ui/crates/worth-ui-native-platform/src/lib.rs;workspaces/worth-ui/crates/worth-ui-certification/tests/milestone_3141_phase1_topology.rs;workspaces/worth-ui/crates/worth-ui-certification/tests/milestone_3141_phase1_topology/preparation_call_graph.rs
P1-PRESENTATION-AUTHORITY-01|workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/authority/work.rs::issue_delta|workspaces/worth-ui/crates/worth-ui-certification/tests/milestone_3141_phase1_topology/compile_contract_artifact.rs::phase_one_compile_contract_artifact_matches_every_executed_case|workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/authority/work.rs;workspaces/worth-ui/crates/worth-ui-certification/tests/milestone_3141_phase1_topology/compile_contract_artifact.rs;_docs/worth-ui/milestone-3.14.1-evidence/compile-contracts.json;scripts/ci/run_worth_ui_compile_contracts.py
P1-PRODUCER-01|workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/work_producer.rs::issue_successor|workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/work_producer_tests.rs::removal_and_insert_carry_exact_identities_vacated_damage_and_total_order|workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/work_producer.rs;workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/work_producer_tests.rs
P1-PRODUCER-COST-01|workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/work_producer.rs::issue_successor|workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/work_producer_tests.rs::unchanged_successor_carries_zero_command_order_and_damage_work|workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/work_producer.rs;workspaces/worth-ui/crates/worth-ui-runtime/src/mounting/presentation/work_producer_tests.rs
P1-PROFILE-01|workspaces/worth-ui/crates/worth-ui-host-native/src/qualification_tests/qualified_dependencies.rs::assert_qualified_dependencies|workspaces/worth-ui/crates/worth-ui-host-native/src/qualification_tests.rs::every_qualified_semantic_and_dependency_pin_matches_the_closed_record|workspaces/worth-ui/crates/worth-ui-host-native/src/qualification_tests/qualified_dependencies.rs;workspaces/worth-ui/crates/worth-ui-host-native/src/qualification_tests.rs
P1-PROTOCOL-01|workspaces/worth-ui/crates/worth-ui-host-contract/src/mounted_frame/presentation_work/delta.rs::from_inert_mechanics|workspaces/worth-ui/crates/worth-ui-certification/tests/milestone_3141_phase1_topology/compile_contract_artifact.rs::phase_one_compile_contract_artifact_matches_every_executed_case|workspaces/worth-ui/crates/worth-ui-host-contract/src/mounted_frame/presentation_work/delta.rs;workspaces/worth-ui/crates/worth-ui-certification/tests/milestone_3141_phase1_topology/compile_contract_artifact.rs;_docs/worth-ui/milestone-3.14.1-evidence/compile-contracts.json;scripts/ci/run_worth_ui_compile_contracts.py
P1-TOPOLOGY-01|workspaces/worth-ui/crates/worth-ui-certification/tests/milestone_3141_phase1_topology/topology_verdict.rs::validate_topology|workspaces/worth-ui/crates/worth-ui-certification/tests/milestone_3141_phase1_topology.rs::phase_one_host_platform_topology_verdict_covers_every_workspace_manifest|workspaces/worth-ui/crates/worth-ui-certification/tests/milestone_3141_phase1_topology/topology_verdict.rs;workspaces/worth-ui/crates/worth-ui-certification/tests/milestone_3141_phase1_topology.rs
P1-WORLDS-01|workspaces/worth-ui/crates/worth-ui-certification/tests/application_contracts/host_platform/world/production.rs::produce_maximum_overlap|workspaces/worth-ui/crates/worth-ui-certification/tests/application_contracts/host_platform/mod.rs::maximum_overlap_removals_cross_public_runtime_and_headless_with_exact_work|workspaces/worth-ui/crates/worth-ui-certification/tests/application_contracts/host_platform/world/production.rs;workspaces/worth-ui/crates/worth-ui-certification/tests/application_contracts/host_platform/mod.rs
P2-APPLICATION-01|workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/application_driver.rs::run|workspaces/worth-ui/apps/platform-pulse/tests/executable_world/courtroom/native_phase2.rs::windows_native_boundary_world_presents_quiesces_and_closes_without_residue|workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/application_driver.rs;workspaces/worth-ui/apps/platform-pulse/tests/executable_world/courtroom/native_phase2.rs;workspaces/worth-ui/crates/worth-ui-certification/tests/milestone_3141_phase1_topology/compile_contract_artifact.rs;_docs/worth-ui/milestone-3.14.1-evidence/p2-world-01.json;scripts/ci/run_worth_ui_shared_ledger_control.py
P2-CLOSE-01|workspaces/worth-ui/crates/worth-ui-host-native/src/native/event_loop/terminal_cleanup.rs::terminal_cleanup_complete|workspaces/worth-ui/apps/platform-pulse/tests/executable_world/courtroom/native_phase2.rs::windows_native_boundary_world_presents_quiesces_and_closes_without_residue|workspaces/worth-ui/crates/worth-ui-host-native/src/native/event_loop/terminal_cleanup.rs;workspaces/worth-ui/apps/platform-pulse/tests/executable_world/courtroom/native_phase2.rs;workspaces/worth-ui/crates/worth-ui-host-native/src/native/event_loop/tests.rs;_docs/worth-ui/milestone-3.14.1-evidence/p2-world-01.json;scripts/ci/run_worth_ui_shared_ledger_control.py
P2-EVENT-LOOP-01|workspaces/worth-ui/crates/worth-ui-host-native/src/native/event_loop/callback_thread.rs::transition|workspaces/worth-ui/apps/platform-pulse/tests/executable_world/courtroom/native_phase2.rs::windows_native_boundary_world_presents_quiesces_and_closes_without_residue|workspaces/worth-ui/crates/worth-ui-host-native/src/native/event_loop/callback_thread.rs;workspaces/worth-ui/apps/platform-pulse/tests/executable_world/courtroom/native_phase2.rs;workspaces/worth-ui/crates/worth-ui-host-native/src/native/event_loop/tests.rs;_docs/worth-ui/milestone-3.14.1-evidence/p2-world-01.json;scripts/ci/run_worth_ui_shared_ledger_control.py
P2-GRAPHICS-01|workspaces/worth-ui/crates/worth-ui-host-native/src/native/graphics/adapter_selection.rs::select_eligible_adapter|workspaces/worth-ui/apps/platform-pulse/tests/executable_world/courtroom/native_phase2.rs::windows_native_boundary_world_presents_quiesces_and_closes_without_residue|workspaces/worth-ui/crates/worth-ui-host-native/src/native/graphics/adapter_selection.rs;workspaces/worth-ui/apps/platform-pulse/tests/executable_world/courtroom/native_phase2.rs;workspaces/worth-ui/crates/worth-ui-host-native/src/native/graphics.rs;_docs/worth-ui/milestone-3.14.1-evidence/p2-world-01.json;scripts/ci/run_worth_ui_shared_ledger_control.py
P2-PIXELS-01|workspaces/worth-ui/apps/platform-pulse/tests/executable_world/native_platform/windows.rs::capture_exposed_client_area|workspaces/worth-ui/apps/platform-pulse/tests/executable_world/courtroom/native_phase2.rs::windows_native_boundary_world_presents_quiesces_and_closes_without_residue|workspaces/worth-ui/apps/platform-pulse/tests/executable_world/native_platform/windows.rs;workspaces/worth-ui/apps/platform-pulse/tests/executable_world/courtroom/native_phase2.rs;_docs/worth-ui/milestone-3.14.1-evidence/p2-world-01.json;scripts/ci/run_worth_ui_shared_ledger_control.py
P2-PORTS-01|workspaces/worth-ui/crates/worth-ui-host-native/src/native/presentation/transaction_state.rs::settle_port_result|workspaces/worth-ui/apps/platform-pulse/tests/executable_world/courtroom/native_phase2.rs::windows_native_boundary_world_presents_quiesces_and_closes_without_residue|workspaces/worth-ui/crates/worth-ui-host-native/src/native/presentation/transaction_state.rs;workspaces/worth-ui/apps/platform-pulse/tests/executable_world/courtroom/native_phase2.rs;workspaces/worth-ui/crates/worth-ui-host-native/src/native/presentation.rs;workspaces/worth-ui/crates/worth-ui-host-native/src/native/event_loop/window_port.rs;workspaces/worth-ui/crates/worth-ui-host-native/src/native/graphics/port.rs;workspaces/worth-ui/crates/worth-ui-host-native/src/native/presentation/port.rs;workspaces/worth-ui/crates/worth-ui-host-native/src/native/presentation/pending_wgpu_readback.rs;workspaces/worth-ui/crates/worth-ui-host-native/src/native/event_loop.rs;_docs/worth-ui/milestone-3.14.1-evidence/p2-world-01.json;scripts/ci/run_worth_ui_shared_ledger_control.py
P2-PRESENT-01|workspaces/worth-ui/crates/worth-ui-host-native/src/native/presentation.rs::present_initial|workspaces/worth-ui/apps/platform-pulse/tests/executable_world/courtroom/native_phase2.rs::windows_native_boundary_world_presents_quiesces_and_closes_without_residue|workspaces/worth-ui/crates/worth-ui-host-native/src/native/presentation.rs;workspaces/worth-ui/apps/platform-pulse/tests/executable_world/courtroom/native_phase2.rs;workspaces/worth-ui/crates/worth-ui-host-native/src/native/presentation/raster.rs;_docs/worth-ui/milestone-3.14.1-evidence/p2-world-01.json;scripts/ci/run_worth_ui_shared_ledger_control.py
P2-READINESS-01|workspaces/worth-ui/crates/worth-ui-host-native/src/native/readiness.rs::commit_latest|workspaces/worth-ui/apps/platform-pulse/tests/executable_world/courtroom/native_phase2.rs::windows_native_boundary_world_presents_quiesces_and_closes_without_residue|workspaces/worth-ui/crates/worth-ui-host-native/src/native/readiness.rs;workspaces/worth-ui/apps/platform-pulse/tests/executable_world/courtroom/native_phase2.rs;_docs/worth-ui/milestone-3.14.1-evidence/p2-world-01.json;scripts/ci/run_worth_ui_shared_ledger_control.py
P2-WINDOW-01|workspaces/worth-ui/crates/worth-ui-host-native/src/native/graphics.rs::basis_changed|workspaces/worth-ui/apps/platform-pulse/tests/executable_world/courtroom/native_phase2.rs::windows_native_boundary_world_presents_quiesces_and_closes_without_residue|workspaces/worth-ui/crates/worth-ui-host-native/src/native/graphics.rs;workspaces/worth-ui/apps/platform-pulse/tests/executable_world/courtroom/native_phase2.rs;_docs/worth-ui/milestone-3.14.1-evidence/p2-world-01.json;scripts/ci/run_worth_ui_shared_ledger_control.py
P2-WORLD-01|workspaces/worth-ui/apps/platform-pulse/src/main.rs::run_native_phase2_world|workspaces/worth-ui/apps/platform-pulse/tests/executable_world/courtroom/native_phase2.rs::windows_native_boundary_world_presents_quiesces_and_closes_without_residue|workspaces/worth-ui/apps/platform-pulse/src/main.rs;workspaces/worth-ui/apps/platform-pulse/tests/executable_world/courtroom/native_phase2.rs;workspaces/worth-ui/crates/worth-ui-certification/tests/milestone_3141_phase1_ledger/result_artifact_mutation.rs;workspaces/worth-ui/apps/platform-pulse/src/native_seed_application.rs"#;

pub(super) struct Mapping<'a> {
    pub(super) production_entry: &'a str,
    pub(super) independent_oracle: &'a str,
    pub(super) source_identity: &'a str,
}

pub(super) fn validate_contract() -> Result<(), String> {
    static VALIDATION: OnceLock<Result<(), String>> = OnceLock::new();
    VALIDATION.get_or_init(validate_contract_once).clone()
}

fn validate_contract_once() -> Result<(), String> {
    let mut requirements = BTreeSet::new();
    for line in CURRENT_MAPPING.lines() {
        let (requirement, _) = line
            .split_once('|')
            .ok_or_else(|| "invalid predecessor current-mapping contract".to_owned())?;
        if !requirements.insert(requirement) {
            return Err(format!(
                "duplicate predecessor current-mapping contract for {requirement}"
            ));
        }
        validate_expected(requirement)?;
    }
    let expected = schema::EXPECTED_REQUIREMENTS[..30]
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    (requirements == expected)
        .then_some(())
        .ok_or_else(|| "predecessor current-mapping contract is incomplete".to_owned())
}

fn validate_expected(requirement: &str) -> Result<(), String> {
    let mapping = expected(requirement)?;
    let sources = mapping.source_identity.split(';').collect::<Vec<_>>();
    if sources.is_empty() || sources.iter().copied().collect::<BTreeSet<_>>().len() != sources.len()
    {
        return Err(format!(
            "{requirement} has invalid required source identities"
        ));
    }
    for entry in [mapping.production_entry, mapping.independent_oracle] {
        row_evidence::validate_named_entry(entry)?;
        let entry_source = entry.rsplit_once("::").map(|entry| entry.0).unwrap();
        if !sources.contains(&entry_source) {
            return Err(format!(
                "{requirement} required sources omit a mapped entry"
            ));
        }
    }
    Ok(())
}

pub(super) fn validate(row: &Value) -> Result<(), String> {
    let requirement = row["requirement"]
        .as_str()
        .ok_or_else(|| "predecessor row omits requirement".to_owned())?;
    let expected = expected(requirement)?;
    require_str(
        row,
        "production_entry",
        expected.production_entry,
        requirement,
    )?;
    require_str(
        row,
        "independent_oracle",
        expected.independent_oracle,
        requirement,
    )?;
    let observed = row["mapping_source_identity"]
        .as_array()
        .and_then(|sources| {
            sources
                .iter()
                .map(Value::as_str)
                .collect::<Option<Vec<_>>>()
        })
        .ok_or_else(|| format!("{requirement} omits mapped source identities"))?;
    let required = expected.source_identity.split(';').collect::<Vec<_>>();
    (observed == required)
        .then_some(())
        .ok_or_else(|| format!("{requirement} has wrong required source identities"))
}

pub(super) fn expected(requirement: &str) -> Result<Mapping<'static>, String> {
    let line = CURRENT_MAPPING
        .lines()
        .find(|line| line.split_once('|').map(|entry| entry.0) == Some(requirement))
        .ok_or_else(|| format!("no current predecessor mapping for {requirement}"))?;
    let mut fields = line.split('|');
    let observed_requirement = fields.next();
    let production_entry = fields.next();
    let independent_oracle = fields.next();
    let source_identity = fields.next();
    match (
        observed_requirement,
        production_entry,
        independent_oracle,
        source_identity,
        fields.next(),
    ) {
        (
            Some(_),
            Some(production_entry),
            Some(independent_oracle),
            Some(source_identity),
            None,
        ) => Ok(Mapping {
            production_entry,
            independent_oracle,
            source_identity,
        }),
        _ => Err(format!(
            "invalid predecessor current-mapping contract for {requirement}"
        )),
    }
}

fn require_str(row: &Value, field: &str, expected: &str, requirement: &str) -> Result<(), String> {
    (row[field].as_str() == Some(expected))
        .then_some(())
        .ok_or_else(|| format!("{requirement} has wrong {field}"))
}

#[test]
fn every_predecessor_mapping_entry_resolves_before_portfolio_execution() {
    validate_contract().unwrap();
}

#[test]
fn phase_two_mapping_digest_matches_the_predecessor_contract() {
    validate_contract().unwrap();
    assert_eq!(
        mapping_digest(),
        super::predecessor_artifact::EXPECTED_MAPPING_DIGEST
    );
}

fn mapping_digest() -> String {
    let mut rows = CURRENT_MAPPING.lines().collect::<Vec<_>>();
    rows.sort_by_key(|line| {
        line.split_once('|')
            .map(|entry| entry.0)
            .unwrap_or_default()
    });
    let mut digest = Sha256::new();
    for row in rows {
        let mut fields = row.split('|');
        for field in fields.by_ref().take(3) {
            digest.update(field.as_bytes());
            digest.update([0]);
        }
        for source in fields.next().unwrap_or_default().split(';') {
            digest.update(source.as_bytes());
            digest.update([0]);
        }
        digest.update([0xff]);
    }
    format!("{:x}", digest.finalize())
}
