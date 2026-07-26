use std::{collections::BTreeMap, path::Path};
use worth_ui_dsl::{
    WorthUiRustAuthoredArtifactInput, WorthUiRustAuthoredArtifactInputModule, WorthUiSourceModuleId,
};

use crate::facade::{WorthUi, WorthUiApp};
use crate::runtime::replacement::candidate::rust_authored_replacement_candidate;
use crate::runtime::{
    WorthUiAdmittedReplacementCandidate, WorthUiCandidateAdmission, WorthUiReplacementCause,
    WorthUiRuntime, WorthUiRuntimeArtifactComparisonOutcome, WorthUiRuntimeDiagnosticPolicy,
    WorthUiRuntimeLaunch,
};
use crate::source::{
    WorthUiArtifact, WorthUiArtifactHandle, WorthUiArtifactIdentitySeed,
    WorthUiArtifactImportHandle, WorthUiArtifactImportNode, WorthUiArtifactModule,
    WorthUiArtifactNode, WorthUiBindingSemanticsLowerer, WorthUiCanonicalArtifactAssembler,
    WorthUiDurableStateEligibility, WorthUiDurableStateIneligibilityReason,
    WorthUiIdentitySeedLowerer, WorthUiStructuralLegalityLowerer,
};

#[test]
fn same_artifact_equivalence_basis_produces_same_runtime_comparison() {
    let app = WorthUi::app()
        .freeze()
        .expect("application preparation should succeed");
    let runtime = launch_runtime(&app, import_artifact(["app/panels/inspector.wui"]));
    let left = admitted_candidate(&app, &runtime, ["app/panels/inspector.wui"]);
    let right = admitted_candidate(&app, &runtime, ["app/panels/inspector.wui"]);

    let left_comparison = runtime
        .compare_admitted_replacement(&left)
        .expect("left candidate compares");
    let right_comparison = runtime
        .compare_admitted_replacement(&right)
        .expect("right candidate compares");

    assert_eq!(left_comparison, right_comparison);
    assert_eq!(
        left_comparison.outcome(),
        WorthUiRuntimeArtifactComparisonOutcome::EquivalentNoOp
    );
    assert_eq!(left_comparison.counters().artifact_comparisons(), 1);
    assert_eq!(left_comparison.counters().impact_narrowing_attempts(), 0);
    assert_eq!(left_comparison.counters().plan_lowering_attempts(), 0);
    assert_eq!(
        left_comparison.artifact_equivalence().basis(),
        left_comparison.runtime_basis().artifact_equivalence_basis()
    );
    assert!(left_comparison.artifact_equivalence().is_equivalent());
}

#[test]
fn diagnostic_richness_does_not_change_runtime_artifact_comparison() {
    let app = WorthUi::app()
        .freeze()
        .expect("application preparation should succeed");
    let minimal_runtime = launch_runtime_with_diagnostics(
        &app,
        import_artifact(["app/panels/inspector.wui"]),
        WorthUiRuntimeDiagnosticPolicy::minimal(),
    );
    let rich_runtime = launch_runtime_with_diagnostics(
        &app,
        import_artifact(["app/panels/inspector.wui"]),
        WorthUiRuntimeDiagnosticPolicy::rich(),
    );
    let minimal_candidate = admitted_candidate_with_cause(
        &app,
        &minimal_runtime,
        ["app/panels/inspector.wui"],
        WorthUiReplacementCause::manual_refresh(7),
    );
    let rich_candidate = admitted_candidate_with_cause(
        &app,
        &rich_runtime,
        ["app/panels/inspector.wui"],
        WorthUiReplacementCause::manual_refresh(9001),
    );
    assert_ne!(
        minimal_candidate.candidate().provenance_handle(),
        rich_candidate.candidate().provenance_handle()
    );

    let minimal_comparison = minimal_runtime
        .compare_admitted_replacement(&minimal_candidate)
        .expect("minimal diagnostic comparison");
    let rich_comparison = rich_runtime
        .compare_admitted_replacement(&rich_candidate)
        .expect("rich diagnostic comparison");

    assert_eq!(minimal_comparison, rich_comparison);
    assert_eq!(
        rich_comparison.outcome(),
        WorthUiRuntimeArtifactComparisonOutcome::EquivalentNoOp
    );
}

#[test]
fn runtime_comparison_consumes_canonical_meaning_not_authored_source_order() {
    let app = WorthUi::app()
        .freeze()
        .expect("application preparation should succeed");
    let file_artifact = file_authored_import_artifact("app/panels/inspector.wui");
    let runtime = launch_runtime(&app, file_artifact);
    let rust_candidate_artifact =
        rust_authored_reordered_import_artifact("app/panels/inspector.wui");
    let candidate = admitted_rust_candidate_from_artifact(
        &app,
        &runtime,
        rust_candidate_artifact,
        WorthUiReplacementCause::rust_authored_input_change(99),
    );

    let comparison = runtime
        .compare_admitted_replacement(&candidate)
        .expect("canonicalized authored-order candidate compares");

    assert_eq!(
        comparison.outcome(),
        WorthUiRuntimeArtifactComparisonOutcome::EquivalentNoOp
    );
    assert_eq!(
        comparison.active_artifact_digest(),
        comparison.candidate_artifact_digest()
    );
    assert_eq!(comparison.counters().artifact_comparisons(), 1);
    assert_eq!(comparison.counters().impact_narrowing_attempts(), 0);
    assert_eq!(comparison.counters().plan_lowering_attempts(), 0);
    assert_eq!(
        candidate.candidate().authoring_lane().as_str(),
        "rust-authored"
    );
}

#[test]
fn meaningful_artifact_difference_classified_before_impact_narrowing() {
    let app = WorthUi::app()
        .freeze()
        .expect("application preparation should succeed");
    let runtime = launch_runtime(&app, import_artifact(["app/panels/inspector.wui"]));
    let candidate = admitted_candidate(&app, &runtime, ["app/panels/settings.wui"]);

    let comparison = runtime
        .compare_admitted_replacement(&candidate)
        .expect("different candidate compares");

    assert_eq!(
        comparison.outcome(),
        WorthUiRuntimeArtifactComparisonOutcome::MeaningfullyDifferent
    );
    assert!(comparison
        .artifact_equivalence()
        .first_difference()
        .is_some());
    assert_eq!(comparison.counters().artifact_comparisons(), 1);
    assert_eq!(comparison.counters().impact_narrowing_attempts(), 0);
    assert_eq!(comparison.counters().plan_lowering_attempts(), 0);
}

fn admitted_candidate<const N: usize>(
    app: &WorthUiApp,
    runtime: &WorthUiRuntime,
    targets: [&str; N],
) -> WorthUiAdmittedReplacementCandidate {
    admitted_candidate_with_cause(
        app,
        runtime,
        targets,
        WorthUiReplacementCause::manual_refresh(7),
    )
}

fn admitted_candidate_with_cause<const N: usize>(
    app: &WorthUiApp,
    runtime: &WorthUiRuntime,
    targets: [&str; N],
    cause: WorthUiReplacementCause,
) -> WorthUiAdmittedReplacementCandidate {
    let candidate = rust_authored_replacement_candidate(
        import_artifact(targets),
        app.capabilities().digest(),
        cause,
    )
    .expect("candidate seals");
    WorthUiCandidateAdmission::for_active_basis(runtime.replacement_admission_basis())
        .admit(candidate)
        .expect("candidate admits")
}

fn admitted_rust_candidate_from_artifact(
    app: &WorthUiApp,
    runtime: &WorthUiRuntime,
    artifact: WorthUiArtifact,
    cause: WorthUiReplacementCause,
) -> WorthUiAdmittedReplacementCandidate {
    let candidate =
        rust_authored_replacement_candidate(artifact, app.capabilities().digest(), cause)
            .expect("rust-authored candidate seals");
    WorthUiCandidateAdmission::for_active_basis(runtime.replacement_admission_basis())
        .admit(candidate)
        .expect("candidate admits")
}

fn file_authored_import_artifact(target_module_path: &str) -> WorthUiArtifact {
    canonical_artifact_from_input(crate::source::test_compilation::compile_source([
        ("app/main.wui", format!(r#"import "{target_module_path}";"#)),
        (target_module_path, String::new()),
    ]))
}

fn rust_authored_reordered_import_artifact(target_module_path: &str) -> WorthUiArtifact {
    canonical_artifact_from_input(crate::source::test_compilation::compile_rust_authored(
        &WorthUiRustAuthoredArtifactInput::from_modules([
            WorthUiRustAuthoredArtifactInputModule::new(target_module_path),
            WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
                .with_import(target_module_path),
        ]),
    ))
}

fn canonical_artifact_from_input(
    artifact_input: worth_ui_dsl::WorthUiSealedSemanticPackage,
) -> WorthUiArtifact {
    let app = WorthUi::app()
        .freeze()
        .expect("application preparation should succeed");
    let snapshot = app.capabilities();
    let resolved = crate::source::WorthUiArtifactInputResolver::resolve(&artifact_input, snapshot)
        .expect("artifact input resolves");
    let structured =
        WorthUiStructuralLegalityLowerer::lower(&resolved, snapshot).expect("structure lowers");
    let bound = WorthUiBindingSemanticsLowerer::lower(&structured, snapshot)
        .expect("binding semantics lower");
    let identity_seeded = WorthUiIdentitySeedLowerer::lower(&bound)
        .expect("identity seeds lower")
        .0;
    WorthUiCanonicalArtifactAssembler::assemble(&identity_seeded)
        .expect("canonical artifact assembles")
}

fn launch_runtime(
    app: &WorthUiApp,
    artifact: WorthUiArtifact,
) -> crate::runtime::WorthUiRuntimeFrameworkLoop {
    launch_runtime_with_diagnostics(app, artifact, WorthUiRuntimeDiagnosticPolicy::minimal())
}

fn launch_runtime_with_diagnostics(
    app: &WorthUiApp,
    artifact: WorthUiArtifact,
    diagnostic_policy: WorthUiRuntimeDiagnosticPolicy,
) -> crate::runtime::WorthUiRuntimeFrameworkLoop {
    app.launch_runtime(
        WorthUiRuntimeLaunch::from_canonical_artifact(artifact).with_diagnostics(diagnostic_policy),
    )
    .expect("runtime launches")
}

fn import_artifact<const N: usize>(targets: [&str; N]) -> WorthUiArtifact {
    let module_id = module_id("app/main.wui");
    let nodes = targets
        .into_iter()
        .enumerate()
        .map(|(node_index, target)| import_node(&module_id, node_index, target))
        .collect::<Vec<_>>();
    let module = WorthUiArtifactModule::new(module_id.clone(), nodes);
    let mut modules = BTreeMap::new();
    modules.insert(module_id.clone(), module);

    WorthUiArtifact::new(modules, vec![module_id])
}

fn import_node(
    module_id: &WorthUiSourceModuleId,
    node_index: usize,
    target: &str,
) -> WorthUiArtifactNode {
    WorthUiArtifactNode::Import(WorthUiArtifactImportNode::new(
        WorthUiArtifactHandle::Import(WorthUiArtifactImportHandle::new(
            module_id.clone(),
            node_index,
        )),
        crate::source::test_compilation::semantic_import(target)
            .target()
            .clone(),
        0,
        WorthUiArtifactIdentitySeed::structural_fallback(format!(
            "module:{}|import:{}",
            module_id.as_str(),
            target
        )),
        WorthUiDurableStateEligibility::Ineligible {
            reason: WorthUiDurableStateIneligibilityReason::NoDurableStateSurface,
        },
    ))
}

fn module_id(path: &str) -> WorthUiSourceModuleId {
    WorthUiSourceModuleId::from_relative_path(Path::new(path)).expect("valid module id")
}
