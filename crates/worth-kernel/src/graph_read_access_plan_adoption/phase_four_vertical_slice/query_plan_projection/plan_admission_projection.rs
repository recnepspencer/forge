#[cfg(test)]
use super::super::execution_binding::WorthGraphReadAccessExecutedVerticalSlice;
use super::super::slice_selection::WorthGraphReadAccessSelectedVerticalSlice;
#[cfg(test)]
use super::admitted_plan_projection::query_plan_admitted_projection;
use super::missing_read_family_projection::missing_query_read_family_projection;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessSlicePlanProjectionStatus {
    QueryPlanAdmitted,
    MissingQueryReadFamilyArtifactForExecution,
}

impl WorthGraphReadAccessSlicePlanProjectionStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QueryPlanAdmitted => "query_plan_admitted",
            Self::MissingQueryReadFamilyArtifactForExecution => {
                "missing_query_read_family_artifact_for_execution"
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessSlicePlanProjection {
    pub(crate) selected_slice_digest: String,
    pub(crate) status: WorthGraphReadAccessSlicePlanProjectionStatus,
    pub(crate) query_family_name: Option<String>,
    pub(crate) query_family_digest_seed: String,
    pub(crate) query_posture: String,
    pub(crate) executed_read_family_digest: Option<String>,
    pub(crate) query_requirement_set_digest: Option<String>,
    pub(crate) admitted_plan_digest: Option<String>,
    pub(crate) query_admission_digest: Option<String>,
    pub(crate) execution_strategy: Option<String>,
    pub(crate) required_worth_artifact: Option<String>,
    pub(crate) blocker: Option<String>,
    pub(crate) projection_digest: String,
}

pub(crate) fn project_query_plan_for_selected_slice(
    selected_slice: &WorthGraphReadAccessSelectedVerticalSlice,
) -> WorthGraphReadAccessSlicePlanProjection {
    missing_query_read_family_projection(selected_slice)
}

#[cfg(test)]
pub(crate) fn project_query_plan_for_executed_slice(
    selected_slice: &WorthGraphReadAccessSelectedVerticalSlice,
    executed_slice: &WorthGraphReadAccessExecutedVerticalSlice,
) -> WorthGraphReadAccessSlicePlanProjection {
    query_plan_admitted_projection(selected_slice, executed_slice)
}

impl WorthGraphReadAccessSlicePlanProjection {
    pub fn selected_slice_digest(&self) -> &str {
        &self.selected_slice_digest
    }

    pub const fn status(&self) -> WorthGraphReadAccessSlicePlanProjectionStatus {
        self.status
    }

    pub fn query_family_name(&self) -> Option<&str> {
        self.query_family_name.as_deref()
    }

    pub fn query_family_digest_seed(&self) -> &str {
        &self.query_family_digest_seed
    }

    pub fn query_posture(&self) -> &str {
        &self.query_posture
    }

    pub fn executed_read_family_digest(&self) -> Option<&str> {
        self.executed_read_family_digest.as_deref()
    }

    pub fn query_requirement_set_digest(&self) -> Option<&str> {
        self.query_requirement_set_digest.as_deref()
    }

    pub fn admitted_plan_digest(&self) -> Option<&str> {
        self.admitted_plan_digest.as_deref()
    }

    pub fn query_admission_digest(&self) -> Option<&str> {
        self.query_admission_digest.as_deref()
    }

    pub fn execution_strategy(&self) -> Option<&str> {
        self.execution_strategy.as_deref()
    }

    pub fn required_worth_artifact(&self) -> Option<&str> {
        self.required_worth_artifact.as_deref()
    }

    pub fn blocker(&self) -> Option<&str> {
        self.blocker.as_deref()
    }

    pub fn projection_digest(&self) -> &str {
        &self.projection_digest
    }

    pub const fn claims_query_plan_admission(&self) -> bool {
        matches!(
            self.status,
            WorthGraphReadAccessSlicePlanProjectionStatus::QueryPlanAdmitted
        )
    }
}
