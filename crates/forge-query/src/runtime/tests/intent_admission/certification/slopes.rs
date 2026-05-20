use super::*;

#[test]
fn intent_admission_certification_reports_small_medium_and_large_width_runs_for_each_lane() {
    let bundle = certify_intent_admission();
    let slope_report = bundle.slope_report();
    let width_runs = slope_report.width_runs();

    assert_eq!(width_runs.len(), 21);

    for lane in [
        ForgeQueryIntentAdmissionSlopeLane::AdmissionClassification,
        ForgeQueryIntentAdmissionSlopeLane::DecisionTraceAssembly,
        ForgeQueryIntentAdmissionSlopeLane::DecisionSupportLookup,
        ForgeQueryIntentAdmissionSlopeLane::CoveredEntrypointInventory,
        ForgeQueryIntentAdmissionSlopeLane::ExecutionProvenanceAssembly,
        ForgeQueryIntentAdmissionSlopeLane::LegacyDelegationParity,
        ForgeQueryIntentAdmissionSlopeLane::DecisionCertificationCoverage,
    ] {
        let rows = width_runs
            .iter()
            .filter(|row| row.lane() == lane)
            .collect::<Vec<_>>();
        assert_eq!(rows.len(), 3);
        assert_eq!(
            rows.iter().map(|row| row.scale()).collect::<Vec<_>>(),
            vec![
                ForgeQueryIntentAdmissionWidthRunScale::Small,
                ForgeQueryIntentAdmissionWidthRunScale::Medium,
                ForgeQueryIntentAdmissionWidthRunScale::Large,
            ]
        );
        assert!(rows[0].width() > 0);
        assert_eq!(rows[1].width(), rows[0].width() * 2);
        assert_eq!(rows[2].width(), rows[0].width() * 3);
    }

    let classification_small = width_runs
        .iter()
        .find(|row| {
            row.lane() == ForgeQueryIntentAdmissionSlopeLane::AdmissionClassification
                && row.scale() == ForgeQueryIntentAdmissionWidthRunScale::Small
        })
        .expect("small admission-classification run should exist");
    let trace_small = width_runs
        .iter()
        .find(|row| {
            row.lane() == ForgeQueryIntentAdmissionSlopeLane::DecisionTraceAssembly
                && row.scale() == ForgeQueryIntentAdmissionWidthRunScale::Small
        })
        .expect("small decision-trace run should exist");
    let provenance_small = width_runs
        .iter()
        .find(|row| {
            row.lane() == ForgeQueryIntentAdmissionSlopeLane::ExecutionProvenanceAssembly
                && row.scale() == ForgeQueryIntentAdmissionWidthRunScale::Small
        })
        .expect("small execution-provenance run should exist");

    assert_eq!(
        classification_small.width(),
        forge_query_intent_admission_family_inventory().rows().len()
    );
    assert!(trace_small.width() >= bundle.counter_snapshot().decision_trace_width());
    assert!(trace_small.width() > classification_small.width());
    assert_eq!(
        provenance_small.width(),
        bundle.counter_snapshot().execution_provenance_width()
    );
}
