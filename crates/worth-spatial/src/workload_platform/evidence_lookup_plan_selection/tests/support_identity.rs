use super::super::{
    select_evidence_lookup_plan, EvidenceLookupPlanQuerySurface, EvidenceLookupPlanRowOutcome,
    EvidenceLookupPlanSelectionErrorKind,
};
use super::fixtures::{query_import_for_family, PlanSelectionSubject};

const QUERY_REQUIRED_EVENT_SIBLING: &str =
    "spatial-touch.boolean.event-ledger-query-required-sibling.v1";

#[test]
fn support_row_order_does_not_change_identity_bound_selected_plan() {
    let subject = PlanSelectionSubject::event_ledger_with_query_required_sibling();
    let query_import = query_import_for_family(subject.catalog(), QUERY_REQUIRED_EVENT_SIBLING);
    let admitted = subject.admit_with_query_import(&query_import);
    let reordered = admitted.with_reversed_support_for_plan_selection_tests();

    let plan = select_evidence_lookup_plan(subject.catalog(), &admitted).expect("plan selects");
    let reordered_plan =
        select_evidence_lookup_plan(subject.catalog(), &reordered).expect("reordered plan selects");

    assert_eq!(
        plan.selected_plan_digest(),
        reordered_plan.selected_plan_digest()
    );
    assert_eq!(plan.counters().selected_family_count(), 2);
    assert_eq!(plan.counters().selected_family_membership_probe_count(), 2);
    assert_eq!(plan.counters().topology_support_rows_consumed_count(), 2);
    assert_eq!(plan.counters().query_support_rows_consumed_count(), 2);
    assert_eq!(plan.counters().raw_evidence_row_scan_count(), 0);
    assert_eq!(plan.counters().broad_receipt_scan_count(), 0);

    let query_required_row = plan
        .rows()
        .iter()
        .find(|row| row.family_identity() == QUERY_REQUIRED_EVENT_SIBLING)
        .expect("query-required sibling row exists");
    assert_eq!(
        query_required_row.outcome(),
        EvidenceLookupPlanRowOutcome::Selected
    );
    assert_eq!(
        query_required_row.query_posture().surface(),
        EvidenceLookupPlanQuerySurface::ConsumerKitSupportPin
    );
}

#[test]
fn duplicate_admitted_support_family_rejects_before_strategy_selection() {
    let subject = PlanSelectionSubject::event_ledger_with_query_required_sibling();
    let query_import = query_import_for_family(subject.catalog(), QUERY_REQUIRED_EVENT_SIBLING);
    let admitted = subject
        .admit_with_query_import(&query_import)
        .with_duplicate_query_support_for_plan_selection_tests();

    let error = select_evidence_lookup_plan(subject.catalog(), &admitted)
        .expect_err("duplicate support identity must reject");

    assert_eq!(
        error.kind(),
        EvidenceLookupPlanSelectionErrorKind::DuplicateAdmittedSupportFamily
    );
    assert_eq!(error.counters().selected_family_count(), 0);
}

#[test]
fn missing_admitted_support_family_rejects_before_query_posture_fallback() {
    let subject = PlanSelectionSubject::event_ledger_with_query_required_sibling();
    let query_import = query_import_for_family(subject.catalog(), QUERY_REQUIRED_EVENT_SIBLING);
    let admitted = subject
        .admit_with_query_import(&query_import)
        .without_query_support_for_plan_selection_tests();

    let error = select_evidence_lookup_plan(subject.catalog(), &admitted)
        .expect_err("missing identity-bound query support must reject");

    assert_eq!(
        error.kind(),
        EvidenceLookupPlanSelectionErrorKind::MissingAdmittedSupportFamily
    );
    assert_eq!(error.counters().selected_family_count(), 0);
    assert_eq!(error.counters().query_support_rows_consumed_count(), 0);
}
