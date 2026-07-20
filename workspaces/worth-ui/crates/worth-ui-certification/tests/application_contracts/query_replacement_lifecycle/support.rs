use worth_query::facade::{domain, runtime};
use worth_ui::facade::app::{
    WorthUiActiveApplicationSession, WorthUiPreparedApplicationReplacement,
};
use worth_ui::facade::query_binding::{
    WorthUiInstalledLiveQueryView, WorthUiQueryLiveCloseOutcome, WorthUiQueryLiveOpenOutcome,
    WorthUiQueryLiveResource, WorthUiQueryLiveRetirement, WorthUiQueryLiveRetirementCloseOutcome,
};
use worth_ui::facade::source::WorthUiWatchedCandidateSubmission;
use worth_ui::facade::{
    app::{WorthUiVirtualizedPlanSummaryRequest, WorthUiVisibleRange},
    runtime::WorthUiFrameBoundary,
};
use worth_ui_certification::scenario::application_authority_closure::candidate_catalog::admit_candidate_catalog;

pub(super) fn prepare_catalog(
    session: &WorthUiActiveApplicationSession,
    submission: WorthUiWatchedCandidateSubmission,
) -> (
    Box<WorthUiPreparedApplicationReplacement>,
    worth_ui::facade::graph::UiAdmittedAllocationCatalogDelta,
) {
    let mut prepared = session
        .prepare_replacement(submission)
        .expect("replacement prepares");
    let catalog = admit_candidate_catalog(&mut prepared);
    (prepared, catalog)
}

pub(super) fn lower_and_stage(
    session: &WorthUiActiveApplicationSession,
    prepared: (
        Box<WorthUiPreparedApplicationReplacement>,
        worth_ui::facade::graph::UiAdmittedAllocationCatalogDelta,
    ),
) -> (
    worth_ui::facade::app::WorthUiPendingApplicationCutover,
    worth_ui::facade::graph::UiAdmittedAllocationCatalogDelta,
) {
    let lowered = session
        .lower_prepared_replacement(*prepared.0)
        .expect("candidate lowers");
    let pending = session
        .stage_prepared_replacement(lowered)
        .expect("candidate stages");
    (pending, prepared.1)
}

pub(super) fn activate(
    session: &mut WorthUiActiveApplicationSession,
    pending: worth_ui::facade::app::WorthUiPendingApplicationCutover,
    catalog: worth_ui::facade::graph::UiAdmittedAllocationCatalogDelta,
) -> worth_ui::facade::app::WorthUiApplicationCutoverReceipt {
    let boundary = activation_boundary(session);
    session
        .activate_prepared_replacement(pending, catalog, boundary, None)
        .expect("candidate cuts over")
        .into_activation()
        .expect("changed Query-backed meaning publishes a successor")
}

pub(super) fn activation_boundary(
    session: &mut WorthUiActiveApplicationSession,
) -> WorthUiFrameBoundary {
    session
        .execute_framework_turn(|_| {})
        .into_completion()
        .into_execution()
        .unwrap_or_else(|_| panic!("empty turn publishes an execution boundary"))
        .into_activation_boundary()
}

pub(super) fn admit_active_resource(
    session: &mut WorthUiActiveApplicationSession,
    view: &WorthUiInstalledLiveQueryView,
    workspace: &mut runtime::WorthQueryWorkspace,
) {
    let resource = open_resource(view, workspace);
    let read = resource
        .read(workspace)
        .unwrap_or_else(|_| panic!("active live read stopped"));
    let projection = resource.project(&read, domain::project_facts().entity_identities());
    let mut admitted = false;
    let completion = session.execute_framework_turn(|turn| {
        turn.query_projection(|source| {
            admitted = source.admit_live_and_submit(resource, projection).is_ok();
        });
    });
    drop(completion.into_completion());
    assert!(admitted);
}

pub(super) fn admit_candidate_resource(
    candidate: &mut WorthUiPreparedApplicationReplacement,
    view: &WorthUiInstalledLiveQueryView,
    workspace: &mut runtime::WorthQueryWorkspace,
) {
    let resource = open_resource(view, workspace);
    let read = resource
        .read(workspace)
        .unwrap_or_else(|_| panic!("candidate live read stopped"));
    let projection = resource.project(&read, domain::project_facts().entity_identities());
    candidate
        .admit_candidate_live_query_projection(resource, projection)
        .expect("candidate owns its Query resource before publication");
}

pub(super) fn assert_visible_query_execution(session: &mut WorthUiActiveApplicationSession) {
    let summary = session
        .inspect_virtualized_plan(WorthUiVirtualizedPlanSummaryRequest::first_view())
        .expect("active Query summary");
    let target = summary.target(WorthUiVisibleRange::rows(0, 1).expect("visible range"));
    session
        .execute_framework_turn(|_| {})
        .into_execution()
        .unwrap_or_else(|_| panic!("empty turn executes"))
        .execute_virtualized_data_frame(target)
        .expect("visible Query frame executes");
}

pub(super) fn open_resource(
    view: &WorthUiInstalledLiveQueryView,
    workspace: &mut runtime::WorthQueryWorkspace,
) -> WorthUiQueryLiveResource {
    match view
        .open_using(domain::current(), workspace)
        .expect("installed authority matches")
    {
        WorthUiQueryLiveOpenOutcome::Opened(resource) => resource,
        WorthUiQueryLiveOpenOutcome::Stopped(_) => panic!("live resource open stopped"),
    }
}

pub(super) fn close(
    resource: WorthUiQueryLiveResource,
    workspace: &mut runtime::WorthQueryWorkspace,
) {
    assert!(matches!(
        resource.close(workspace),
        WorthUiQueryLiveCloseOutcome::Closed(_)
    ));
}

pub(super) fn close_retirement(
    retirement: WorthUiQueryLiveRetirement,
    workspace: &mut runtime::WorthQueryWorkspace,
) {
    let WorthUiQueryLiveRetirementCloseOutcome::Closed(receipt) = retirement.close(workspace)
    else {
        panic!("retired Query resource must close")
    };
    assert_eq!(receipt.closed_resource_count(), 1);
    let query_receipt = receipt
        .query_close_receipts()
        .next()
        .expect("Query owns the exact close proof");
    assert_eq!(
        query_receipt
            .close_receipt()
            .disposal_work()
            .lifecycle_closeout_count(),
        1
    );
}
