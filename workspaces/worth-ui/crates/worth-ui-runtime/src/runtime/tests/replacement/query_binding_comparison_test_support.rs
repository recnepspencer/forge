use crate::capability::{
    QueryDenialPresentation, WorthUiQueryViewRegistration,
};
use crate::facade::{WorthUi, WorthUiApp};
use crate::runtime::tests::dependency_impact_narrowing_test_support::lower_rust_authored_artifact;
use crate::runtime::tests::replacement_impact_test_support::{admitted_candidate, launch_runtime};
use crate::runtime::{
    WorthUiAdmittedReplacementCandidate, WorthUiNodeReplacementPlan, WorthUiRuntimeImpactNarrowing,
};
use crate::source::{WorthUiArtifact, WorthUiRustAuthoredArtifactInputModule};

pub(super) fn standard_query_app() -> WorthUiApp {
    query_app(true, QueryDenialPresentation::structured_status())
}

pub(super) fn lifecycle_drift_query_app() -> WorthUiApp {
    query_app(false, QueryDenialPresentation::structured_status())
}

pub(super) fn denial_presentation_drift_query_app() -> WorthUiApp {
    query_app(true, QueryDenialPresentation::advisory_text())
}

pub(super) fn query_artifact(app: &WorthUiApp, binding_id: &str) -> WorthUiArtifact {
    lower_rust_authored_artifact(
        app,
        [WorthUiRustAuthoredArtifactInputModule::new("app/main.wui").with_binding(binding_id)],
    )
}

pub(super) fn phase11_pipeline(
    active_app: &WorthUiApp,
    active_artifact: WorthUiArtifact,
    candidate_artifact: WorthUiArtifact,
) -> (
    crate::runtime::WorthUiRuntimeFrameworkLoop,
    WorthUiAdmittedReplacementCandidate,
    WorthUiRuntimeImpactNarrowing,
    WorthUiNodeReplacementPlan,
) {
    let runtime = launch_runtime(active_app, active_artifact);
    let admitted = admitted_candidate(active_app, &runtime, candidate_artifact);
    let artifact_comparison = runtime
        .compare_admitted_replacement(&admitted)
        .expect("runtime comparison succeeds");
    let impact = runtime
        .classify_replacement_impact(&artifact_comparison, &admitted)
        .expect("impact classification succeeds");
    let narrowing = runtime
        .narrow_replacement_impact(&impact, &admitted)
        .expect("impact narrowing succeeds");
    let identity_report = runtime
        .build_identity_match_graph(&narrowing, &admitted)
        .expect("identity report succeeds");
    let plan = runtime
        .classify_node_replacements(&impact, &narrowing, &identity_report)
        .expect("node replacement plan succeeds");
    (runtime, admitted, narrowing, plan)
}

fn query_app(
    live: bool,
    denial: QueryDenialPresentation,
) -> WorthUiApp {
    let installed = worth_ui_query_binding::certification::worth_ui_installed_test_domain(
        if live { "live-query-app" } else { "snapshot-query-app" },
    );
    let selection = query_registration(
        &installed,
        "workspace.view_binding.selection",
        live,
        denial.clone(),
    );
    let detail = query_registration(
        &installed,
        "workspace.view_binding.detail",
        live,
        denial,
    );
    WorthUi::app()
        .register_query_view(selection)
        .expect("installed selection view should register")
        .register_query_view(detail)
        .expect("installed detail view should register")
        .freeze()
}

fn query_registration(
    installed: &worth_ui_query_binding::WorthUiInstalledQueryDomain,
    id: &str,
    live: bool,
    denial: QueryDenialPresentation,
) -> WorthUiQueryViewRegistration {
    let view = if live {
        installed.live_measurement_view(id)
    } else {
        installed.measurement_view(id)
    }
    .expect("installed query view should admit");
    WorthUiQueryViewRegistration::new(view).with_denial_presentation(denial)
}
