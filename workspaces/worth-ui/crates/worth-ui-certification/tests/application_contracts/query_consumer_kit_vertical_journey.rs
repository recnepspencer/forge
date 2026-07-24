use worth_foundational::facade::CanonicalF32;
use worth_query::facade::{domain, runtime::WorthQueryWorkspace};
use worth_ui::facade::{app::WorthUiActiveApplicationSession, runtime::WorthUiQueryLaneFactLink};
use worth_ui_query_binding::{
    WorthUiInstalledQueryBindingReference, WorthUiQueryInspection,
    WorthUiQueryInspectionEvidencePolicy, WorthUiQueryInspectionRelevance, WorthUiQueryViewShape,
    WorthUiQueryWorkspaceExt, WorthUiSettledSnapshotFact, WorthUiSettledSnapshotProjection,
};

use crate::query_consumer_kit_workspace::{
    installed_measurement_workspace, interactive_borrowed_collection_requirements,
};
use crate::query_replacement_lifecycle::scenario::{
    snapshot_application, submission, FIRST_VIEW, NEXT_COMPONENT, SECOND_VIEW,
};
use worth_ui_certification::scenario::application_authority_closure::candidate_catalog::admit_candidate_catalog;

#[test]
fn ui_binding_and_runtime_converge_with_the_exact_query_projection() {
    let mut workspace = installed_measurement_workspace("public-query-binding-journey");
    let installed = workspace
        .worth_ui()
        .expect("the public extension resolves installed Worth UI authority");
    let view = installed
        .measurement_view(FIRST_VIEW)
        .expect("the installed domain derives one coherent view");
    let successor_view = installed
        .measurement_view(SECOND_VIEW)
        .expect("the installed domain derives one successor view");
    let view_identity = view.definition().identity().clone();
    let successor_identity = successor_view.definition().identity().clone();
    let app = snapshot_application(view, successor_view, &mut workspace);
    assert_eq!(app.capabilities().view_bindings().len(), 2);
    assert!(app.graph().node_count() > 0);
    let installed_reference = app
        .resolve_query_view(&view_identity, WorthUiQueryViewShape::Collection)
        .expect("the file-authored binding resolves its installed operation reference");
    let successor_reference = app
        .resolve_query_view(&successor_identity, WorthUiQueryViewShape::Collection)
        .expect("the successor binding resolves its installed operation reference");
    let settled = settle_reference(&installed_reference, &mut workspace);
    assert_query_projection(&settled);
    let expected_fact = settled.fact().clone();
    let mut session = app.launch().expect("the exact Query generation launches");
    let fact_link = session
        .query_fact_link(FIRST_VIEW)
        .expect("the active plan exposes one generation-owned fact link");

    let admitted_fact = admit_current_projection(&mut session, settled, &fact_link, &expected_fact);
    let refreshed_projection = settle_reference(&installed_reference, &mut workspace);
    assert_query_projection(&refreshed_projection);
    let refreshed_fact = refresh_current_projection(&mut session, refreshed_projection, &fact_link);
    assert_eq!(
        refreshed_fact.binding_reference(),
        admitted_fact.binding_reference()
    );
    assert_ne!(
        refreshed_fact.settlement_reference(),
        admitted_fact.settlement_reference()
    );
    activate_successor_projection(&mut session, &successor_reference, &mut workspace);
    assert_successor_projection(&mut session, &fact_link);
    let _shutdown = session.shutdown();
}

fn settle_reference(
    reference: &WorthUiInstalledQueryBindingReference,
    workspace: &mut WorthQueryWorkspace,
) -> WorthUiSettledSnapshotProjection {
    reference
        .enter_snapshot_attempt(workspace)
        .expect("the application reference enters the exact Query world")
        .prepare_snapshot_consumer(interactive_borrowed_collection_requirements())
        .expect("Query mints the one consumer contract")
        .execute(workspace)
        .unwrap()
        .publish()
        .unwrap()
        .consume()
        .unwrap()
        .settle()
        .unwrap()
}

fn assert_query_projection(settled: &WorthUiSettledSnapshotProjection) {
    assert_eq!(settled.fact().projected_measurement_fact_count(), 1);
    assert_eq!(settled.fact().result_state(), settled.result_state());
    assert_eq!(
        settled.fact().result_state(),
        domain::WorthQueryOperationResultState::Ready
    );
    assert_eq!(
        settled.fact().warning_count(),
        settled.execution_warnings().len()
            + settled
                .projection_warnings()
                .map_or(0, |warnings| warnings.warning_kinds().len())
    );
    let query_counters = settled.counters();
    assert_eq!(query_counters.runtime_authority_checks, 1);
    assert_eq!(query_counters.input_contract_checks, 1);
    assert_eq!(query_counters.executor_contacts, 1);
    assert_eq!(query_counters.publication_checks, 1);
    assert_eq!(query_counters.consumption_contacts, 1);
    let measurement_facts = settled.fact().measurement_facts();
    assert_eq!(measurement_facts.observations().len(), 1);
    assert_eq!(
        measurement_facts.observations()[0].extent(),
        CanonicalF32::from_f32(240.0)
    );
    let inspection = WorthUiQueryInspection::settled_projection(
        settled,
        WorthUiQueryInspectionRelevance::Relevant,
        WorthUiQueryInspectionEvidencePolicy::Rich,
    );
    assert!(std::ptr::eq(inspection.exact_projection(), settled));
    assert_eq!(inspection.counters().rich_evidence_section_count(), 1);
    assert_eq!(
        measurement_facts
            .refinement_counters()
            .admitted_observation_count(),
        1
    );
}

fn admit_current_projection(
    session: &mut WorthUiActiveApplicationSession,
    settled: WorthUiSettledSnapshotProjection,
    fact_link: &WorthUiQueryLaneFactLink,
    expected_fact: &WorthUiSettledSnapshotFact,
) -> std::sync::Arc<WorthUiSettledSnapshotFact> {
    let expected_extent = expected_fact.measurement_facts().observations()[0].extent();
    let mut admitted_fact = None;
    let mut frame_ingress = None;
    let completion = session
        .execute_framework_turn(|turn| {
            turn.query_projection(|query| {
                admitted_fact = Some(
                    query
                        .admit_settled(settled)
                        .expect("runtime retains the exact settled projection once"),
                );
                frame_ingress = Some(
                    query
                        .submit_settled(fact_link)
                        .expect("the active generation resolves its retained fact link"),
                );
            });
        })
        .expect("no mounted presentation lease is active");
    drop(completion.into_completion());
    let admitted_fact = admitted_fact.expect("the framework turn returns the admitted UI fact");
    assert_eq!(
        admitted_fact.settlement_reference(),
        expected_fact.settlement_reference()
    );
    assert_eq!(
        admitted_fact.binding_reference(),
        expected_fact.binding_reference()
    );
    assert_eq!(admitted_fact.result_state(), expected_fact.result_state());
    assert_eq!(
        admitted_fact.measurement_facts().observations()[0].extent(),
        expected_extent
    );
    assert_fact_coordinates(&admitted_fact, 1, 1);
    let frame_ingress = frame_ingress.expect("the requested settled fact entered the frame");
    assert_eq!(frame_ingress.counters().link_resolution_count(), 1);
    assert_eq!(frame_ingress.counters().retained_fact_resolution_count(), 1);
    assert_eq!(frame_ingress.counters().allocation_submission_count(), 1);
    assert!(frame_ingress.gateway().submission().is_some());
    assert_gateway_coordinates(&frame_ingress, 1, 1);
    assert_active_query_residue(session);
    admitted_fact
}

fn refresh_current_projection(
    session: &mut WorthUiActiveApplicationSession,
    settled: WorthUiSettledSnapshotProjection,
    fact_link: &WorthUiQueryLaneFactLink,
) -> std::sync::Arc<WorthUiSettledSnapshotFact> {
    let mut refreshed_fact = None;
    let mut frame_ingress = None;
    let completion = session
        .execute_framework_turn(|turn| {
            turn.query_projection(|query| {
                refreshed_fact = Some(
                    query
                        .refresh_settled(settled)
                        .expect("the exact binding atomically replaces its settlement"),
                );
                frame_ingress = Some(
                    query
                        .submit_settled(fact_link)
                        .expect("the unchanged plan link resolves the refreshed settlement"),
                );
            });
        })
        .expect("no mounted presentation lease is active");
    drop(completion.into_completion());
    let refreshed_fact = refreshed_fact.expect("the refresh returns the current UI fact");
    assert_fact_coordinates(&refreshed_fact, 2, 2);
    let frame_ingress = frame_ingress.expect("the refreshed fact entered the frame");
    assert_eq!(frame_ingress.counters().link_resolution_count(), 1);
    assert_eq!(frame_ingress.counters().retained_fact_resolution_count(), 1);
    assert_eq!(frame_ingress.counters().allocation_submission_count(), 1);
    assert!(frame_ingress.gateway().submission().is_some());
    assert_gateway_coordinates(&frame_ingress, 2, 2);
    assert_active_query_residue(session);
    refreshed_fact
}

fn assert_fact_coordinates(fact: &WorthUiSettledSnapshotFact, generation: u64, order: u64) {
    assert_eq!(
        fact.source_generation()
            .map(|coordinate| coordinate.as_u64()),
        Some(generation)
    );
    assert_eq!(
        fact.source_order().map(|coordinate| coordinate.as_u64()),
        Some(order)
    );
}

fn assert_gateway_coordinates(
    frame_ingress: &worth_ui::facade::runtime::WorthUiQueryFrameIngressOutcome,
    generation: u64,
    order: u64,
) {
    let ingress = frame_ingress
        .gateway()
        .evidence()
        .expect("successful allocation ingress carries admitted source evidence")
        .ingress();
    assert_eq!(ingress.key().source_generation().as_u64(), generation);
    assert_eq!(ingress.key().source_order().as_u64(), order);
}

fn assert_active_query_residue(session: &WorthUiActiveApplicationSession) {
    let active_scan = session.inspect_query_state_residue();
    assert!(active_scan.query_installed());
    assert_eq!(active_scan.scanned_query_bindings(), 2);
    assert_eq!(active_scan.scanned_plan_query_links(), 1);
    assert_eq!(active_scan.scanned_settled_snapshots(), 1);
    assert_eq!(active_scan.scanned_live_resources(), 0);
    assert_eq!(active_scan.operation_live_subsystem_construction_count(), 0);
    assert_eq!(active_scan.operation_live_succession_operation_count(), 0);
    assert!(active_scan.is_clean());
}

fn activate_successor_projection(
    session: &mut WorthUiActiveApplicationSession,
    successor_reference: &WorthUiInstalledQueryBindingReference,
    workspace: &mut WorthQueryWorkspace,
) {
    let prior_generation = session.generation_identity().clone();
    let mut prepared = session
        .prepare_replacement(submission(
            "query-consumer-kit-successor-source",
            NEXT_COMPONENT,
            &[SECOND_VIEW],
            session.capabilities(),
        ))
        .expect("the changed real file source prepares through the public session");
    prepared
        .admit_candidate_settled_query_projection(settle_reference(successor_reference, workspace))
        .expect("the successor candidate owns its independent exact Query settlement");
    let catalog = admit_candidate_catalog(session, &mut prepared);
    let lowered = session
        .lower_prepared_replacement(*prepared)
        .expect("the changed application lowers");
    let summary = lowered.summary();
    let cost = lowered.cost_envelope();
    assert_eq!(summary.affected_handle_count(), 2);
    assert_eq!(summary.query_rebind_entry_count(), 2);
    assert_eq!(cost.query_bindings_planned(), 2);
    let pending = session
        .stage_prepared_replacement(lowered)
        .expect("the changed application stages");
    let boundary = crate::query_replacement_lifecycle::support::activation_boundary(session);
    let cutover = session
        .activate_prepared_replacement(pending, catalog, boundary, None)
        .expect("the public replacement transaction succeeds")
        .into_activation()
        .expect("the changed application publishes exactly once");
    assert_eq!(cutover.prior_generation(), &prior_generation);
    assert_eq!(cutover.active_generation(), session.generation_identity());
    assert!(cutover.publication().generation_is_coherent());
    assert!(cutover.publication().host_is_coherent());
    assert!(cutover.operation_live_retirement().is_empty());
}

fn assert_successor_projection(
    session: &mut WorthUiActiveApplicationSession,
    predecessor_link: &WorthUiQueryLaneFactLink,
) {
    let successor_link = session
        .query_fact_link(SECOND_VIEW)
        .expect("the successor plan carries the candidate settlement link");
    let mut successor_ingress = None;
    let mut stale_denial = None;
    let completion = session
        .execute_framework_turn(|turn| {
            turn.query_projection(|query| {
                successor_ingress = Some(
                    query
                        .submit_settled(&successor_link)
                        .expect("the successor resolves the one preserved settlement"),
                );
                stale_denial = query.submit_settled(predecessor_link).err();
            });
        })
        .expect("no mounted presentation lease is active");
    drop(completion.into_completion());
    assert_eq!(
        stale_denial,
        Some(worth_ui::facade::runtime::WorthUiQueryFrameIngressDenial::StaleApplicationGeneration)
    );
    assert_eq!(
        successor_ingress
            .unwrap()
            .counters()
            .link_resolution_count(),
        1
    );
    let successor_scan = session.inspect_query_state_residue();
    assert_eq!(successor_scan.scanned_settled_snapshots(), 1);
    assert_eq!(successor_scan.scanned_plan_query_links(), 1);
    assert!(successor_scan.is_clean());
}
