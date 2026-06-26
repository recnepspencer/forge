use super::super::batch_admission::WorthGraphReadAccessGroupedAdmissionMeasurementStatus;
use super::production_phase_five_closeout;

#[test]
fn spatial_and_dense_reads_do_not_scalarize_batch_admission() {
    let closeout = production_phase_five_closeout();

    assert_eq!(
        closeout
            .grouped_admission_report()
            .scalarized_caller_loop_count(),
        0
    );
    assert_eq!(closeout.counters().scalarized_caller_loop_count(), 0);
    assert!(
        !closeout.grouped_admission_report().rows().is_empty(),
        "Phase 5 must expose grouped-admission rows even before execution counters exist"
    );
    assert!(closeout
        .grouped_admission_report()
        .rows()
        .iter()
        .all(|row| {
            row.scalarized_caller_loop_count() == 0
                && row.caller_work_measurement_status()
                    == WorthGraphReadAccessGroupedAdmissionMeasurementStatus::NoGraphReadExecutionClaimed
        }));
}
