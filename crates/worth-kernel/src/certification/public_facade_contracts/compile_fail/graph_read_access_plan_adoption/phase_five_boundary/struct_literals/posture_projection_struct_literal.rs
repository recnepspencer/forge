use worth_kernel::graph_read_access_plan_adoption::{
    WorthGraphReadAccessSpatialDensePostureOutcome,
    WorthGraphReadAccessSpatialDensePostureProjection, WorthGraphReadAccessUnresolvedSliceKind,
};

fn main() {
    let _ = WorthGraphReadAccessSpatialDensePostureProjection {
        slice_kind: WorthGraphReadAccessUnresolvedSliceKind::SpatialGraphRead,
        outcome: WorthGraphReadAccessSpatialDensePostureOutcome::RequiredQueryPosture,
        source_posture_row_digest: String::new(),
        source_requirement_record_digest: String::new(),
        read_family_identity_digest: None,
        requirement_row_digest: None,
        query_family_name: None,
        query_family_digest_seed: String::new(),
        query_posture: String::new(),
        denial_kind: None,
        query_plan_digest: None,
        query_receipt_digest: None,
        execution_counter_digest: None,
        blocker: None,
        removal_trigger: None,
        projection_digest: String::new(),
    };
}
