use super::super::slice_selection::WorthGraphReadAccessSelectedVerticalSlice;
use super::super::stable_digest;
use super::plan_gap_projection::missing_query_read_family_artifact_blocker;
use super::{
    WorthGraphReadAccessSlicePlanProjection, WorthGraphReadAccessSlicePlanProjectionStatus,
};

pub(crate) fn missing_query_read_family_projection(
    selected_slice: &WorthGraphReadAccessSelectedVerticalSlice,
) -> WorthGraphReadAccessSlicePlanProjection {
    let status =
        WorthGraphReadAccessSlicePlanProjectionStatus::MissingQueryReadFamilyArtifactForExecution;
    let blocker = missing_query_read_family_artifact_blocker();
    let projection_digest = stable_digest(&[
        "worth_graph_read_access_slice_plan_projection_v1".to_string(),
        format!("slice:{}", selected_slice.slice_digest()),
        format!("status:{}", status.as_str()),
        format!(
            "query_family:{}",
            selected_slice.query_family_name().unwrap_or("none")
        ),
        format!(
            "query_family_seed:{}",
            selected_slice.query_family_digest_seed()
        ),
        format!("query_posture:{}", selected_slice.query_posture()),
        "required_worth_artifact:ForgeQueryReadFamily".to_string(),
        format!("blocker:{blocker}"),
    ]);
    WorthGraphReadAccessSlicePlanProjection {
        selected_slice_digest: selected_slice.slice_digest().to_string(),
        status,
        query_family_name: selected_slice.query_family_name().map(str::to_string),
        query_family_digest_seed: selected_slice.query_family_digest_seed().to_string(),
        query_posture: selected_slice.query_posture().to_string(),
        executed_read_family_digest: None,
        query_requirement_set_digest: None,
        admitted_plan_digest: None,
        query_admission_digest: None,
        execution_strategy: None,
        required_worth_artifact: Some("ForgeQueryReadFamily".to_string()),
        blocker: Some(blocker.to_string()),
        projection_digest,
    }
}
