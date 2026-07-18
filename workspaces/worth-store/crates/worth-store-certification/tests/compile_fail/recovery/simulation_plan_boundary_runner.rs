use std::path::Path;

use worth_store_test_support::compiler_boundary::{
    cargo_dependency_manifest, run_cargo_ui_fixture_suite,
};

#[test]
fn simulation_plan_boundary_rejects_lower_authority_callers_at_compile_time() {
    let root = store_workspace_root();
    let evidence = run_cargo_ui_fixture_suite(
        root,
        "physical-simulation-plan-boundary",
        dependency_manifest(root),
        "production",
        "diagnostic-test",
        &root.join(
            "crates/worth-store-certification/tests/compile_fail/recovery/simulation_plan_boundary",
        ),
        FIXTURES,
    )
    .unwrap();
    assert_eq!(evidence.fixtures.len(), FIXTURES.len());
}

const FIXTURES: &[(&str, &[&str])] = &[
    (
        "certified_scenario_cannot_satisfy_lowered_plan.rs",
        &["PhysicalSimulationPlan", "CertifiedPhysicalScenario"],
    ),
    (
        "lowered_plan_cannot_expose_source_scenario.rs",
        &["scenario"],
    ),
    (
        "lowered_plan_cannot_expose_source_definition.rs",
        &["definition"],
    ),
    ("lowered_plan_cannot_expose_source_family.rs", &["family"]),
    (
        "lowered_plan_cannot_expose_source_expectation.rs",
        &["expectation"],
    ),
    ("lowered_plan_cannot_expose_source_fault.rs", &["fault"]),
    (
        "lowered_plan_cannot_expose_source_schedule.rs",
        &["schedule"],
    ),
    (
        "lowered_plan_cannot_expose_scenario_definition.rs",
        &["scenario_definition"],
    ),
    (
        "plan_struct_literal_cannot_be_minted.rs",
        &["PhysicalSimulationPlan", "private"],
    ),
    (
        "copied_plan_digest_cannot_be_identity.rs",
        &["PhysicalSimulationPlanIdentity", "&str"],
    ),
    (
        "json_value_cannot_satisfy_lowered_plan.rs",
        &["PhysicalSimulationPlan", "Value"],
    ),
    (
        "fixture_label_cannot_satisfy_forbidden_shortcut_set.rs",
        &["ForbiddenShortcutSet", "&str"],
    ),
];

fn dependency_manifest(root: &Path) -> String {
    cargo_dependency_manifest(
        &[(
            "worth-store-physical-certification",
            root.join("crates/worth-store-physical-certification")
                .as_path(),
            &[],
        )],
        &[("serde_json", "1")],
    )
}

fn store_workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("certification crate lives under the Store workspace")
}
