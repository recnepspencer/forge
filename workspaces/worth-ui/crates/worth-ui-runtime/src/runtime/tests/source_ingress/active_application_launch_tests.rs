use worth_ui_inspection::{
    UiEvidenceRichness, UiInspectionQuery, UiInspectionScope, UiInspectionTarget,
};

use crate::capability::WorthUiQueryViewRegistration;
use crate::facade::WorthUi;
use crate::runtime::tests::active_application_session_test_support::source_backed_component_session;
use crate::runtime::tests::source_ingress_boundary_test_support::lower_file_submission;
use crate::runtime::tests::source_ingress_test_support::file_import_provider;
use crate::runtime::WorthUiWatcherEvent;

#[test]
fn launch_and_ordinary_frame_bind_runtime_and_inspection_to_one_generation() {
    let mut session = source_backed_component_session();
    let generation = session.generation_identity().clone();
    let inspection = session.inspect(UiInspectionQuery::new(
        UiInspectionTarget::product_root(),
        UiInspectionScope::graph(),
    ));
    let completion = session
        .execute_framework_turn(|_| {})
        .expect("no mounted presentation lease is active");
    let completion_generation = completion.generation_identity().clone();
    drop(completion);

    assert_eq!(session.generation_identity(), &generation);
    assert_eq!(session.inspect_runtime().generation_identity(), &generation);
    assert_eq!(inspection.generation_identity(), &generation);
    assert_eq!(completion_generation, generation);
}

#[test]
fn active_session_expands_evidence_from_its_current_application_generation() {
    let session = source_backed_component_session();
    let graph_node_identity = session
        .graph()
        .node_identities()
        .next()
        .expect("source-backed active application should have a graph node");
    let evidence_ref = session
        .graph()
        .evidence_ref_for_node(graph_node_identity)
        .expect("active graph authority should derive its node evidence ref");

    let expansion = session.expand_evidence_ref(evidence_ref, UiEvidenceRichness::summary());

    assert!(expansion.outcome().is_available());
    assert!(expansion.followup_query().is_some());
}

#[test]
fn query_free_and_installed_query_apps_share_the_active_session_lifecycle() {
    let query_free = source_backed_component_session();
    let installed = worth_ui_query_binding::certification::worth_ui_installed_test_domain(
        "active-session-query-parity",
    );
    let view = installed
        .live_measurement_view("workspace.view_binding.active_session")
        .expect("installed Query view should admit");
    let builder = WorthUi::app()
        .register_query_view(WorthUiQueryViewRegistration::new(view))
        .expect("installed Query view should register");
    let snapshot = builder.freeze().expect("Query snapshot should prepare");
    let submission = lower_file_submission(
        file_import_provider(),
        [WorthUiWatcherEvent::modified("app/main.wui")],
        snapshot.capabilities(),
    );
    let mut query_installed = WorthUi::app()
        .register_query_view(WorthUiQueryViewRegistration::new(
            installed
                .live_measurement_view("workspace.view_binding.active_session")
                .expect("installed Query view should admit again"),
        ))
        .expect("installed Query view should register again")
        .with_candidate_submission(submission)
        .freeze()
        .expect("Query-installed source app should prepare")
        .launch()
        .expect("Query-installed source app should launch");

    let query_generation = query_installed.generation_identity().clone();
    let completion = query_installed
        .execute_framework_turn(|_| {})
        .expect("no mounted presentation lease is active");
    let completion_generation = completion.generation_identity().clone();
    drop(completion);

    assert_eq!(completion_generation, query_generation);
    assert_eq!(
        query_installed.inspect_runtime().generation_identity(),
        &query_generation
    );
    assert_ne!(
        query_free.generation_identity(),
        query_installed.generation_identity()
    );
    assert_eq!(
        snapshot.capabilities().digest(),
        query_installed.capabilities().digest()
    );
}
