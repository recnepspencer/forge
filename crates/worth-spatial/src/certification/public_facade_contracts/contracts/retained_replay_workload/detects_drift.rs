use super::contract_subject::{projection_consumed_receipt, retained_replay_parts};
use crate::public_api_planar_projection_consumption::contract_subject::projection_consumed_planar_parts;
use worth_spatial::facade::retained_replay_workload::{
    RetainedWorkload, UnsupportedReplayReasonCode,
};

#[test]
fn retained_capture_detects_live_retained_projection_drift_before_replay() {
    let retained_world = retained_replay_parts("retained-replay-drift-retained");
    let other_retained = projection_consumed_planar_parts("retained-replay-drift-projection");
    let drifted_projection =
        projection_consumed_receipt("retained-replay-drift-projection", &other_retained);

    let denial =
        RetainedWorkload::from_retained_planar_facts(retained_world.retained_parts.retained)
            .declared("reject projection-consumed facts from another retained basis")
            .with_projection_consumed_facts(drifted_projection)
            .capture()
            .expect_err("retained/projection drift must deny retained capture");

    assert_eq!(
        denial.reason_code(),
        UnsupportedReplayReasonCode::RetainedProjectionDrift
    );
    assert_eq!(
        denial.human_reason(),
        "Retained replay workload requires projection-consumed facts to reference the same retained planar fact digest as the retained artifact."
    );
    assert!(!denial.can_enter_diagnostics_workload());
    assert!(!denial.can_enter_operator_execution());
}

#[test]
fn retained_workload_capture_detects_projection_drift_before_replay() {
    let retained_world = retained_replay_parts("retained-capture-drift-retained");
    let other_retained = projection_consumed_planar_parts("retained-capture-drift-projection");
    let drifted_projection =
        projection_consumed_receipt("retained-capture-drift-projection", &other_retained);

    let denial =
        RetainedWorkload::from_retained_planar_facts(retained_world.retained_parts.retained)
            .declared("capture must reject projection drift")
            .with_projection_consumed_facts(drifted_projection)
            .capture()
            .expect_err("retained capture must reject projection drift before replay");

    assert_eq!(
        denial.reason_code(),
        UnsupportedReplayReasonCode::RetainedProjectionDrift
    );
    assert_eq!(
        denial.human_reason(),
        "Retained replay workload requires projection-consumed facts to reference the same retained planar fact digest as the retained artifact."
    );
}
