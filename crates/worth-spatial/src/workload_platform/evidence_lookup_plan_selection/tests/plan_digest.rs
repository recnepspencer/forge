use super::fixtures::PlanSelectionSubject;

#[test]
fn same_authority_and_catalog_produce_same_lookup_plan_digest() {
    let subject = PlanSelectionSubject::event_ledger();

    let left = subject.select_event_plan();
    let right = subject.select_event_plan();

    assert_eq!(left.selected_plan_digest(), right.selected_plan_digest());
    assert_eq!(left.admitted_input_digest(), right.admitted_input_digest());
    assert_eq!(left.catalog_digest(), subject.catalog().catalog_digest());
}
