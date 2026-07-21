use worth_foundational::facade::{AspectValue, CanonicalF32};
use worth_query::facade::domain;
use worth_ui::facade::query_binding::{
    WorthUiQueryAllocationDetail, WorthUiQueryConsumerRequirements, WorthUiQueryDenialPresentation,
    WorthUiQueryViewIdentity, WorthUiQueryViewShape, WorthUiQueryWorkspaceExt,
};
use worth_ui_query_binding::{
    WorthUiQueryInspection, WorthUiQueryInspectionEvidencePolicy, WorthUiQueryInspectionRelevance,
};

use crate::query_consumer_kit_workspace::{
    installed_measurement_workspace, measurement_value_path, observation_basis,
};
use crate::query_replacement_lifecycle::scenario::{
    snapshot_application, submission, FIRST_VIEW, NEXT_COMPONENT, SECOND_VIEW,
};
use worth_ui_certification::scenario::application_authority_closure::candidate_catalog::admit_candidate_catalog;

#[test]
fn public_file_authored_application_consumes_exact_query_progression_into_runtime() {
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
    let app = snapshot_application(view, successor_view);
    assert_eq!(app.capabilities().view_bindings().len(), 2);
    assert!(app.graph().node_count() > 0);
    let installed_reference = app
        .resolve_query_view(
            &WorthUiQueryViewIdentity::new(FIRST_VIEW).unwrap(),
            WorthUiQueryViewShape::Collection,
        )
        .expect("the file-authored binding resolves its installed operation reference");
    let successor_reference = app
        .resolve_query_view(
            &WorthUiQueryViewIdentity::new(SECOND_VIEW).unwrap(),
            WorthUiQueryViewShape::Collection,
        )
        .expect("the successor binding resolves its installed operation reference");
    let settled = installed_reference
        .enter_snapshot_attempt(&workspace, observation_basis())
        .expect("the application reference enters the exact Query world")
        .prepare_snapshot_consumer(WorthUiQueryConsumerRequirements::new(
            domain::WorthQueryConsumerBoundaryRequirements {
                presentation: domain::WorthQueryConsumerPresentationPosture::Interactive,
                allocation: domain::WorthQueryConsumerAllocationPosture::Borrowed,
            },
            WorthUiQueryAllocationDetail::BorrowedFactSlice,
            WorthUiQueryViewShape::Collection,
            WorthUiQueryDenialPresentation::StructuredStatus,
            WorthUiQueryInspectionRelevance::Relevant,
        ))
        .expect("Query mints the one consumer contract")
        .execute(&mut workspace)
        .unwrap()
        .publish()
        .unwrap()
        .consume(domain::project_facts().display_field(measurement_value_path()))
        .unwrap()
        .settle()
        .unwrap();
    assert_eq!(settled.fact().projected_measurement_fact_count(), 1);
    let query_counters = settled.counters();
    assert_eq!(query_counters.runtime_authority_checks, 1);
    assert_eq!(query_counters.input_contract_checks, 1);
    assert_eq!(query_counters.executor_contacts, 1);
    assert_eq!(query_counters.publication_checks, 1);
    assert_eq!(query_counters.consumption_contacts, 1);
    let measurement_facts = settled
        .fact()
        .measurement_facts()
        .expect("the settled Query fact derives the admitted UI measurement subset");
    assert_eq!(measurement_facts.observations().len(), 1);
    assert_eq!(
        measurement_facts.observations()[0].extent(),
        CanonicalF32::from_f32(240.0)
    );
    let inspection = WorthUiQueryInspection::settled_projection(
        &settled,
        WorthUiQueryInspectionRelevance::Relevant,
        WorthUiQueryInspectionEvidencePolicy::Rich,
    );
    assert!(std::ptr::eq(inspection.exact_projection(), &settled));
    assert_eq!(inspection.counters().rich_evidence_section_count(), 1);
    assert_eq!(
        measurement_facts
            .refinement_counters()
            .admitted_observation_count(),
        1
    );
    assert_eq!(
        settled
            .exact_query_projection()
            .authority()
            .facts()
            .display_fields()[0]
            .native_value()
            .scalar(),
        Some(&AspectValue::Float32(CanonicalF32::from_f32(240.0)))
    );
    let expected_fact = settled.fact().clone();
    let mut session = app.launch().expect("the exact Query generation launches");
    let fact_link = session
        .query_fact_link(FIRST_VIEW)
        .expect("the active plan exposes one generation-owned fact link");
    let mut admitted_fact = None;
    let mut frame_ingress = None;
    let completion = session.execute_framework_turn(|turn| {
        turn.query_projection(|query| {
            admitted_fact = Some(
                query
                    .admit_settled(settled)
                    .expect("runtime retains the exact settled projection once"),
            );
            frame_ingress = Some(
                query
                    .submit_settled(&fact_link)
                    .expect("the active generation resolves its retained fact link"),
            );
        });
    });
    drop(completion.into_completion());
    let admitted_fact = admitted_fact.expect("the framework turn returns the admitted UI fact");
    assert_eq!(
        admitted_fact.settlement_identity(),
        expected_fact.settlement_identity()
    );
    assert_eq!(
        admitted_fact.query_binding_identity(),
        expected_fact.query_binding_identity()
    );
    assert_eq!(
        admitted_fact
            .source_generation()
            .map(|generation| generation.as_u64()),
        Some(1)
    );
    let frame_ingress = frame_ingress.expect("the requested settled fact entered the frame");
    assert_eq!(frame_ingress.counters().link_resolution_count(), 1);
    assert_eq!(frame_ingress.counters().retained_fact_resolution_count(), 1);
    assert_eq!(frame_ingress.counters().allocation_submission_count(), 1);
    assert!(frame_ingress.gateway().submission().is_some());
    let active_scan = session.inspect_query_state_residue();
    assert!(active_scan.query_installed());
    assert_eq!(active_scan.scanned_query_bindings(), 2);
    assert_eq!(active_scan.scanned_plan_query_links(), 1);
    assert_eq!(active_scan.scanned_settled_snapshots(), 1);
    assert_eq!(active_scan.scanned_live_resources(), 0);
    assert_eq!(active_scan.managed_live_subsystem_construction_count(), 0);
    assert_eq!(active_scan.managed_live_succession_operation_count(), 0);
    assert!(active_scan.is_clean());

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
        .admit_candidate_settled_query_projection(
            successor_reference
                .enter_snapshot_attempt(&workspace, observation_basis())
                .unwrap()
                .prepare_snapshot_consumer(WorthUiQueryConsumerRequirements::new(
                    domain::WorthQueryConsumerBoundaryRequirements {
                        presentation: domain::WorthQueryConsumerPresentationPosture::Interactive,
                        allocation: domain::WorthQueryConsumerAllocationPosture::Borrowed,
                    },
                    WorthUiQueryAllocationDetail::BorrowedFactSlice,
                    WorthUiQueryViewShape::Collection,
                    WorthUiQueryDenialPresentation::StructuredStatus,
                    WorthUiQueryInspectionRelevance::Relevant,
                ))
                .unwrap()
                .execute(&mut workspace)
                .unwrap()
                .publish()
                .unwrap()
                .consume(domain::project_facts().display_field(measurement_value_path()))
                .unwrap()
                .settle()
                .unwrap(),
        )
        .expect("the successor candidate owns its independent exact Query settlement");
    let catalog = admit_candidate_catalog(&session, &mut prepared);
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
    let boundary = crate::query_replacement_lifecycle::support::activation_boundary(&mut session);
    let cutover = session
        .activate_prepared_replacement(pending, catalog, boundary, None)
        .expect("the public replacement transaction succeeds")
        .into_activation()
        .expect("the changed application publishes exactly once");
    assert_eq!(cutover.prior_generation(), &prior_generation);
    assert_eq!(cutover.active_generation(), session.generation_identity());
    assert!(cutover.publication().generation_is_coherent());
    assert!(cutover.publication().host_is_coherent());
    assert!(cutover.managed_live_compatibility_retirement().is_empty());

    let successor_link = session
        .query_fact_link(SECOND_VIEW)
        .expect("the successor plan carries the candidate settlement link");
    let mut successor_ingress = None;
    let mut stale_denial = None;
    let completion = session.execute_framework_turn(|turn| {
        turn.query_projection(|query| {
            successor_ingress = Some(
                query
                    .submit_settled(&successor_link)
                    .expect("the successor resolves the one preserved settlement"),
            );
            stale_denial = query.submit_settled(&fact_link).err();
        });
    });
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
    let _shutdown = session.shutdown();
}
