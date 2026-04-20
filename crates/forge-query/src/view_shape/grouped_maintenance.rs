use super::grouped_planning::GroupedViewPlanningArtifact;
use super::grouped_policy::GroupedDeltaAdmissionPolicy;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ViewShapeMaintenanceContract {
    Ungrouped,
    KanbanGrouped {
        grouped_planning: GroupedViewPlanningArtifact,
    },
}

impl ViewShapeMaintenanceContract {
    pub fn grouped_delta_policy(&self) -> Option<&GroupedDeltaAdmissionPolicy> {
        match self {
            Self::Ungrouped => None,
            Self::KanbanGrouped { grouped_planning } => Some(grouped_planning.grouped_delta_policy()),
        }
    }

    pub fn grouped_planning(&self) -> Option<&GroupedViewPlanningArtifact> {
        match self {
            Self::Ungrouped => None,
            Self::KanbanGrouped { grouped_planning } => Some(grouped_planning),
        }
    }
}
