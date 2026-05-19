use super::*;

#[test]
fn intent_admission_certification_reports_oracle_parity_and_slope_evidence() {
    let bundle = certify_intent_admission();
    let comparison_rows = bundle.oracle_report().comparison_rows();
    let support_rows = bundle.support_traceability_report().rows();
    let comparison_lanes = comparison_rows
        .iter()
        .map(|row| row.lane())
        .collect::<Vec<_>>();
    let support_lane_map = support_rows
        .iter()
        .map(|row| {
            (
                row.lane(),
                row.family(),
                row.entrypoint(),
                row.support_detail(),
            )
        })
        .collect::<Vec<_>>();
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
    assert_eq!(
        comparison_lanes,
        vec![
            ForgeQueryIntentAdmissionOracleLane::AdmittedControl,
            ForgeQueryIntentAdmissionOracleLane::AdvisoryControl,
            ForgeQueryIntentAdmissionOracleLane::ViolationControl,
            ForgeQueryIntentAdmissionOracleLane::DeferredControl,
            ForgeQueryIntentAdmissionOracleLane::UnsupportedControl,
        ]
    );
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
    assert_eq!(bundle.legacy_parity_report().rows().len(), 6);
    for row in bundle.legacy_parity_report().rows() {
        let expected_labels = match row.lane() {
            ForgeQueryIntentAdmissionLegacyParityLane::AuthoritativeExecution
            | ForgeQueryIntentAdmissionLegacyParityLane::EffectExecution => {
                vec!["decision", "handoff", "binding", "provenance", "result"]
            }
            ForgeQueryIntentAdmissionLegacyParityLane::ReadExecutionCurrent
            | ForgeQueryIntentAdmissionLegacyParityLane::ReadExecutionInBasisContext
            | ForgeQueryIntentAdmissionLegacyParityLane::RoutingExecutionRuntime
            | ForgeQueryIntentAdmissionLegacyParityLane::RoutingExecutionWorkspace => {
                vec!["trace", "provenance", "result"]
            }
        };
        assert_eq!(
            row.checks()
                .iter()
                .map(|check| check.label())
                .collect::<Vec<_>>(),
            expected_labels,
            "legacy parity lane {:?} must retain its exact certified checks",
            row.lane()
        );
        assert!(
            row.all_checks_pass(),
            "legacy parity lane {:?} must pass every check: {:?}",
            row.lane(),
            row.checks()
                .iter()
                .map(|check| format!("{}={}", check.label(), check.passed()))
                .collect::<Vec<_>>()
        );
    }
    assert_eq!(support_rows.len(), 6);
    assert_eq!(
        support_lane_map,
        vec![
            (
                "admitted",
                ForgeQueryIntentAdmissionFamily::AuthoritativeUserIntent.as_str(),
                ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteIntent.as_str(),
                support_rows[0].support_detail(),
            ),
            (
                "advisory",
                ForgeQueryIntentAdmissionFamily::AuthoritativeUserIntent.as_str(),
                ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteIntent.as_str(),
                support_rows[1].support_detail(),
            ),
            (
                "violation",
                ForgeQueryIntentAdmissionFamily::EffectTriggeredWriteIntent.as_str(),
                ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteNextEffectWriteIntent.as_str(),
                support_rows[2].support_detail(),
            ),
            (
                "routing_admitted",
                ForgeQueryIntentAdmissionFamily::LowerRuntimeCapabilityRoutingIntent.as_str(),
                ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteExistingTruthProbeRouting
                    .as_str(),
                support_rows[3].support_detail(),
            ),
            (
                "deferred",
                ForgeQueryIntentAdmissionFamily::InspectionMaterializationIntent.as_str(),
                ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteInspectionNeighborDeferred
                    .as_str(),
                support_rows[4].support_detail(),
            ),
            (
                "unsupported",
                ForgeQueryIntentAdmissionFamily::InspectionMaterializationIntent.as_str(),
                ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteInspectionNeighborDeferred
                    .as_str(),
                support_rows[5].support_detail(),
            ),
        ]
    );
    let routing_support_row = support_rows
        .iter()
        .find(|row| row.lane() == "routing_admitted")
        .expect("routing admitted support traceability row should exist");
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
    assert_eq!(
        routing_support_row.family(),
        ForgeQueryIntentAdmissionFamily::LowerRuntimeCapabilityRoutingIntent.as_str()
    );
    assert!(
        routing_support_row
            .support_detail()
            .starts_with("support:admitted:"),
        "routing admitted lane must certify against the executable support matrix"
    );
    assert!(
        routing_support_row
            .support_detail()
            .contains("implemented-existing-truth-probe-routing-floor"),
        "routing admitted lane must retain the concrete implemented routing-floor detail"
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
