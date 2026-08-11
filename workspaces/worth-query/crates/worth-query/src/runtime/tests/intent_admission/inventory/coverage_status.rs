use super::*;

#[test]
fn coverage_inventory_marks_projection_and_inspection_advisory_classes_as_exercised() {
    let inventory = worth_query_intent_admission_coverage_inventory();
    let projection = inventory
        .rows()
        .iter()
        .find(|row| {
            row.entrypoint() == WorthQueryIntentAdmissionCoveredEntrypoint::ProjectionConsumption
        })
        .expect("projection row should exist");
    let inspection = inventory
        .rows()
        .iter()
        .find(|row| {
            row.entrypoint() == WorthQueryIntentAdmissionCoveredEntrypoint::ExecuteUnifiedInspection
        })
        .expect("unified inspection row should exist");

    assert_eq!(
        projection.advisory_decision_class(),
        WorthQueryIntentAdmissionDecisionClass::ProjectionWarningBearingAdmission
    );
    assert_eq!(
        inspection.advisory_decision_class(),
        WorthQueryIntentAdmissionDecisionClass::InspectionDetailRedactionAdvisory
    );
}
