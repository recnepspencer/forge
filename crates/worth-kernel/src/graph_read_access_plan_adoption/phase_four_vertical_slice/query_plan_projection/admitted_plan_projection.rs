#[cfg(test)]
use super::super::execution_binding::WorthGraphReadAccessExecutedVerticalSlice;
#[cfg(test)]
use super::super::slice_selection::WorthGraphReadAccessSelectedVerticalSlice;
#[cfg(test)]
use super::super::stable_digest;
#[cfg(test)]
use super::{
    WorthGraphReadAccessSlicePlanProjection, WorthGraphReadAccessSlicePlanProjectionStatus,
};

#[cfg(test)]
pub(crate) fn query_plan_admitted_projection(
    selected_slice: &WorthGraphReadAccessSelectedVerticalSlice,
    executed_slice: &WorthGraphReadAccessExecutedVerticalSlice,
) -> WorthGraphReadAccessSlicePlanProjection {
    let status = WorthGraphReadAccessSlicePlanProjectionStatus::QueryPlanAdmitted;
    let execution_strategy = "construction_query_access_plan";
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
        format!(
            "executed_read_family:{}",
            executed_slice.executed_read_family_digest()
        ),
        format!(
            "query_requirement_set:{}",
            executed_slice.query_requirement_set_digest()
        ),
        format!("admitted_plan:{}", executed_slice.admitted_plan_digest()),
        format!(
            "query_admission:{}",
            executed_slice.query_admission_digest()
        ),
        format!("execution_strategy:{execution_strategy}"),
    ]);
    WorthGraphReadAccessSlicePlanProjection {
        selected_slice_digest: selected_slice.slice_digest().to_string(),
        status,
        query_family_name: selected_slice.query_family_name().map(str::to_string),
        query_family_digest_seed: selected_slice.query_family_digest_seed().to_string(),
        query_posture: selected_slice.query_posture().to_string(),
        executed_read_family_digest: Some(executed_slice.executed_read_family_digest().to_string()),
        query_requirement_set_digest: Some(
            executed_slice.query_requirement_set_digest().to_string(),
        ),
        admitted_plan_digest: Some(executed_slice.admitted_plan_digest().to_string()),
        query_admission_digest: Some(executed_slice.query_admission_digest().to_string()),
        execution_strategy: Some(execution_strategy.to_string()),
        required_worth_artifact: None,
        blocker: None,
        projection_digest,
    }
}
