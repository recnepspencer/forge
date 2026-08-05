use super::*;

pub(in crate::harness::milestone_eight_certification) fn bundle_from_view_execution(
    query_digest: String,
    plan_digest: String,
    result_shape_digest: String,
    delivery_digest: String,
    counters: Vec<String>,
    artifact_binding_matrix_digest: String,
    support_profile_digest: String,
) -> MilestoneEightCertificationBundle {
    bundle_from_view_execution_with_identity(
        query_digest,
        plan_digest,
        result_shape_digest,
        delivery_digest,
        counters,
        artifact_binding_matrix_digest,
        support_profile_digest,
        String::new(),
        String::new(),
        String::new(),
    )
}

pub(in crate::harness::milestone_eight_certification) fn bundle_from_view_execution_with_identity(
    query_digest: String,
    plan_digest: String,
    result_shape_digest: String,
    delivery_digest: String,
    counters: Vec<String>,
    artifact_binding_matrix_digest: String,
    support_profile_digest: String,
    identity_consumption_digest: String,
    inspector_identity_digest: String,
    inspector_identity_classification: String,
) -> MilestoneEightCertificationBundle {
    let counter_snapshot_digest = digest_parts(&counters);
    MilestoneEightCertificationBundle {
        query_digest,
        plan_digest,
        result_shape_digest,
        delivery_digest,
        counter_snapshot_digest,
        artifact_binding_matrix_digest,
        support_profile_digest,
        identity_consumption_digest,
        inspector_identity_digest,
        inspector_identity_classification,
    }
}
