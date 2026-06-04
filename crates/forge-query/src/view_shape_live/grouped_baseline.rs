use crate::basis::ResolvedSnapshotBasis;
use crate::identity::BasisDigest;
use crate::view_shape::{ViewShapePlanArtifact, ViewShapePlanDigest};

use super::counters::ViewShapeLiveCounters;
use super::error::{ViewShapeLiveError, ViewShapeLiveFailureClass};
#[cfg(test)]
use super::grouped_execution::GroupedExecutionSurfaceArtifact;
use super::grouped_state::{desired_state_from_members, GroupedDesiredStateArtifact};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthoritativeGroupedBaselineArtifact {
    plan_digest: ViewShapePlanDigest,
    basis_digest: BasisDigest,
    desired_state: GroupedDesiredStateArtifact,
}

impl AuthoritativeGroupedBaselineArtifact {
    pub fn plan_digest(&self) -> &ViewShapePlanDigest {
        &self.plan_digest
    }

    pub fn basis_digest(&self) -> &BasisDigest {
        &self.basis_digest
    }

    pub fn desired_state(&self) -> &GroupedDesiredStateArtifact {
        &self.desired_state
    }
}

#[cfg(test)]
pub fn materialize_authoritative_grouped_baseline(
    plan: &ViewShapePlanArtifact,
    basis: ResolvedSnapshotBasis,
    grouped_execution: &GroupedExecutionSurfaceArtifact,
) -> Result<AuthoritativeGroupedBaselineArtifact, ViewShapeLiveError> {
    if plan.family() != crate::view_shape::ViewShapeFamily::KanbanGrouped {
        return Err(ViewShapeLiveError::new(
            ViewShapeLiveFailureClass::GroupedBaselineMismatch,
            "authoritative grouped baseline may only be materialized for kanban grouped plans",
            ViewShapeLiveCounters::default(),
        ));
    }
    if basis.identity().schema_basis() != plan.validated().query().schema_basis()
        || basis.identity().schema_basis() != plan.validated().result_shape().schema_basis()
    {
        return Err(ViewShapeLiveError::new(
            ViewShapeLiveFailureClass::BasisInvariantRejected,
            format!(
                "grouped baseline basis schema '{}' does not match validated query/result-shape schema '{}'",
                basis.identity().schema_basis().as_str(),
                plan.validated().query().schema_basis().as_str()
            ),
            ViewShapeLiveCounters::default(),
        ));
    }

    let grouped_planning = plan.grouped_planning_artifact().ok_or_else(|| {
        ViewShapeLiveError::new(
            ViewShapeLiveFailureClass::GroupedBaselineMismatch,
            "grouped baseline requires planner-issued grouped planning artifact",
            ViewShapeLiveCounters::default(),
        )
    })?;
    if grouped_execution.plan_digest().as_str() != plan.view_plan_digest().as_str() {
        return Err(ViewShapeLiveError::new(
            ViewShapeLiveFailureClass::GroupedBaselineMismatch,
            format!(
                "grouped execution surface plan digest '{}' does not match grouped baseline plan digest '{}'",
                grouped_execution.plan_digest().as_str(),
                plan.view_plan_digest().as_str()
            ),
            ViewShapeLiveCounters::default(),
        ));
    }
    if grouped_execution.basis_digest() != basis.proof().digest() {
        return Err(ViewShapeLiveError::new(
            ViewShapeLiveFailureClass::GroupedBaselineMismatch,
            format!(
                "grouped execution surface basis digest '{}' does not match grouped baseline basis digest '{}'",
                grouped_execution.basis_digest().as_str(),
                basis.proof().digest().as_str()
            ),
            ViewShapeLiveCounters::default(),
        ));
    }
    if grouped_execution.grouped_planning() != grouped_planning {
        return Err(ViewShapeLiveError::new(
            ViewShapeLiveFailureClass::GroupedBaselineMismatch,
            "grouped execution surface planning artifact does not match grouped baseline planning artifact",
            ViewShapeLiveCounters::default(),
        ));
    }
    let native_grouping_aspect_key = grouped_planning.native_grouping_aspect_key();
    if grouped_execution.member_rows().is_empty() {
        return Err(ViewShapeLiveError::new(
            ViewShapeLiveFailureClass::GroupedBaselineMismatch,
            "grouped execution surface must expose at least one authoritative member row",
            ViewShapeLiveCounters::default(),
        ));
    }
    for member_row in grouped_execution.member_rows() {
        if member_row.lane().native_grouping_aspect_key() != native_grouping_aspect_key {
            return Err(ViewShapeLiveError::new(
                ViewShapeLiveFailureClass::GroupedBaselineMismatch,
                format!(
                    "grouped execution surface grouping aspect '{}' does not match grouped baseline aspect '{}'",
                    member_row.lane().grouping_aspect(),
                    native_grouping_aspect_key.as_str()
                ),
                ViewShapeLiveCounters::default(),
            ));
        }
    }

    Ok(AuthoritativeGroupedBaselineArtifact {
        plan_digest: plan.view_plan_digest().clone(),
        basis_digest: basis.proof().digest().clone(),
        desired_state: desired_state_from_members(
            native_grouping_aspect_key.clone(),
            grouped_execution
                .member_rows()
                .iter()
                .map(|member_row| {
                    (
                        member_row.member_key().to_string(),
                        member_row.lane().lane_key().to_string(),
                    )
                })
                .collect(),
        ),
    })
}

pub fn materialize_authoritative_grouped_baseline_from_members(
    plan: &ViewShapePlanArtifact,
    basis: ResolvedSnapshotBasis,
    members: impl IntoIterator<Item = (String, String)>,
) -> Result<AuthoritativeGroupedBaselineArtifact, ViewShapeLiveError> {
    if plan.family() != crate::view_shape::ViewShapeFamily::KanbanGrouped {
        return Err(ViewShapeLiveError::new(
            ViewShapeLiveFailureClass::GroupedBaselineMismatch,
            "authoritative grouped baseline may only be materialized for kanban grouped plans",
            ViewShapeLiveCounters::default(),
        ));
    }
    if basis.identity().schema_basis() != plan.validated().query().schema_basis()
        || basis.identity().schema_basis() != plan.validated().result_shape().schema_basis()
    {
        return Err(ViewShapeLiveError::new(
            ViewShapeLiveFailureClass::BasisInvariantRejected,
            format!(
                "grouped baseline basis schema '{}' does not match validated query/result-shape schema '{}'",
                basis.identity().schema_basis().as_str(),
                plan.validated().query().schema_basis().as_str()
            ),
            ViewShapeLiveCounters::default(),
        ));
    }
    let grouped_planning = plan.grouped_planning_artifact().ok_or_else(|| {
        ViewShapeLiveError::new(
            ViewShapeLiveFailureClass::GroupedBaselineMismatch,
            "grouped baseline requires planner-issued grouped planning artifact",
            ViewShapeLiveCounters::default(),
        )
    })?;
    let members = members.into_iter().collect::<Vec<_>>();
    if members.is_empty() {
        return Err(ViewShapeLiveError::new(
            ViewShapeLiveFailureClass::GroupedBaselineMismatch,
            "grouped baseline requires at least one authoritative member row",
            ViewShapeLiveCounters::default(),
        ));
    }

    Ok(AuthoritativeGroupedBaselineArtifact {
        plan_digest: plan.view_plan_digest().clone(),
        basis_digest: basis.proof().digest().clone(),
        desired_state: desired_state_from_members(
            grouped_planning.native_grouping_aspect_key().clone(),
            members,
        ),
    })
}
