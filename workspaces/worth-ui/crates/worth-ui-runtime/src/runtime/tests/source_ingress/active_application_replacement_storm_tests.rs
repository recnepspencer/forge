use worth_ui_inspection::{UiInspectionQuery, UiInspectionScope, UiInspectionTarget};

use crate::runtime::tests::active_application_session_test_support::{
    admit_candidate_catalog, component_candidate_submission, source_backed_component_session,
};
use crate::runtime::{
    WorthUiSourceProvider, WorthUiWatchedCandidateSubmissionDenial, WorthUiWatcherEvent,
};

const STORM_STEP_COUNT: usize = 1_000;

#[test]
fn active_application_storm_never_exposes_mixed_generation_truth() {
    let mut session = source_backed_component_session();
    let host_session = session.host_session_identity();
    let mut active_component = "workspace.component.active_session_current";

    for step in 0..STORM_STEP_COUNT {
        let prior_generation = session.generation_identity().clone();
        match step % 4 {
            0 => assert_equivalent_candidate_discard(&session, active_component, step),
            1 | 3 => assert_malformed_source_denial(&session, step),
            2 => {
                active_component = alternate_component(active_component);
                publish_structural_replacement(&mut session, active_component, step);
            }
            _ => unreachable!(),
        }
        if step % 4 != 2 {
            assert_eq!(session.generation_identity(), &prior_generation);
        }
        assert_coherent_active_projections(&session, host_session);
    }
}

fn assert_equivalent_candidate_discard(
    session: &crate::facade::WorthUiActiveApplicationSession,
    active_component: &str,
    step: usize,
) {
    let outcome = session
        .prepare_replacement(component_candidate_submission(
            session,
            &format!("active-storm-noop-{step}"),
            active_component,
        ))
        .expect("equivalent storm candidate should admit");
    let prepared = outcome;
    let lowered = session
        .lower_prepared_replacement(*prepared)
        .expect("equivalent storm candidate continues through lowering");
    let pending = session
        .stage_prepared_replacement(lowered)
        .expect("equivalent storm candidate reaches staged authority");
    drop(pending);
}

fn assert_malformed_source_denial(
    session: &crate::facade::WorthUiActiveApplicationSession,
    step: usize,
) {
    let source_name = format!("active-storm-invalid-{step}");
    let provider = WorthUiSourceProvider::in_memory(source_name.clone())
        .with_file("app/main.wui", "component MissingBrace {");
    let mut source = session.source_event_ingress(provider).start();
    let denial = source
        .ingest([WorthUiWatcherEvent::provider_revision(source_name)])
        .expect("malformed material should still debounce")
        .lower_to_candidate_submission(session.capabilities())
        .expect_err("malformed storm candidate must deny before preparation");
    let WorthUiWatchedCandidateSubmissionDenial::DslCompilation(report) = denial else {
        panic!("malformed source must remain localized to DSL compilation");
    };
    assert_eq!(
        report.diagnostics()[0].identity().code(),
        worth_ui_dsl::WorthUiDslCompileDiagnosticCode::UnterminatedBlock
    );
}

fn publish_structural_replacement(
    session: &mut crate::facade::WorthUiActiveApplicationSession,
    next_component: &str,
    step: usize,
) {
    let outcome = session
        .prepare_replacement(component_candidate_submission(
            session,
            &format!("active-storm-success-{step}"),
            next_component,
        ))
        .expect("structural storm candidate should prepare");
    let mut prepared = outcome;
    let catalog = admit_candidate_catalog(&mut prepared);
    let lowered = session
        .lower_prepared_replacement(*prepared)
        .expect("storm candidate should lower");
    let pending = session
        .stage_prepared_replacement(lowered)
        .expect("storm candidate should stage");
    let boundary = session
        .execute_framework_turn(|_| {})
        .expect("no mounted presentation lease is active")
        .into_completion()
        .into_execution()
        .expect("storm boundary turn should complete")
        .into_activation_boundary();
    let cutover = session
        .activate_prepared_replacement(pending, catalog, boundary, None)
        .unwrap_or_else(|denial| panic!("storm cutover {step} should publish: {denial:?}"));
    let cutover = cutover
        .into_activation()
        .expect("storm candidates change executable meaning");
    assert!(cutover.operation_live_retirement().is_empty());
    assert_eq!(
        cutover
            .allocation_catalog_successor()
            .counters()
            .carried_row_visits(),
        0
    );
}

fn assert_coherent_active_projections(
    session: &crate::facade::WorthUiActiveApplicationSession,
    host_session: crate::facade::WorthUiHostSessionIdentity,
) {
    let generation = session.generation_identity();
    assert_eq!(session.inspect_runtime().generation_identity(), generation);
    let inspection = session.inspect(UiInspectionQuery::new(
        UiInspectionTarget::product_root(),
        UiInspectionScope::graph(),
    ));
    assert_eq!(inspection.generation_identity(), generation);
    assert_eq!(session.host_session_identity(), host_session);
    let graph = session.graph();
    let nodes = graph.node_identities().collect::<Vec<_>>();
    assert_eq!(nodes.len(), graph.node_count());
    assert!(nodes
        .iter()
        .all(|node| graph.lookup().graph_node(*node).is_some()));
}

fn alternate_component(current: &str) -> &'static str {
    if current == "workspace.component.active_session_current" {
        "workspace.component.active_session_candidate"
    } else {
        "workspace.component.active_session_current"
    }
}
