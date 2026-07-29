use crate::capability::{QueryDenialPresentation, WorthUiQueryViewRegistration};
use crate::facade::{WorthUi, WorthUiApp};
use crate::runtime::tests::dependency_impact_narrowing_test_support::lower_rust_authored_artifact;
use crate::runtime::tests::replacement_impact_test_support::{admitted_candidate, launch_runtime};
use crate::runtime::{
    WorthUiAdmittedReplacementCandidate, WorthUiNodeReplacementPlan, WorthUiRuntimeImpactNarrowing,
};
use crate::source::WorthUiArtifact;
use std::sync::OnceLock;
use worth_ui_dsl::WorthUiRustAuthoredArtifactInputModule;

static LIVE_QUERY_DOMAIN: OnceLock<worth_ui_query_binding::WorthUiInstalledQueryDomain> =
    OnceLock::new();
static SNAPSHOT_QUERY_DOMAIN: OnceLock<worth_ui_query_binding::WorthUiInstalledQueryDomain> =
    OnceLock::new();

pub(super) fn standard_query_app() -> WorthUiApp {
    query_app(true, QueryDenialPresentation::structured_status())
}

pub(super) fn fresh_standard_query_app() -> WorthUiApp {
    fresh_query_app(true, QueryDenialPresentation::structured_status())
}

pub(super) fn lifecycle_drift_query_app() -> WorthUiApp {
    query_app(false, QueryDenialPresentation::structured_status())
}

pub(super) fn mixed_change_query_apps() -> (WorthUiApp, WorthUiApp) {
    let active_installed = LIVE_QUERY_DOMAIN
        .get_or_init(|| {
            worth_ui_query_binding::certification::worth_ui_installed_test_domain("live-query-app")
        })
        .clone();
    let candidate_installed = SNAPSHOT_QUERY_DOMAIN
        .get_or_init(|| {
            worth_ui_query_binding::certification::worth_ui_installed_test_domain(
                "snapshot-query-app",
            )
        })
        .clone();
    let active = app_with_mixed_change_views(&active_installed, true);
    let candidate = app_with_mixed_change_views(&candidate_installed, false);
    (active, candidate)
}

pub(super) fn denial_presentation_drift_query_app() -> WorthUiApp {
    query_app(true, QueryDenialPresentation::advisory_text())
}

pub(super) fn wide_query_app(binding_count: usize) -> WorthUiApp {
    let installed = worth_ui_query_binding::certification::worth_ui_installed_test_domain(
        "wide-query-comparison-app",
    );
    let mut builder = WorthUi::app()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse());
    for binding_index in 0..binding_count {
        let binding_id = format!("workspace.view_binding.item_{binding_index:03}");
        builder = builder
            .register_query_view(query_registration(
                &installed,
                &binding_id,
                true,
                QueryDenialPresentation::structured_status(),
            ))
            .expect("installed wide Query view should register");
    }
    builder
        .register_query_view(query_registration(
            &installed,
            "workspace.view_binding.replacement",
            true,
            QueryDenialPresentation::structured_status(),
        ))
        .expect("replacement Query view should register")
        .freeze()
        .expect("wide application preparation should succeed")
}

pub(super) fn query_artifact(app: &WorthUiApp, binding_id: &str) -> WorthUiArtifact {
    lower_rust_authored_artifact(
        app,
        [WorthUiRustAuthoredArtifactInputModule::new("app/main.wui").with_binding(binding_id)],
    )
}

pub(super) fn query_artifact_with_bindings(
    app: &WorthUiApp,
    binding_ids: &[&str],
) -> WorthUiArtifact {
    let module = binding_ids.iter().fold(
        WorthUiRustAuthoredArtifactInputModule::new("app/main.wui"),
        |module, binding_id| module.with_binding(*binding_id),
    );
    lower_rust_authored_artifact(app, [module])
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

fn query_app(live: bool, denial: QueryDenialPresentation) -> WorthUiApp {
    let (installed, label) = if live {
        (&LIVE_QUERY_DOMAIN, "live-query-app")
    } else {
        (&SNAPSHOT_QUERY_DOMAIN, "snapshot-query-app")
    };
    let installed = installed
        .get_or_init(|| {
            worth_ui_query_binding::certification::worth_ui_installed_test_domain(label)
        })
        .clone();
    app_from_installed_query_domain(&installed, live, denial)
}

fn fresh_query_app(live: bool, denial: QueryDenialPresentation) -> WorthUiApp {
    let installed =
        worth_ui_query_binding::certification::worth_ui_installed_test_domain(if live {
            "fresh-live-query-app"
        } else {
            "fresh-snapshot-query-app"
        });
    app_from_installed_query_domain(&installed, live, denial)
}

fn app_from_installed_query_domain(
    installed: &worth_ui_query_binding::WorthUiInstalledQueryDomain,
    live: bool,
    denial: QueryDenialPresentation,
) -> WorthUiApp {
    let selection = query_registration(installed, "workspace.view_binding.selection", live, denial);
    let detail = query_registration(installed, "workspace.view_binding.detail", live, denial);
    WorthUi::app()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .register_query_view(selection)
        .expect("installed selection view should register")
        .register_query_view(detail)
        .expect("installed detail view should register")
        .freeze()
        .expect("application preparation should succeed")
}

fn app_with_mixed_change_views(
    installed: &worth_ui_query_binding::WorthUiInstalledQueryDomain,
    selection_is_live: bool,
) -> WorthUiApp {
    let denial = QueryDenialPresentation::structured_status();
    WorthUi::app()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .register_query_view(query_registration(
            installed,
            "workspace.view_binding.selection",
            selection_is_live,
            denial,
        ))
        .expect("installed selection view should register")
        .register_query_view(query_registration(
            installed,
            "workspace.view_binding.detail",
            true,
            denial,
        ))
        .expect("installed detail view should register")
        .register_query_view(query_registration(
            installed,
            "workspace.view_binding.replacement",
            true,
            denial,
        ))
        .expect("installed replacement view should register")
        .freeze()
        .expect("mixed-change application preparation should succeed")
}

fn query_registration(
    installed: &worth_ui_query_binding::WorthUiInstalledQueryDomain,
    id: &str,
    live: bool,
    denial: QueryDenialPresentation,
) -> WorthUiQueryViewRegistration {
    let view: worth_ui_query_binding::WorthUiInstalledQueryView = if live {
        installed.live_measurement_view(id).map(Into::into)
    } else {
        installed.measurement_view(id).map(Into::into)
    }
    .expect("installed query view should admit");
    WorthUiQueryViewRegistration::new(view).with_denial_presentation(denial)
}
