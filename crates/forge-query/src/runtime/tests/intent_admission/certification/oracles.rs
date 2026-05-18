use super::*;

#[test]
fn runtime_floor_certification_reports_oracle_parity_and_slope_evidence() {
    let bundle = certify_intent_admission_runtime_floor();
    let comparison_rows = bundle.oracle_report().comparison_rows();
    let support_rows = bundle.support_traceability_report().rows();
    let deferred_support_row = support_rows
        .iter()
        .find(|row| row.lane() == "deferred")
        .expect("deferred support traceability row should exist");
    let unsupported_support_row = support_rows
        .iter()
        .find(|row| row.lane() == "unsupported")
        .expect("unsupported support traceability row should exist");

    assert_eq!(bundle.oracle_report().manifest_rows().len(), 5);
    assert_eq!(comparison_rows.len(), 5);
    assert!(comparison_rows
        .iter()
        .all(|row| !row.row_digest().is_empty()));
    for row in comparison_rows {
        assert_eq!(
            row.expected_digest(),
            row.actual_digest(),
            "oracle lane {:?} must converge\nexpected:{}\nactual:{}",
            row.lane(),
            row.expected_detail(),
            row.actual_detail()
        );
    }
    assert_eq!(bundle.legacy_parity_report().rows().len(), 4);
    assert_eq!(support_rows.len(), 5);
    assert_eq!(
        deferred_support_row.family(),
        ForgeQueryIntentAdmissionFamily::InspectionMaterializationIntent.as_str()
    );
    assert_eq!(
        unsupported_support_row.family(),
        ForgeQueryIntentAdmissionFamily::InspectionMaterializationIntent.as_str()
    );
    assert!(
        deferred_support_row
            .support_detail()
            .starts_with("support:deferred:"),
        "deferred support lane must certify against the executable support matrix"
    );
    assert!(
        unsupported_support_row
            .support_detail()
            .starts_with("unsupported:"),
        "unsupported support lane must certify against coverage posture rather than a fake matrix row"
    );
    assert_ne!(
        deferred_support_row.row_digest(),
        unsupported_support_row.row_digest(),
        "deferred and unsupported support lanes must stay mechanically distinct"
    );
    assert_eq!(
        bundle.counter_snapshot().intent_family_lookup_width(),
        forge_query_intent_admission_family_inventory().rows().len()
    );
    assert_eq!(
        bundle.counter_snapshot().covered_entrypoint_lookup_width(),
        forge_query_intent_admission_coverage_inventory()
            .rows()
            .len()
    );
    assert!(!bundle
        .output_digest("admission_classification_slope_digest")
        .expect("slope output should exist")
        .is_empty());
    assert!(!bundle
        .output_digest("decision_trace_assembly_slope_digest")
        .expect("slope output should exist")
        .is_empty());
    assert!(!bundle
        .output_digest("legacy_delegation_parity_slope_digest")
        .expect("slope output should exist")
        .is_empty());
}
