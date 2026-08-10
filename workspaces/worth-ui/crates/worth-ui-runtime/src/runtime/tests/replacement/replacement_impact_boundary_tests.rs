use crate::facade::WorthUi;
use crate::runtime::tests::replacement_impact_test_support::{
    admitted_candidate, artifact_from_modules, component_module, impact_test_app, import_artifact,
    launch_runtime, surface_module, token_module, two_module_import_artifact,
};
use crate::runtime::{
    WorthUiCommandImpact, WorthUiReplacementImpact, WorthUiReplacementImpactDenial,
    WorthUiRuntimeArtifactComparisonOutcome, WorthUiTokenThemeImpact,
    WorthUiUnsupportedReplacementImpact,
};

#[test]
fn equivalent_artifact_changes_classify_to_noop() {
    let app = impact_test_app();
    let artifact = artifact_from_modules(&app, [surface_module("workspace.surface.main")]);
    let runtime = launch_runtime(&app, artifact);
    let candidate = admitted_candidate(
        &app,
        &runtime,
        artifact_from_modules(&app, [surface_module("workspace.surface.main")]),
    );

    let comparison = runtime
        .compare_admitted_replacement(&candidate)
        .expect("equivalent candidate compares");
    let classification = runtime
        .classify_replacement_impact(&comparison, &candidate)
        .expect("equivalent candidate classifies");

    assert_eq!(
        comparison.outcome(),
        WorthUiRuntimeArtifactComparisonOutcome::EquivalentNoOp
    );
    assert_eq!(classification.impact(), &WorthUiReplacementImpact::NoOp);
    assert!(classification.impact().is_noop());
    assert_eq!(classification.counters().artifact_comparisons_consumed(), 1);
    assert_eq!(classification.counters().dependency_metadata_reads(), 0);
    assert_eq!(classification.counters().impact_metadata_lookups(), 0);
    assert_eq!(classification.counters().plan_lowering_attempts(), 0);
}

#[test]
fn lane_affecting_change_classified_before_plan_lowering() {
    let app = impact_test_app();
    let runtime = launch_runtime(
        &app,
        artifact_from_modules(&app, [surface_module("workspace.surface.main")]),
    );
    let candidate = admitted_candidate(
        &app,
        &runtime,
        artifact_from_modules(&app, [surface_module("workspace.surface.overlay")]),
    );
    let comparison = runtime
        .compare_admitted_replacement(&candidate)
        .expect("surface candidate compares");

    let classification = runtime
        .classify_replacement_impact(&comparison, &candidate)
        .expect("surface semantic change classifies");

    match classification.impact() {
        WorthUiReplacementImpact::LaneAffecting { lane_impact, scope } => {
            assert!(lane_impact.requires_lane_parity());
            assert_eq!(lane_impact.reason(), Some("surface-semantics-changed"));
            assert!(scope.is_local_subtree());
            assert_eq!(scope.impacted_handle_count(), 1);
        }
        impact => panic!("expected lane-affecting impact, got {impact:?}"),
    }
    assert_eq!(classification.counters().dependency_metadata_reads(), 1);
    assert_eq!(classification.counters().plan_lowering_attempts(), 0);
}

#[test]
fn mismatched_comparison_candidate_rejected_before_impact_classification() {
    let app = impact_test_app();
    let runtime = launch_runtime(
        &app,
        artifact_from_modules(&app, [surface_module("workspace.surface.main")]),
    );
    let command_candidate = admitted_candidate(
        &app,
        &runtime,
        artifact_from_modules(&app, [surface_module("workspace.surface.command_save")]),
    );
    let token_candidate = admitted_candidate(
        &app,
        &runtime,
        artifact_from_modules(&app, [token_module("theme.text.secondary")]),
    );
    let command_comparison = runtime
        .compare_admitted_replacement(&command_candidate)
        .expect("command candidate compares");

    let denial = runtime
        .classify_replacement_impact(&command_comparison, &token_candidate)
        .expect_err("comparison evidence cannot certify a different candidate");

    match denial {
        WorthUiReplacementImpactDenial::ComparisonCandidateMismatch {
            comparison_candidate_artifact_digest,
            admitted_candidate_artifact_digest,
            counters,
        } => {
            assert_eq!(
                comparison_candidate_artifact_digest,
                command_comparison.candidate_artifact_digest()
            );
            assert_ne!(
                comparison_candidate_artifact_digest,
                admitted_candidate_artifact_digest
            );
            assert_eq!(counters.artifact_comparisons_consumed(), 1);
            assert_eq!(counters.dependency_metadata_reads(), 0);
            assert_eq!(counters.plan_lowering_attempts(), 0);
        }
        denial => panic!("expected comparison/candidate mismatch, got {denial:?}"),
    }
}

#[test]
fn comparison_from_different_active_basis_rejected_before_impact_classification() {
    let app = impact_test_app();
    let comparison_runtime = launch_runtime(
        &app,
        artifact_from_modules(&app, [surface_module("workspace.surface.main")]),
    );
    let admission_runtime = launch_runtime(
        &app,
        artifact_from_modules(&app, [token_module("theme.text.primary")]),
    );
    let candidate_artifact =
        artifact_from_modules(&app, [surface_module("workspace.surface.command_open")]);
    let comparison_candidate = admitted_candidate(&app, &comparison_runtime, candidate_artifact);
    let admitted_same_artifact = admitted_candidate(
        &app,
        &admission_runtime,
        artifact_from_modules(&app, [surface_module("workspace.surface.command_open")]),
    );
    let comparison = comparison_runtime
        .compare_admitted_replacement(&comparison_candidate)
        .expect("candidate compares against first active runtime");

    let denial = admission_runtime
        .classify_replacement_impact(&comparison, &admitted_same_artifact)
        .expect_err("comparison evidence cannot certify a different active basis");

    match denial {
        WorthUiReplacementImpactDenial::ComparisonActiveBasisMismatch {
            comparison_active_artifact_digest,
            admitted_active_artifact_digest,
            counters,
        } => {
            assert_eq!(
                comparison_active_artifact_digest,
                comparison.active_artifact_digest()
            );
            assert_ne!(
                comparison_active_artifact_digest,
                admitted_active_artifact_digest
            );
            assert_eq!(counters.artifact_comparisons_consumed(), 1);
            assert_eq!(counters.dependency_metadata_reads(), 0);
            assert_eq!(counters.plan_lowering_attempts(), 0);
        }
        denial => panic!("expected active-basis mismatch, got {denial:?}"),
    }
}

#[test]
fn import_insertion_classifies_as_bounded_structure_without_mutating_active_state() {
    let app = WorthUi::app()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .freeze()
        .map(crate::facade::entry::WorthUiCertificationApplicationTransition::activate_builder_host)
        .expect("application preparation should succeed");
    let runtime = launch_runtime(&app, import_artifact(["app/panels/inspector.wui"]));
    let active_before = runtime.inspect_active();
    let last_valid_before = runtime.last_valid();
    let candidate = admitted_candidate(
        &app,
        &runtime,
        import_artifact(["app/panels/inspector.wui", "app/panels/settings.wui"]),
    );
    let comparison = runtime
        .compare_admitted_replacement(&candidate)
        .expect("candidate compares before broad impact denial");

    let classification = runtime
        .classify_replacement_impact(&comparison, &candidate)
        .expect("one structural import insertion has exact bounded scope");

    assert!(matches!(
        classification.impact(),
        WorthUiReplacementImpact::StructuralReplacement(scope)
            if scope.impacted_handle_count() == 1
    ));
    assert_eq!(runtime.inspect_active(), active_before);
    assert_eq!(runtime.last_valid(), last_valid_before);
    assert_eq!(classification.counters().plan_lowering_attempts(), 0);
}

#[test]
fn same_module_node_retirement_remains_bounded_structural_replacement() {
    let app = impact_test_app();
    let runtime = launch_runtime(
        &app,
        artifact_from_modules(&app, [component_module("workspace.component.dashboard")]),
    );
    let candidate = admitted_candidate(
        &app,
        &runtime,
        artifact_from_modules(&app, [component_module("workspace.component.replacement")]),
    );
    let comparison = runtime
        .compare_admitted_replacement(&candidate)
        .expect("same-module component replacement compares");

    let classification = runtime
        .classify_replacement_impact(&comparison, &candidate)
        .expect("node retirement and creation remain bounded by the existing module");

    assert!(matches!(
        classification.impact(),
        WorthUiReplacementImpact::StructuralReplacement(scope)
            if scope.impacted_handle_count() == 1 && !scope.is_broad()
    ));
    assert_eq!(classification.counters().broad_replacement_denials(), 0);
}

#[test]
fn broad_replacement_without_state_drop_receipts_rejected() {
    let app = WorthUi::app()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .freeze()
        .map(crate::facade::entry::WorthUiCertificationApplicationTransition::activate_builder_host)
        .expect("application preparation should succeed");
    let runtime = launch_runtime(&app, import_artifact(["app/panels/inspector.wui"]));
    let candidate = admitted_candidate(&app, &runtime, two_module_import_artifact());
    let comparison = runtime
        .compare_admitted_replacement(&candidate)
        .expect("module-count candidate compares");

    let denial = runtime
        .classify_replacement_impact(&comparison, &candidate)
        .expect_err("broad replacement needs durable-state receipts");

    match denial {
        WorthUiReplacementImpactDenial::UnsupportedImpact {
            unsupported_impact:
                WorthUiUnsupportedReplacementImpact::MissingDurableStateReceipts { scope },
            counters,
        } => {
            assert!(scope.is_broad());
            assert!(!scope.durable_state_receipts_complete());
            assert_eq!(scope.full_artifact_handle_count(), 2);
            assert_eq!(counters.broad_replacement_denials(), 1);
            assert_eq!(counters.plan_lowering_attempts(), 0);
        }
        denial => panic!("expected missing durable-state receipt denial, got {denial:?}"),
    }
}

#[test]
fn theme_only_change_does_not_force_full_tree_replacement() {
    let app = impact_test_app();
    let runtime = launch_runtime(
        &app,
        artifact_from_modules(&app, [token_module("theme.text.primary")]),
    );
    let candidate = admitted_candidate(
        &app,
        &runtime,
        artifact_from_modules(&app, [token_module("theme.text.secondary")]),
    );
    let comparison = runtime
        .compare_admitted_replacement(&candidate)
        .expect("token candidate compares");

    let classification = runtime
        .classify_replacement_impact(&comparison, &candidate)
        .expect("token semantic change classifies");

    match classification.impact() {
        WorthUiReplacementImpact::LocalSubtree(scope) => {
            assert!(scope.is_local_subtree());
            assert_eq!(scope.impacted_handle_count(), 1);
            assert!(scope.full_artifact_handle_count() >= scope.impacted_handle_count());
        }
        impact => panic!("expected local subtree impact, got {impact:?}"),
    }
    assert_eq!(
        classification.token_theme_impact(),
        WorthUiTokenThemeImpact::ThemeOnly
    );
    assert_eq!(
        classification.command_impact(),
        WorthUiCommandImpact::Unchanged
    );
    assert_eq!(classification.counters().plan_lowering_attempts(), 0);
}

#[test]
fn command_only_change_does_not_force_full_tree_replacement() {
    let app = impact_test_app();
    let runtime = launch_runtime(
        &app,
        artifact_from_modules(&app, [surface_module("workspace.surface.command_save")]),
    );
    let candidate = admitted_candidate(
        &app,
        &runtime,
        artifact_from_modules(&app, [surface_module("workspace.surface.command_open")]),
    );
    let comparison = runtime
        .compare_admitted_replacement(&candidate)
        .expect("command-slot candidate compares");

    let classification = runtime
        .classify_replacement_impact(&comparison, &candidate)
        .expect("command-slot semantic change classifies");

    match classification.impact() {
        WorthUiReplacementImpact::LocalSubtree(scope) => {
            assert!(scope.is_local_subtree());
            assert_eq!(scope.impacted_handle_count(), 1);
        }
        impact => panic!("expected local command impact, got {impact:?}"),
    }
    assert_eq!(
        classification.command_impact(),
        WorthUiCommandImpact::BindingOnly
    );
    assert_eq!(
        classification.token_theme_impact(),
        WorthUiTokenThemeImpact::Unchanged
    );
    assert_eq!(classification.counters().plan_lowering_attempts(), 0);
}

#[test]
fn surface_command_slot_change_is_preserved_when_lane_change_dominates_scope() {
    let app = impact_test_app();
    let runtime = launch_runtime(
        &app,
        artifact_from_modules(&app, [surface_module("workspace.surface.command_save")]),
    );
    let candidate = admitted_candidate(
        &app,
        &runtime,
        artifact_from_modules(
            &app,
            [surface_module("workspace.surface.overlay_command_open")],
        ),
    );
    let comparison = runtime
        .compare_admitted_replacement(&candidate)
        .expect("surface command and lane candidate compares");

    let classification = runtime
        .classify_replacement_impact(&comparison, &candidate)
        .expect("combined surface semantic change classifies");

    match classification.impact() {
        WorthUiReplacementImpact::LaneAffecting { lane_impact, scope } => {
            assert!(lane_impact.requires_lane_parity());
            assert_eq!(lane_impact.reason(), Some("surface-semantics-changed"));
            assert!(scope.is_local_subtree());
            assert_eq!(scope.impacted_handle_count(), 1);
        }
        impact => panic!("expected lane-affecting impact, got {impact:?}"),
    }
    assert_eq!(
        classification.command_impact(),
        WorthUiCommandImpact::BindingOnly
    );
    assert_eq!(
        classification.token_theme_impact(),
        WorthUiTokenThemeImpact::Unchanged
    );
    assert_eq!(classification.counters().dependency_metadata_reads(), 1);
    assert_eq!(classification.counters().plan_lowering_attempts(), 0);
}
