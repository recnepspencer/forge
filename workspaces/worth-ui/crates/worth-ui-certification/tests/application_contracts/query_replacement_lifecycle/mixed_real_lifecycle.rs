use std::time::Duration;
use worth_ui_test_support::{
    WorthUiActiveSessionCertificationExt, WorthUiFrameworkTurnCertificationExt,
};

use worth_query::facade::runtime::WorthQueryWorkspace;
use worth_ui::facade::app::{
    WorthUiActiveApplicationSession, WorthUiApplicationReplacementOutcome, WorthUiVisibleRange,
};
use worth_ui::facade::source::{
    WorthUiFilesystemSourceProvider, WorthUiFilesystemSourceWatcher,
    WorthUiWatchedCandidateSubmission,
};
use worth_ui_certification::scenario::application_authority_closure::candidate_catalog::admit_candidate_catalog;
use worth_ui_host_egui::WorthUiHostEgui;
use worth_ui_query_binding::{
    WorthUiInstalledLiveQueryView, WorthUiQueryViewShape, WorthUiQueryWorkspaceExt,
};
use worth_ui_runtime::facade::application::{
    WorthUiOrdinaryFrameTarget, WorthUiVirtualizedDataFrameTarget,
    WorthUiVirtualizedPlanAvailability, WorthUiVirtualizedPlanSummaryRequest,
};

#[path = "mixed_real_lifecycle/hostile_mounted_journey.rs"]
mod hostile_mounted_journey;
#[path = "mixed_real_lifecycle/mounted_successor.rs"]
mod mounted_successor;
mod query_patch;

use super::scenario::{
    application_with_submission_and_host, capability_application,
    installed_workspace_with_measurement_authority, ACTIVE_COMPONENT, FIRST_VIEW, NEXT_COMPONENT,
    SECOND_VIEW,
};
use super::support::{
    activation_boundary, admit_active_resource, admit_candidate_resource, close, close_retirement,
    open_resource,
};
use crate::filesystem_contract_workspace::FilesystemContractWorkspace;

const PANEL: &str = "app/panel.wui";
const SETTLEMENT_TIMEOUT: Duration = Duration::from_secs(5);
const CHURN_COUNT: usize = 64;

#[test]
fn one_real_session_composes_watcher_query_egui_denials_and_churn() {
    let watched = FilesystemContractWorkspace::new("mixed-real-lifecycle");
    watched.write("app/main.wui", "import \"app/panel.wui\";");
    watched.write(PANEL, &query_source(ACTIVE_COMPONENT, FIRST_VIEW, false));
    let mut watcher =
        WorthUiFilesystemSourceWatcher::start(WorthUiFilesystemSourceProvider::new(watched.root()))
            .expect("the production watcher registers the real source tree");

    let (mut query_workspace, measurement) =
        installed_workspace_with_measurement_authority("mixed-real-lifecycle");
    let installed = query_workspace
        .worth_ui()
        .expect("Worth UI domain installed");
    let first = installed
        .live_measurement_view(FIRST_VIEW)
        .expect("first installed view");
    let second = installed
        .live_measurement_view(SECOND_VIEW)
        .expect("second installed view");
    let capabilities = capability_application(first.clone(), second.clone(), &mut query_workspace);
    let first_reference = capabilities
        .resolve_query_view(
            first.definition().identity(),
            WorthUiQueryViewShape::Collection,
        )
        .expect("the live capability application resolves its exact installed reference");
    let initial = watcher
        .take_initial_snapshot()
        .expect("the watcher owns the settled initial source")
        .attempt_candidate_for_certification(capabilities.capabilities())
        .expect("initial real files lower");
    let context = egui::Context::default();
    let mut session = application_with_submission_and_host(
        first.clone(),
        second.clone(),
        initial,
        WorthUiHostEgui::new(context.clone()),
        &mut query_workspace,
    )
    .launch()
    .expect("the watcher-backed Query and egui application launches");
    admit_active_resource(&mut session, &first, &mut query_workspace);
    query_patch::apply_real_live_patch(
        &mut session,
        &first_reference,
        &measurement,
        &mut query_workspace,
    );
    execute_real_egui_frame(&context, &mut session, true);

    let launch_generation = session.generation_identity().clone();
    watched.write_atomic(PANEL, &query_source(ACTIVE_COMPONENT, FIRST_VIEW, true));
    let primed = activate_settled(&mut watcher, &mut session, None, &mut query_workspace)
        .into_activation()
        .expect("the first replacement primes complete allocation truth");
    assert!(primed.operation_live_retirement().is_empty());
    assert_ne!(session.generation_identity(), &launch_generation);
    execute_real_egui_frame(&context, &mut session, true);

    let primed_generation = session.generation_identity().clone();
    watched.write_atomic(PANEL, &query_source(ACTIVE_COMPONENT, FIRST_VIEW, false));
    let no_op = activate_settled(&mut watcher, &mut session, None, &mut query_workspace);
    assert!(no_op.semantic_no_op().is_some());
    assert_eq!(session.generation_identity(), &primed_generation);
    execute_real_egui_frame(&context, &mut session, true);

    let first_target = visible_target(&session);
    watched.write_atomic(PANEL, &query_source(NEXT_COMPONENT, SECOND_VIEW, false));
    let changed = activate_settled(
        &mut watcher,
        &mut session,
        Some(&second),
        &mut query_workspace,
    )
    .into_activation()
    .expect("bounded Query change publishes");
    assert_ne!(session.generation_identity(), &primed_generation);
    close_retirement(
        changed.into_operation_live_retirement(),
        &mut query_workspace,
    );
    assert_stale_query_target(&mut session, first_target);
    execute_real_egui_frame(&context, &mut session, true);

    assert_denied_edit_preserves_output(
        &watched,
        &mut watcher,
        &context,
        &mut session,
        PANEL,
        "not a valid Worth UI declaration",
    );
    assert_denied_edit_preserves_output(
        &watched,
        &mut watcher,
        &context,
        &mut session,
        PANEL,
        &format!("component {NEXT_COMPONENT} {{"),
    );

    let before_removed_import = session.generation_identity().clone();
    watched.remove(PANEL);
    let missing = watcher
        .settle(SETTLEMENT_TIMEOUT)
        .expect("removed imported file settles as final tree truth");
    assert!(missing
        .attempt_candidate_for_certification(session.capabilities())
        .is_err());
    assert_eq!(session.generation_identity(), &before_removed_import);
    execute_real_egui_frame(&context, &mut session, true);

    watched.write_atomic(PANEL, &query_source(NEXT_COMPONENT, SECOND_VIEW, false));
    let restored = activate_settled(&mut watcher, &mut session, None, &mut query_workspace);
    if restored.semantic_no_op().is_some() {
        assert_eq!(session.generation_identity(), &before_removed_import);
    } else {
        let receipt = restored
            .into_activation()
            .expect("restoration must return one canonical executable decision");
        assert_eq!(receipt.prior_generation(), &before_removed_import);
        assert_eq!(receipt.active_generation(), session.generation_identity());
        assert!(receipt.operation_live_retirement().is_empty());
    }
    execute_real_egui_frame(&context, &mut session, true);

    watched.write_atomic(PANEL, &query_free_source(ACTIVE_COMPONENT));
    let query_free = activate_settled(&mut watcher, &mut session, None, &mut query_workspace)
        .into_activation()
        .expect("Query removal publishes");
    close_retirement(
        query_free.into_operation_live_retirement(),
        &mut query_workspace,
    );
    assert_eq!(
        session.virtualized_plan_availability(),
        WorthUiVirtualizedPlanAvailability::QueryFree
    );
    execute_real_egui_frame(&context, &mut session, false);

    watched.write_atomic(PANEL, &query_source(ACTIVE_COMPONENT, FIRST_VIEW, false));
    let rebound = activate_settled(
        &mut watcher,
        &mut session,
        Some(&first),
        &mut query_workspace,
    )
    .into_activation()
    .expect("Query reintroduction publishes");
    assert!(rebound.operation_live_retirement().is_empty());
    execute_real_egui_frame(&context, &mut session, true);

    let frozen = freeze_churn_candidates(&session, CHURN_COUNT);
    for (submission, next_is_second) in frozen {
        let next_view = if next_is_second { &second } else { &first };
        let changed = activate_submission(
            &mut session,
            submission,
            Some(next_view),
            &mut query_workspace,
        )
        .into_activation()
        .expect("each production-frozen candidate publishes");
        assert_eq!(changed.operation_live_retirement().len(), 1);
        close_retirement(
            changed.into_operation_live_retirement(),
            &mut query_workspace,
        );
        execute_real_egui_frame(&context, &mut session, true);
    }

    close_retirement(
        session.shutdown().into_operation_live_retirement(),
        &mut query_workspace,
    );
    close(
        open_resource(&first, &mut query_workspace),
        &mut query_workspace,
    );
    let shutdown = watcher.shutdown().expect("the real watcher unregisters");
    assert!(shutdown.observed_notification_count() >= 7);
    watched.close();
}

fn activate_settled(
    watcher: &mut WorthUiFilesystemSourceWatcher,
    session: &mut WorthUiActiveApplicationSession,
    candidate_view: Option<&WorthUiInstalledLiveQueryView>,
    workspace: &mut WorthQueryWorkspace,
) -> WorthUiApplicationReplacementOutcome {
    let submission = watcher
        .settle(SETTLEMENT_TIMEOUT)
        .expect("the external edit settles through the production watcher")
        .attempt_candidate_for_certification(session.capabilities())
        .expect("the stable source lowers through production semantics");
    activate_submission(session, submission, candidate_view, workspace)
}

fn activate_submission(
    session: &mut WorthUiActiveApplicationSession,
    submission: WorthUiWatchedCandidateSubmission,
    candidate_view: Option<&WorthUiInstalledLiveQueryView>,
    workspace: &mut WorthQueryWorkspace,
) -> WorthUiApplicationReplacementOutcome {
    let mut prepared = session
        .prepare_replacement(submission)
        .expect("candidate prepares through the public session");
    if let Some(view) = candidate_view {
        admit_candidate_resource(&mut prepared, view, workspace);
    }
    let catalog = admit_candidate_catalog(session, &mut prepared);
    let lowered = session
        .lower_prepared_replacement(*prepared)
        .expect("candidate lowers");
    let pending = session
        .stage_prepared_replacement(lowered)
        .expect("candidate stages");
    let boundary = activation_boundary(session);
    session
        .activate_prepared_replacement(pending, catalog, boundary, None)
        .expect("valid candidate reaches its executable decision")
}

fn assert_denied_edit_preserves_output(
    workspace: &FilesystemContractWorkspace,
    watcher: &mut WorthUiFilesystemSourceWatcher,
    context: &egui::Context,
    session: &mut WorthUiActiveApplicationSession,
    path: &str,
    bytes: &str,
) {
    let generation = session.generation_identity().clone();
    workspace.write(path, bytes);
    let denied = watcher
        .settle(SETTLEMENT_TIMEOUT)
        .expect("stable denied bytes remain observable filesystem truth");
    assert!(denied
        .attempt_candidate_for_certification(session.capabilities())
        .is_err());
    assert_eq!(session.generation_identity(), &generation);
    execute_real_egui_frame(context, session, true);
}

fn execute_real_egui_frame(
    context: &egui::Context,
    session: &mut WorthUiActiveApplicationSession,
    query_expected: bool,
) {
    let query_target = query_expected.then(|| visible_target(session));
    let native = context.run(raw_input(), |_| {
        let execution = session
            .execute_framework_turn(|_| {})
            .expect("no mounted presentation lease is active")
            .into_execution()
            .unwrap_or_else(|_| panic!("egui framework turn"));
        let ordinary = execution
            .execute_ordinary_frame(WorthUiOrdinaryFrameTarget::root_shell())
            .expect("ordinary frame executes");
        drop(ordinary);
        if let Some(target) = query_target {
            execution
                .execute_virtualized_data_frame(target)
                .expect("visible Query frame executes");
        }
    });
    assert!(
        native.shapes.is_empty(),
        "lane execution is receipt-only and cannot contact egui"
    );
}

fn visible_target(session: &WorthUiActiveApplicationSession) -> WorthUiVirtualizedDataFrameTarget {
    session
        .inspect_virtualized_plan(WorthUiVirtualizedPlanSummaryRequest::first_view())
        .expect("active Query summary")
        .target(WorthUiVisibleRange::rows(0, 1).expect("one visible row"))
}

fn assert_stale_query_target(
    session: &mut WorthUiActiveApplicationSession,
    target: WorthUiVirtualizedDataFrameTarget,
) {
    let execution = session
        .execute_framework_turn(|_| {})
        .expect("no mounted presentation lease is active")
        .into_execution()
        .unwrap_or_else(|_| panic!("stale-target framework turn"));
    assert!(execution.execute_virtualized_data_frame(target).is_err());
}

fn freeze_churn_candidates(
    session: &WorthUiActiveApplicationSession,
    count: usize,
) -> Vec<(WorthUiWatchedCandidateSubmission, bool)> {
    let workspace = FilesystemContractWorkspace::new("mixed-frozen-churn");
    workspace.write("app/main.wui", "import \"app/panel.wui\";");
    let provider = WorthUiFilesystemSourceProvider::new(workspace.root());
    let candidates = (0..count)
        .map(|step| {
            let next_is_second = step % 2 == 0;
            let source = if next_is_second {
                query_source(NEXT_COMPONENT, SECOND_VIEW, false)
            } else {
                query_source(ACTIVE_COMPONENT, FIRST_VIEW, false)
            };
            workspace.write_atomic(PANEL, &source);
            let submission = provider
                .read()
                .expect("production reader freezes the deterministic candidate")
                .attempt_candidate_for_certification(session.capabilities())
                .expect("frozen churn source lowers");
            (submission, next_is_second)
        })
        .collect();
    workspace.close();
    candidates
}

fn query_source(component: &str, view: &str, reformatted: bool) -> String {
    if reformatted {
        format!(
            "\n component {component} {{\n region workspace.region.query_lifecycle {{\n sizing workspace.sizing.query_lifecycle;\n }}\n }}\n binding {view} {{}}\n"
        )
    } else {
        format!("{}\nbinding {view} {{}}", query_free_source(component))
    }
}

fn query_free_source(component: &str) -> String {
    format!(
        "component {component} {{ region workspace.region.query_lifecycle {{ sizing workspace.sizing.query_lifecycle; }} }}"
    )
}

fn raw_input() -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(1024.0, 720.0),
        )),
        ..Default::default()
    }
}
