use std::collections::BTreeSet;

use crate::runtime::tests::{
    dependency_impact_narrowing_test_support::{
        candidate_with_forged_query_support_hook_count, lower_rust_authored_artifact,
        query_bound_app, query_bound_artifact, query_bound_surface_app,
        surface_and_query_binding_module,
    },
    replacement_impact_test_support::{
        admitted_candidate, artifact_from_modules, impact_test_app, launch_runtime, surface_module,
    },
};
use crate::runtime::{WorthUiQueryDependencySurface, WorthUiRuntimeImpactNarrowingDenial};
use crate::source::WorthUiRustAuthoredArtifactInputModule;

#[test]
fn equivalent_dependency_metadata_produces_equivalent_runtime_impact() {
    let app = impact_test_app();
    let runtime = launch_runtime(
        &app,
        artifact_from_modules(&app, [token_and_surface_module("theme.text.primary")]),
    );
    let left = admitted_candidate(
        &app,
        &runtime,
        artifact_from_modules(&app, [token_and_surface_module("theme.text.secondary")]),
    );
    let right = admitted_candidate(
        &app,
        &runtime,
        artifact_from_modules(&app, [token_and_surface_module("theme.text.secondary")]),
    );

    let left_narrowing = classify_and_narrow(&runtime, &left);
    let right_narrowing = classify_and_narrow(&runtime, &right);

    assert_eq!(left_narrowing, right_narrowing);
    assert_eq!(left_narrowing.counters().full_artifact_scans(), 0);
    assert_eq!(left_narrowing.counters().plan_lowering_attempts(), 0);
}

#[test]
fn changed_module_impact_lookup_does_not_scan_full_artifact() {
    let app = impact_test_app();
    let runtime = launch_runtime(
        &app,
        artifact_from_modules(&app, [token_and_surface_module("theme.text.primary")]),
    );
    let candidate = admitted_candidate(
        &app,
        &runtime,
        artifact_from_modules(&app, [token_and_surface_module("theme.text.secondary")]),
    );

    let narrowing = classify_and_narrow(&runtime, &candidate);

    assert_eq!(narrowing.affected_handle_count(), 1);
    assert_eq!(narrowing.full_artifact_handle_count(), 2);
    assert_eq!(
        narrowing.affected_source_modules(),
        &["app/main.wui".to_owned()]
    );
    assert_eq!(narrowing.affected_subtree_digests().len(), 1);
    assert_eq!(narrowing.counters().dependency_metadata_reads(), 1);
    assert_eq!(narrowing.counters().module_impact_lookups(), 1);
    assert_eq!(narrowing.counters().subtree_impact_lookups(), 1);
    assert_eq!(narrowing.counters().runtime_hook_lookups(), 1);
    assert_eq!(narrowing.counters().full_artifact_scans(), 0);
}

#[test]
fn runtime_owned_query_dependency_links_preserved_during_impact_narrowing() {
    let app = query_bound_app();
    let runtime = launch_runtime(
        &app,
        query_bound_artifact(&app, "workspace.view_binding.selection"),
    );
    let candidate = admitted_candidate(
        &app,
        &runtime,
        query_bound_artifact(&app, "workspace.view_binding.detail"),
    );

    let narrowing = classify_and_narrow(&runtime, &candidate);
    let surfaces = narrowing
        .query_dependency_invalidations()
        .iter()
        .map(|invalidation| invalidation.surface())
        .collect::<BTreeSet<_>>();

    assert_eq!(narrowing.query_dependency_invalidations().len(), 4);
    assert_eq!(
        surfaces,
        BTreeSet::from([
            WorthUiQueryDependencySurface::LiveView,
            WorthUiQueryDependencySurface::RegionScopedLiveInvalidation,
            WorthUiQueryDependencySurface::SignalCompatibilityAndContinuation,
            WorthUiQueryDependencySurface::AsyncResourcesAndResultState,
        ])
    );
    assert!(narrowing
        .query_dependency_invalidations()
        .iter()
        .all(|invalidation| invalidation.view_binding_id() == "workspace.view_binding.detail"));
    assert_eq!(narrowing.counters().full_artifact_scans(), 0);
    assert_eq!(narrowing.counters().plan_lowering_attempts(), 0);
}

#[test]
fn query_bound_change_cannot_be_narrowed_by_ui_subtree_only() {
    let app = query_bound_surface_app();
    let runtime = launch_runtime(
        &app,
        lower_rust_authored_artifact(
            &app,
            [surface_and_query_binding_module(
                "workspace.surface.command_save",
                "workspace.view_binding.selection",
            )],
        ),
    );
    let candidate = candidate_with_forged_query_support_hook_count(
        &runtime,
        lower_rust_authored_artifact(
            &app,
            [surface_and_query_binding_module(
                "workspace.surface.command_open",
                "workspace.view_binding.selection",
            )],
        ),
        4,
    );
    let comparison = runtime
        .compare_admitted_replacement(&candidate)
        .expect("candidate compares before hostile narrowing");
    let classification = runtime
        .classify_replacement_impact(&comparison, &candidate)
        .expect("candidate classifies before hostile narrowing");

    let denial = runtime
        .narrow_replacement_impact(&classification, &candidate)
        .expect_err("forged Query support without dependency hooks denies");

    match denial {
        WorthUiRuntimeImpactNarrowingDenial::QueryDependencyPostureMissing {
            expected_runtime_hook_count,
            observed_runtime_hook_count,
            counters,
        } => {
            assert_eq!(expected_runtime_hook_count, 4);
            assert_eq!(observed_runtime_hook_count, 0);
            assert_eq!(counters.full_artifact_scans(), 0);
            assert_eq!(counters.plan_lowering_attempts(), 0);
        }
        denial => panic!("expected Query dependency posture denial, got {denial:?}"),
    }
}

#[test]
fn renderer_resource_change_cannot_broaden_unrelated_widget_subtrees() {
    let app = impact_test_app();
    let runtime = launch_runtime(
        &app,
        artifact_from_modules(
            &app,
            [surface_with_unrelated_widget_module(
                "workspace.surface.command_save",
            )],
        ),
    );
    let candidate = admitted_candidate(
        &app,
        &runtime,
        artifact_from_modules(
            &app,
            [surface_with_unrelated_widget_module(
                "workspace.surface.command_open",
            )],
        ),
    );

    let narrowing = classify_and_narrow(&runtime, &candidate);

    assert_eq!(narrowing.affected_handle_count(), 1);
    assert_eq!(narrowing.full_artifact_handle_count(), 2);
    assert_eq!(narrowing.renderer_resource_invalidations().len(), 1);
    assert_eq!(
        narrowing.renderer_resource_invalidations()[0].affected_resource_count(),
        1
    );
    assert!(!narrowing.renderer_resource_invalidations()[0].ordinary_widget_subtrees_broadened());
    assert_eq!(narrowing.counters().full_artifact_scans(), 0);
}

#[test]
fn lane_affecting_change_preserves_dependency_narrowing_scope() {
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

    let narrowing = classify_and_narrow(&runtime, &candidate);

    let lane_impact = narrowing
        .lane_impact()
        .expect("placement-class change carries lane impact");
    assert!(lane_impact.requires_lane_parity());
    assert_eq!(lane_impact.reason(), Some("surface-semantics-changed"));
    assert_eq!(narrowing.affected_handle_count(), 1);
    assert_eq!(
        narrowing.affected_source_modules(),
        &["app/main.wui".to_owned()]
    );
    assert_eq!(narrowing.affected_subtree_digests().len(), 1);
    assert_eq!(narrowing.counters().subtree_impact_lookups(), 1);
    assert_eq!(narrowing.counters().full_artifact_scans(), 0);
    assert_eq!(narrowing.counters().plan_lowering_attempts(), 0);
}

fn classify_and_narrow(
    runtime: &crate::runtime::WorthUiRuntime,
    candidate: &crate::runtime::WorthUiAdmittedReplacementCandidate,
) -> crate::runtime::WorthUiRuntimeImpactNarrowing {
    let comparison = runtime
        .compare_admitted_replacement(candidate)
        .expect("candidate compares before narrowing");
    let classification = runtime
        .classify_replacement_impact(&comparison, candidate)
        .expect("candidate impact classifies before narrowing");
    runtime
        .narrow_replacement_impact(&classification, candidate)
        .expect("candidate impact narrows")
}

fn token_and_surface_module(token_id: &str) -> WorthUiRustAuthoredArtifactInputModule {
    WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
        .with_token(token_id, token_id)
        .with_surface("workspace.surface.main")
}

fn surface_with_unrelated_widget_module(
    changed_surface_id: &str,
) -> WorthUiRustAuthoredArtifactInputModule {
    WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
        .with_surface(changed_surface_id)
        .with_surface("workspace.surface.main")
}
