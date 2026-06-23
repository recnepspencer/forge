use worth_ui::facade::{
    WorthUiActivatedProjectionRebindPlan, WorthUiHeaderMenuPlan,
    WorthUiPreservedProjectionRebindPlan, WorthUiProjectionRebindBatchReceipt,
    WorthUiProjectionRebindCounters, WorthUiProjectionRebindRowReceipt,
    WorthUiProjectionRebindStatus,
};

fn main() {
    let _preserved: WorthUiPreservedProjectionRebindPlan<WorthUiHeaderMenuPlan> =
        WorthUiPreservedProjectionRebindPlan {
            evidence: impossible(),
            admitted_projection: impossible(),
            status: WorthUiProjectionRebindStatus::EquivalentAfterActivation,
        };

    let _activated: WorthUiActivatedProjectionRebindPlan<WorthUiHeaderMenuPlan> =
        WorthUiActivatedProjectionRebindPlan {
            evidence: impossible(),
            admitted_projection: impossible(),
        };

    let _row = WorthUiProjectionRebindRowReceipt {
        projection_identity: impossible(),
        projection_family: impossible(),
        status: WorthUiProjectionRebindStatus::EquivalentAfterActivation,
        previous_frame_digest: 1,
        rebound_frame_digest: 1,
    };

    let _counters = WorthUiProjectionRebindCounters {
        inspected_projection_count: 1,
        dependency_intersection_count: 0,
        rebuild_attempt_count: 0,
        preserved_frame_count: 1,
        denied_frame_count: 0,
        rebuilt_frame_count: 0,
    };

    let _batch = WorthUiProjectionRebindBatchReceipt {
        runtime_instance: impossible(),
        change_evidence_digest: impossible(),
        counters: _counters,
        rows: vec![],
    };
}

fn impossible<T>() -> T {
    panic!("fixture must fail to compile before values are needed")
}
