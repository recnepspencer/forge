use std::collections::{BTreeMap, BTreeSet};

use worth_query::facade::runtime::{
    QuerySubscriptionFamily,
    QuerySubscriptionSupportPosture,
};

use super::query_binding::WorthUiQueryBindingEvidenceIndex;
use super::query_binding_comparison_test_support::{
    phase11_pipeline, query_artifact, result_shape_drift_query_app, standard_query_app,
};
use crate::capability::QueryLiveCompatibility;
use crate::runtime::{
    WorthUiQueryBindingComparisonOutcome, WorthUiQueryBindingPostureDriftFamily,
    WorthUiQuerySupportReceipt, WorthUiQuerySupportStatus,
};
use crate::source::{
    WorthUiArtifact, WorthUiArtifactDependencyDeriver, WorthUiArtifactDependencyGraph,
    WorthUiRuntimeDependencyHookKind,
};

#[test]
fn query_support_and_live_drift_deny_preservation_before_subscription_reuse() {
    let app = standard_query_app();
    let artifact = query_artifact(&app, "workspace.view_binding.selection");
    let active = WorthUiQueryBindingEvidenceIndex::from_active_artifact(&artifact);
    let candidate = evidence_index_with_live_posture_and_support_receipt(
        &artifact,
        QuerySubscriptionSupportPosture::RuntimeBackedDeferred,
        WorthUiQuerySupportStatus::Deferred,
    );

    let active_posture = active
        .get("workspace.view_binding.selection")
        .expect("active evidence")
        .posture();
    let candidate_posture = candidate
        .get("workspace.view_binding.selection")
        .expect("candidate evidence")
        .posture();

    assert_eq!(
        active_posture.drift_families_against(candidate_posture),
        vec![
            WorthUiQueryBindingPostureDriftFamily::SupportAdmission,
            WorthUiQueryBindingPostureDriftFamily::LiveCompatibility,
        ]
    );
}

#[test]
fn result_shape_drift_rebinds_through_typed_query_identity_before_projection_reuse() {
    let active_app = standard_query_app();
    let candidate_app = result_shape_drift_query_app();
    let active = query_artifact(&active_app, "workspace.view_binding.selection");
    let candidate = query_artifact(&candidate_app, "workspace.view_binding.selection");
    let (runtime, admitted, narrowing, plan) = phase11_pipeline(&active_app, active, candidate);

    let comparison = runtime
        .compare_query_bindings(&plan, &narrowing, &admitted)
        .expect("query binding comparison succeeds");
    let binding = comparison
        .binding_for_view_binding_id("workspace.view_binding.selection")
        .expect("selection binding exists");

    assert_eq!(
        binding.outcome(),
        WorthUiQueryBindingComparisonOutcome::RebindRequired
    );
    assert!(binding.posture_drifts().is_empty());
    assert_ne!(
        binding
            .active_posture()
            .expect("active posture")
            .projection_consumption_digest(),
        binding
            .candidate_posture()
            .expect("candidate posture")
            .projection_consumption_digest()
    );
}

#[test]
fn async_and_recovery_runtime_surfaces_are_distinct_query_posture_families() {
    let app = standard_query_app();
    let artifact = query_artifact(&app, "workspace.view_binding.selection");
    let full_runtime_surface = evidence_index_with_runtime_hook_kinds(
        &artifact,
        &[
            WorthUiRuntimeDependencyHookKind::QuerySignalContinuation,
            WorthUiRuntimeDependencyHookKind::QueryAsyncResultState,
        ],
    );
    let missing_async_surface = evidence_index_with_runtime_hook_kinds(
        &artifact,
        &[WorthUiRuntimeDependencyHookKind::QuerySignalContinuation],
    );
    let missing_recovery_surface = evidence_index_with_runtime_hook_kinds(
        &artifact,
        &[WorthUiRuntimeDependencyHookKind::QueryAsyncResultState],
    );

    let full_posture = full_runtime_surface
        .get("workspace.view_binding.selection")
        .expect("full runtime surface evidence")
        .posture();
    let missing_async_posture = missing_async_surface
        .get("workspace.view_binding.selection")
        .expect("missing async evidence")
        .posture();
    let missing_recovery_posture = missing_recovery_surface
        .get("workspace.view_binding.selection")
        .expect("missing recovery evidence")
        .posture();

    assert_eq!(
        full_posture.drift_families_against(missing_async_posture),
        vec![WorthUiQueryBindingPostureDriftFamily::AsyncResultState]
    );
    assert_eq!(
        full_posture.drift_families_against(missing_recovery_posture),
        vec![WorthUiQueryBindingPostureDriftFamily::Recovery]
    );
}

fn evidence_index_with_runtime_hook_kinds(
    artifact: &WorthUiArtifact,
    retained_kinds: &[WorthUiRuntimeDependencyHookKind],
) -> WorthUiQueryBindingEvidenceIndex {
    let report = WorthUiArtifactDependencyDeriver::derive_with_report(artifact);
    let graph = report.basis().dependency_graph();
    let filtered_graph = WorthUiArtifactDependencyGraph::new(
        graph.edges().to_vec(),
        graph.module_dependencies().clone(),
        graph.subtree_digests().clone(),
        runtime_hooks_with_only(retained_kinds, graph),
    );

    WorthUiQueryBindingEvidenceIndex::from_artifact_graph_and_support_receipt(
        artifact,
        &filtered_graph,
        WorthUiQuerySupportReceipt::with_runtime_hook_count_for_test(
            WorthUiQuerySupportStatus::Supported,
            4,
            0x51a7_e11du64,
        ),
    )
}

fn evidence_index_with_live_posture_and_support_receipt(
    artifact: &WorthUiArtifact,
    live_posture: QuerySubscriptionSupportPosture,
    support_status: WorthUiQuerySupportStatus,
) -> WorthUiQueryBindingEvidenceIndex {
    let report = WorthUiArtifactDependencyDeriver::derive_with_report(artifact);
    let graph = report.basis().dependency_graph();
    let rewritten_graph = WorthUiArtifactDependencyGraph::new(
        graph.edges().to_vec(),
        graph.module_dependencies().clone(),
        graph.subtree_digests().clone(),
        runtime_hooks_with_live_posture(live_posture, graph),
    );

    WorthUiQueryBindingEvidenceIndex::from_artifact_graph_and_support_receipt(
        artifact,
        &rewritten_graph,
        WorthUiQuerySupportReceipt::with_runtime_hook_count_for_test(
            support_status,
            4,
            0x51a7_e11du64,
        ),
    )
}

fn runtime_hooks_with_only(
    retained_kinds: &[WorthUiRuntimeDependencyHookKind],
    graph: &WorthUiArtifactDependencyGraph,
) -> BTreeMap<crate::source::WorthUiArtifactHandle, Vec<crate::source::WorthUiRuntimeDependencyHook>>
{
    let retained_kinds = retained_kinds.iter().copied().collect::<BTreeSet<_>>();
    graph
        .runtime_hooks()
        .iter()
        .filter_map(|(handle, hooks)| {
            let retained_hooks = hooks
                .iter()
                .filter(|hook| retained_kinds.contains(&hook.kind()))
                .cloned()
                .collect::<Vec<_>>();
            (!retained_hooks.is_empty()).then(|| (handle.clone(), retained_hooks))
        })
        .collect()
}

fn runtime_hooks_with_live_posture(
    live_posture: QuerySubscriptionSupportPosture,
    graph: &WorthUiArtifactDependencyGraph,
) -> BTreeMap<crate::source::WorthUiArtifactHandle, Vec<crate::source::WorthUiRuntimeDependencyHook>>
{
    graph
        .runtime_hooks()
        .iter()
        .map(|(handle, hooks)| {
            let rewritten_hooks = hooks
                .iter()
                .map(|hook| {
                    crate::source::WorthUiRuntimeDependencyHook::new(
                        hook.kind(),
                        hook.view_binding_id().clone(),
                        hook.query_capability().clone(),
                        hook.query_composition_profile_digest(),
                        hook.view_shape().clone(),
                        hook.result_shape().clone(),
                        hook.basis_posture().clone(),
                        QueryLiveCompatibility::from_subscription_posture(
                            QuerySubscriptionFamily::CollectionMembership,
                            live_posture,
                        ),
                        hook.denial_presentation().clone(),
                    )
                })
                .collect::<Vec<_>>();
            (handle.clone(), rewritten_hooks)
        })
        .collect()
}
