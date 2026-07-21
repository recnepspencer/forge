use worth_ui_inspection::{UiInspectionQuery, UiInspectionScope, UiInspectionTarget};

use crate::runtime::tests::active_application_session_test_support::{
    admit_candidate_catalog, component_candidate_submission, source_backed_component_session,
};

#[test]
fn successful_cutover_publishes_runtime_app_and_inspection_as_one_generation() {
    let mut session = source_backed_component_session();
    let prior_generation = session.generation_identity().clone();
    let mut prepared = session
        .prepare_replacement(component_candidate_submission(
            &session,
            "active-session-successor",
            "workspace.component.active_session_candidate",
        ))
        .expect("successor candidate should prepare");
    let catalog = admit_candidate_catalog(&mut prepared);
    let successor_generation = prepared.inspect_candidate(UiInspectionQuery::new(
        UiInspectionTarget::product_root(),
        UiInspectionScope::graph(),
    ));
    let successor_generation = successor_generation.generation_identity().clone();
    let lowered = session
        .lower_prepared_replacement(*prepared)
        .expect("prepared successor should lower");
    let candidate_summary = lowered.summary();
    let candidate_cost = lowered.cost_envelope();
    assert_eq!(candidate_summary.active_generation(), &prior_generation);
    assert_eq!(
        candidate_summary.candidate_generation(),
        &successor_generation
    );
    assert_eq!(candidate_summary, lowered.summary());
    assert_eq!(candidate_cost, lowered.cost_envelope());
    assert!(candidate_summary.replacement_classification_count() > 0);
    assert!(candidate_cost.admission_checks() > 0);
    let pending = session
        .stage_prepared_replacement(lowered)
        .expect("lowered successor should stage");
    let boundary = session
        .execute_framework_turn(|_| {})
        .into_completion()
        .into_execution()
        .expect("empty framework turn should yield an activation boundary")
        .into_activation_boundary();
    let receipt = session
        .activate_prepared_replacement(pending, catalog, boundary, None)
        .expect("candidate-owned catalog should cut over atomically");
    let active_inspection = session.inspect(UiInspectionQuery::new(
        UiInspectionTarget::product_root(),
        UiInspectionScope::graph(),
    ));
    let receipt = receipt
        .into_activation()
        .expect("changed executable meaning publishes a successor");
    let catalog_successor = receipt.allocation_catalog_successor();
    assert_eq!(catalog_successor.predecessor_rows(), 0);
    assert_eq!(catalog_successor.carried_rows(), 0);
    assert_eq!(catalog_successor.counters().carried_row_visits(), 0);
    assert_eq!(
        catalog_successor.successor_rows(),
        catalog_successor.transitions().len()
    );
    assert!(catalog_successor.transitions().iter().all(|transition| {
        transition.disposition()
            == crate::runtime::exports::UiAllocationCatalogRowDisposition::Inserted
    }));
    assert_eq!(receipt.prior_generation(), &prior_generation);
    assert_eq!(receipt.active_generation(), &successor_generation);
    assert!(receipt.publication().generation_is_coherent());
    assert!(receipt.publication().host_is_coherent());
    assert_eq!(
        receipt.publication().application_generation(),
        &successor_generation
    );
    assert_eq!(
        receipt.publication().runtime().active_plan_digest(),
        receipt.plan_swap().next_active_plan_digest()
    );
    assert_eq!(
        receipt.publication().runtime().artifact_digest(),
        receipt.plan_swap().next_active_artifact_digest()
    );
    assert!(matches!(
        receipt.publication().scheduler(),
        crate::runtime::UiAllocationFrameDispatcherState::Open(_)
    ));
    let reload_cost = receipt
        .reload_cost()
        .expect("public cutover carries counters from its real replacement phases");
    let context = reload_cost
        .context()
        .expect("production counters carry scope");
    assert_eq!(context.active_generation(), &prior_generation);
    assert_eq!(context.candidate_generation(), &successor_generation);
    assert_eq!(
        context.active_plan_digest(),
        receipt.plan_swap().previous_active_plan_digest()
    );
    assert_eq!(
        context.candidate_plan_digest(),
        receipt.plan_swap().next_active_plan_digest()
    );
    assert_eq!(
        context.candidate_artifact_digest(),
        receipt.plan_swap().next_active_artifact_digest()
    );
    let foundational = reload_cost
        .foundational_evidence()
        .expect("production reload counters certify before Foundational projection");
    assert_eq!(foundational.receipt_count(), reload_cost.packets().len());
    assert_eq!(session.generation_identity(), &successor_generation);
    assert_eq!(
        session.inspect_runtime().generation_identity(),
        &successor_generation
    );
    assert_eq!(
        active_inspection.generation_identity(),
        &successor_generation
    );
}

#[test]
fn foreign_catalog_denial_preserves_active_and_candidate_inspection_scopes() {
    let mut session = source_backed_component_session();
    let active_generation = session.generation_identity().clone();
    let prepared = session
        .prepare_replacement(component_candidate_submission(
            &session,
            "active-session-foreign-catalog",
            "workspace.component.active_session_candidate",
        ))
        .expect("structurally different candidate should prepare");
    let candidate = prepared.inspect_candidate(UiInspectionQuery::new(
        UiInspectionTarget::product_root(),
        UiInspectionScope::graph(),
    ));
    assert_ne!(candidate.generation_identity(), &active_generation);
    let lowered = session
        .lower_prepared_replacement(*prepared)
        .expect("prepared application basis should lower");
    let pending = session
        .stage_prepared_replacement(lowered)
        .expect("lowered application replacement should stage");
    let (foreign_snapshot, first, second) =
        crate::runtime::tests::allocation_catalog_test_support::admitted_disjoint_planning_admissions(
            "active-session.foreign-catalog",
        );
    let foreign_catalog = foreign_snapshot
        .admit_allocation_catalog_delta(vec![first, second], vec![])
        .expect("foreign graph should admit its own allocation delta");
    let boundary = session
        .execute_framework_turn(|_| {})
        .into_completion()
        .into_execution()
        .expect("empty framework turn should yield an activation boundary")
        .into_activation_boundary();
    let denial =
        match session.activate_prepared_replacement(pending, foreign_catalog, boundary, None) {
            Ok(_) => panic!("catalog from a different graph authority must deny"),
            Err(denial) => denial,
        };
    assert!(matches!(
        denial,
        crate::facade::WorthUiApplicationCutoverDenial::PreparedApplicationGraphMismatch
    ));
    assert_eq!(session.generation_identity(), &active_generation);
    assert_eq!(
        session.inspect_runtime().generation_identity(),
        &active_generation
    );
}
