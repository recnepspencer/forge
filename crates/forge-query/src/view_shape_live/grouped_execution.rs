use crate::basis::ResolvedSnapshotBasis;
use crate::identity::{BasisDigest, hash_parts};
use crate::view_shape::{GroupedViewPlanningArtifact, ViewShapePlanArtifact, ViewShapePlanDigest};
use forge_runtime_bridge::facade::BridgeGroupedTruthViewArtifact;

use super::counters::ViewShapeLiveCounters;
use super::error::{ViewShapeLiveError, ViewShapeLiveFailureClass};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GroupedExecutionLaneValue {
    grouping_aspect: String,
    lane_key: String,
}

impl GroupedExecutionLaneValue {
    pub fn grouping_aspect(&self) -> &str {
        &self.grouping_aspect
    }

    pub fn lane_key(&self) -> &str {
        &self.lane_key
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GroupedExecutionMemberRow {
    member_key: String,
    lane: GroupedExecutionLaneValue,
}

impl GroupedExecutionMemberRow {
    pub fn member_key(&self) -> &str {
        &self.member_key
    }

    pub fn lane(&self) -> &GroupedExecutionLaneValue {
        &self.lane
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GroupedExecutionSurfaceArtifact {
    digest: String,
    plan_digest: ViewShapePlanDigest,
    basis_digest: BasisDigest,
    truth_view_digest: String,
    grouped_planning: GroupedViewPlanningArtifact,
    member_rows: Vec<GroupedExecutionMemberRow>,
}

impl GroupedExecutionSurfaceArtifact {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn plan_digest(&self) -> &ViewShapePlanDigest {
        &self.plan_digest
    }

    pub fn basis_digest(&self) -> &BasisDigest {
        &self.basis_digest
    }

    pub fn truth_view_digest(&self) -> &str {
        &self.truth_view_digest
    }

    pub fn grouped_planning(&self) -> &GroupedViewPlanningArtifact {
        &self.grouped_planning
    }

    pub fn member_rows(&self) -> &[GroupedExecutionMemberRow] {
        &self.member_rows
    }
}

pub fn materialize_grouped_execution_surface_from_truth_view(
    plan: &ViewShapePlanArtifact,
    basis: ResolvedSnapshotBasis,
    truth_view: &BridgeGroupedTruthViewArtifact,
) -> Result<GroupedExecutionSurfaceArtifact, ViewShapeLiveError> {
    if plan.family() != crate::view_shape::ViewShapeFamily::KanbanGrouped {
        return Err(ViewShapeLiveError::new(
            ViewShapeLiveFailureClass::GroupedBaselineMismatch,
            "grouped execution surface may only be materialized for kanban grouped plans",
            ViewShapeLiveCounters::default(),
        ));
    }
    if basis.identity().schema_basis() != plan.validated().query().schema_basis()
        || basis.identity().schema_basis() != plan.validated().result_shape().schema_basis()
    {
        return Err(ViewShapeLiveError::new(
            ViewShapeLiveFailureClass::BasisInvariantRejected,
            format!(
                "grouped execution basis schema '{}' does not match validated query/result-shape schema '{}'",
                basis.identity().schema_basis().as_str(),
                plan.validated().query().schema_basis().as_str()
            ),
            ViewShapeLiveCounters::default(),
        ));
    }

    let grouped_planning = plan.grouped_planning_artifact().cloned().ok_or_else(|| {
        ViewShapeLiveError::new(
            ViewShapeLiveFailureClass::GroupedBaselineMismatch,
            "grouped execution surface requires planner-issued grouped planning artifact",
            ViewShapeLiveCounters::default(),
        )
    })?;
    if truth_view.contract().grouping_aspect() != grouped_planning.grouping_aspect() {
        return Err(ViewShapeLiveError::new(
            ViewShapeLiveFailureClass::GroupedBaselineMismatch,
            format!(
                "grouped truth-view aspect '{}' does not match planned grouping aspect '{}'",
                truth_view.contract().grouping_aspect(),
                grouped_planning.grouping_aspect()
            ),
            ViewShapeLiveCounters::default(),
        ));
    }
    if truth_view.basis_snapshot_identity().as_str() != basis.identity().snapshot_token() {
        return Err(ViewShapeLiveError::new(
            ViewShapeLiveFailureClass::GroupedBaselineMismatch,
            format!(
                "grouped truth-view snapshot '{}' does not match grouped execution basis snapshot '{}'",
                truth_view.basis_snapshot_identity().as_str(),
                basis.identity().snapshot_token()
            ),
            ViewShapeLiveCounters::default(),
        ));
    }
    if truth_view.contract().identity_binding().field_key()
        != grouped_planning.identity_binding().field_key()
    {
        return Err(ViewShapeLiveError::new(
            ViewShapeLiveFailureClass::GroupedBaselineMismatch,
            format!(
                "grouped truth-view identity binding '{}' does not match planned identity binding '{}'",
                truth_view.contract().identity_binding().field_key(),
                grouped_planning.identity_binding().field_key()
            ),
            ViewShapeLiveCounters::default(),
        ));
    }
    if truth_view.contract().grouping_binding().field_key()
        != grouped_planning.grouping_binding().field_key()
    {
        return Err(ViewShapeLiveError::new(
            ViewShapeLiveFailureClass::GroupedBaselineMismatch,
            format!(
                "grouped truth-view grouping binding '{}' does not match planned grouping binding '{}'",
                truth_view.contract().grouping_binding().field_key(),
                grouped_planning.grouping_binding().field_key()
            ),
            ViewShapeLiveCounters::default(),
        ));
    }

    let member_rows = truth_view
        .members()
        .iter()
        .map(|member| GroupedExecutionMemberRow {
            member_key: canonical_value_text(member.identity_value()),
            lane: GroupedExecutionLaneValue {
                grouping_aspect: grouped_planning.grouping_aspect().to_string(),
                lane_key: canonical_value_text(member.lane().value()),
            },
        })
        .collect::<Vec<_>>();
    if member_rows.is_empty() {
        return Err(ViewShapeLiveError::new(
            ViewShapeLiveFailureClass::GroupedBaselineMismatch,
            "grouped truth-view artifact produced no member rows",
            ViewShapeLiveCounters::default(),
        ));
    }

    let digest = hash_parts(&[
        format!("plan:{}", plan.view_plan_digest().as_str()),
        format!("basis:{}", basis.proof().digest().as_str()),
        format!("grouped_truth:{}", truth_view.digest().as_str()),
        format!("members:{}", member_rows.len()),
    ]);

    Ok(GroupedExecutionSurfaceArtifact {
        digest,
        plan_digest: plan.view_plan_digest().clone(),
        basis_digest: basis.proof().digest().clone(),
        truth_view_digest: truth_view.digest().as_str().to_string(),
        grouped_planning,
        member_rows,
    })
}

fn canonical_value_text(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(text) => text.clone(),
        other => serde_json::to_string(other).unwrap_or_else(|_| "<invalid-json>".to_string()),
    }
}
