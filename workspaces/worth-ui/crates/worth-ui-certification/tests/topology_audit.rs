use std::path::{Path, PathBuf};

use worth_ui::facade::app::WorthUi;
use worth_ui::facade::inspection::{
    UiInspectionMilestoneExpectation, UiInspectionPosture, UiInspectionQuery, UiInspectionScope,
    UiInspectionSupportReason, UiInspectionSupportStatus, UiInspectionSupportWorld,
    UiInspectionTarget,
};
use worth_ui_certification::topology::{
    audit_consumers_route_inspection_through_worth_ui_facade, audit_evidence_family_storage_homes,
    audit_host_egui_dependency_boundary, audit_host_output_plan_encapsulation,
    audit_inspection_crate_does_not_export_runtime_owned_evidence_surface,
    audit_inspection_future_artifact_seed_topology, audit_inspection_public_module_names,
    audit_inspection_public_module_role_purity, audit_no_cross_crate_deep_imports,
    audit_non_dsl_crates_do_not_reach_dsl_internals, audit_phase3_lifecycle_public_surface,
    audit_preboundary_receipt_and_posture_files_do_not_lower_to_foundational,
    audit_product_lifecycle_facade,
    audit_public_inspection_facades_do_not_export_family_local_records,
    audit_public_surfaces_do_not_recreate_query_owned_lanes,
    audit_required_runtime_lifecycle_aggregates_do_not_cheat_with_default_or_option,
    expected_phase3_lifecycle_subsystems,
};
use worth_ui_host_headless::WorthUiHeadlessHost as HeadlessHost;

#[path = "topology_audit/immutable_inspection.rs"]
mod immutable_inspection;

fn workspace_root() -> &'static worth_ui_certification::topology::WorkspaceSourceInventory {
    super::workspace_source_inventory()
}

fn topology_negative_fixture_root(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/topology_negative")
        .join(name)
}

fn assert_has_violation(
    violations: &[String],
    expected_file_fragment: &str,
    expected_reason_fragment: &str,
) {
    assert!(
        violations.iter().any(|violation| {
            violation.contains(expected_file_fragment)
                && violation.contains(expected_reason_fragment)
        }),
        "expected a violation containing file fragment `{expected_file_fragment}` and reason fragment `{expected_reason_fragment}`;\nactual violations:\n{}",
        violations.join("\n")
    );
}

#[test]
fn host_egui_only_uses_host_contract_surfaces() {
    let violations = audit_host_egui_dependency_boundary(workspace_root());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

#[test]
fn host_egui_boundary_audit_rejects_known_bad_runtime_import_fixture() {
    let inventory = worth_ui_certification::topology::WorkspaceSourceInventory::capture(
        topology_negative_fixture_root("host_egui_forbidden_runtime_import"),
    );
    let violations = audit_host_egui_dependency_boundary(&inventory);
    assert_has_violation(
        &violations,
        "worth-ui-host-egui",
        "reaches worth-ui-runtime internals",
    );
}

#[test]
fn canonical_plan_and_product_facades_keep_host_output_encapsulated() {
    let violations = audit_host_output_plan_encapsulation(workspace_root());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

#[test]
fn product_lifecycle_facade_exposes_observation_not_plan_authority() {
    let violations = audit_product_lifecycle_facade(workspace_root());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

#[test]
fn host_output_encapsulation_audit_rejects_known_plan_and_facade_leaks() {
    let inventory = worth_ui_certification::topology::WorkspaceSourceInventory::capture(
        topology_negative_fixture_root("host_output_plan_leak"),
    );
    let violations = audit_host_output_plan_encapsulation(&inventory);
    assert_has_violation(&violations, "egui_plan_leak.rs", "egui-specific meaning");
    assert_has_violation(&violations, "runtime.rs", "owned execution plan");
}

#[test]
fn no_crate_deep_imports_sibling_internals() {
    let violations = audit_no_cross_crate_deep_imports(workspace_root());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

#[test]
fn non_dsl_crates_only_use_admitted_dsl_boundary_types() {
    let violations = audit_non_dsl_crates_do_not_reach_dsl_internals(workspace_root());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

#[test]
fn non_dsl_audit_rejects_known_bad_dsl_internal_import_fixture() {
    let inventory = worth_ui_certification::topology::WorkspaceSourceInventory::capture(
        topology_negative_fixture_root("non_dsl_deep_import"),
    );
    let violations = audit_non_dsl_crates_do_not_reach_dsl_internals(&inventory);
    assert_has_violation(&violations, "lib.rs", "reaches worth-ui-dsl internals");
}

#[test]
fn public_surfaces_do_not_recreate_query_owned_lanes() {
    let violations = audit_public_surfaces_do_not_recreate_query_owned_lanes(workspace_root());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

#[test]
fn inspection_crate_exposes_no_forbidden_public_module_names() {
    let violations = audit_inspection_public_module_names(workspace_root());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

#[test]
fn inspection_crate_does_not_export_runtime_owned_evidence_surface() {
    let violations =
        audit_inspection_crate_does_not_export_runtime_owned_evidence_surface(workspace_root());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

#[test]
fn inspection_public_module_name_audit_rejects_nested_forbidden_module_fixture() {
    let inventory = worth_ui_certification::topology::WorkspaceSourceInventory::capture(
        topology_negative_fixture_root("inspection_forbidden_nested_public_module"),
    );
    let violations = audit_inspection_public_module_names(&inventory);
    assert_has_violation(
        &violations,
        "nested\\mod.rs",
        "forbidden public module `debug`",
    );
}

#[test]
fn inspection_crate_public_modules_stay_role_pure() {
    let violations = audit_inspection_public_module_role_purity(workspace_root());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

#[test]
fn inspection_role_purity_audit_rejects_mixed_public_module_fixture() {
    let inventory = worth_ui_certification::topology::WorkspaceSourceInventory::capture(
        topology_negative_fixture_root("inspection_role_purity_drift"),
    );
    let violations = audit_inspection_public_module_role_purity(&inventory);
    assert_has_violation(
        &violations,
        "receipt/mod.rs",
        "single public responsibility",
    );
}

#[test]
fn inspection_crate_seeds_private_future_artifact_homes() {
    let violations = audit_inspection_future_artifact_seed_topology(workspace_root());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

#[test]
fn inspection_artifact_seed_audit_rejects_missing_or_public_seed_fixture() {
    let inventory = worth_ui_certification::topology::WorkspaceSourceInventory::capture(
        topology_negative_fixture_root("inspection_missing_artifact_seed_homes"),
    );
    let violations = audit_inspection_future_artifact_seed_topology(&inventory);
    assert_has_violation(
        &violations,
        "receipt/replay/mod.rs",
        "future replay inspection artifacts lack an honest internal home",
    );
    assert_has_violation(
        &violations,
        "receipt/mod.rs",
        "private `replay` child module",
    );
}

#[test]
fn query_lane_audit_rejects_nested_public_surface_fixture() {
    let inventory = worth_ui_certification::topology::WorkspaceSourceInventory::capture(
        topology_negative_fixture_root("public_query_lane_recreation"),
    );
    let violations = audit_public_surfaces_do_not_recreate_query_owned_lanes(&inventory);
    assert_has_violation(
        &violations,
        "nested.rs",
        "recreates a Query-owned support/async-result/inspection/causal-explanation/projection-fact lane",
    );
}

#[test]
fn consumers_route_inspection_through_the_worth_ui_facade() {
    let violations = audit_consumers_route_inspection_through_worth_ui_facade(workspace_root());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

#[test]
fn evidence_families_keep_owner_local_record_homes() {
    let violations = audit_evidence_family_storage_homes(workspace_root());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

#[test]
fn public_inspection_facades_do_not_export_family_local_records() {
    let violations =
        audit_public_inspection_facades_do_not_export_family_local_records(workspace_root());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

#[test]
fn consumer_bypass_audit_rejects_known_bad_fixture() {
    let inventory = worth_ui_certification::topology::WorkspaceSourceInventory::capture(
        topology_negative_fixture_root("inspection_facade_bypass_consumer"),
    );
    let violations = audit_consumers_route_inspection_through_worth_ui_facade(&inventory);
    assert_has_violation(
        &violations,
        "fake-inspection-consumer\\Cargo.toml",
        "depends on `worth-ui-runtime` directly",
    );
    assert_has_violation(
        &violations,
        "src\\lib.rs",
        "must enter through worth_ui::facade",
    );
}

#[test]
fn preboundary_receipt_and_posture_files_stay_out_of_foundational() {
    let violations =
        audit_preboundary_receipt_and_posture_files_do_not_lower_to_foundational(workspace_root());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

#[test]
fn foundational_lowering_audit_rejects_known_bad_preboundary_fixture() {
    let inventory = worth_ui_certification::topology::WorkspaceSourceInventory::capture(
        topology_negative_fixture_root("preboundary_foundational_lowering"),
    );
    let violations =
        audit_preboundary_receipt_and_posture_files_do_not_lower_to_foundational(&inventory);
    assert_has_violation(
        &violations,
        "closure_posture.rs",
        "lowers runtime-local receipt/support/posture truth into worth_foundational before a real boundary",
    );
}

#[test]
fn phase3_lifecycle_public_surface_is_curated() {
    let violations = audit_phase3_lifecycle_public_surface(workspace_root());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

#[test]
fn required_runtime_lifecycle_aggregates_do_not_cheat_with_default_or_option() {
    let violations =
        audit_required_runtime_lifecycle_aggregates_do_not_cheat_with_default_or_option(
            workspace_root(),
        );
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

#[test]
fn lifecycle_aggregate_audit_rejects_known_bad_default_and_option_fixture() {
    let inventory = worth_ui_certification::topology::WorkspaceSourceInventory::capture(
        topology_negative_fixture_root("lifecycle_aggregate_cheat"),
    );
    let violations =
        audit_required_runtime_lifecycle_aggregates_do_not_cheat_with_default_or_option(&inventory);
    assert_has_violation(
        &violations,
        "worth_ui_runtime_bootstrap.rs",
        "default required lifecycle state",
    );
    assert_has_violation(
        &violations,
        "worth_ui_runtime_bootstrap.rs",
        "weaken required lifecycle state with Option/map storage",
    );
}

#[test]
fn lifecycle_inventories_match_phase3_closure_inventory() {
    let app = WorthUi::app()
        .bind_certification_host_adapter(
            worth_ui_host_contract::UiCertificationHostBindingGrant::for_certification(),
            HeadlessHost,
        )
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .freeze()
        .expect("application preparation should succeed");
    let expected = expected_phase3_lifecycle_subsystems();

    let runtime_rows: Vec<_> = app
        .runtime_support_inventory()
        .rows()
        .iter()
        .map(|row| row.subsystem())
        .collect();
    let inspection_rows: Vec<_> = app
        .inspection_closure_report()
        .rows()
        .iter()
        .map(|row| {
            (
                row.subsystem(),
                row.scope(),
                row.status(),
                row.reason(),
                row.expected_in(),
            )
        })
        .collect();
    let expected_inspection_rows: Vec<_> = expected
        .iter()
        .flat_map(|subsystem| {
            [
                (
                    *subsystem,
                    UiInspectionScope::Graph,
                    UiInspectionSupportStatus::Unsupported,
                    Some(UiInspectionSupportReason::BelongsArchitecturallyNotYetAdmitted),
                    Some(UiInspectionMilestoneExpectation::Milestone31),
                ),
                (
                    *subsystem,
                    UiInspectionScope::Measurement,
                    UiInspectionSupportStatus::Unsupported,
                    Some(UiInspectionSupportReason::BelongsArchitecturallyNotYetAdmitted),
                    Some(UiInspectionMilestoneExpectation::Milestone31),
                ),
                (
                    *subsystem,
                    UiInspectionScope::Planning,
                    UiInspectionSupportStatus::Supported,
                    None,
                    None,
                ),
                (
                    *subsystem,
                    UiInspectionScope::Mounting,
                    UiInspectionSupportStatus::Unsupported,
                    Some(UiInspectionSupportReason::BelongsArchitecturallyNotYetAdmitted),
                    Some(UiInspectionMilestoneExpectation::Milestone31),
                ),
                (
                    *subsystem,
                    UiInspectionScope::Rebind,
                    UiInspectionSupportStatus::Unsupported,
                    Some(UiInspectionSupportReason::BelongsArchitecturallyNotYetAdmitted),
                    Some(UiInspectionMilestoneExpectation::Milestone31),
                ),
            ]
        })
        .collect();

    assert_eq!(runtime_rows, expected);
    assert_eq!(inspection_rows, expected_inspection_rows);
}
