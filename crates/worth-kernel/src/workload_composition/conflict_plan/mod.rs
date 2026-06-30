mod spatial;
mod spatial_lowering;
mod topology;
mod topology_lowering;

pub use spatial::{
    lower_selected_spatial_conflict_plan, SelectedSpatialConflictFamilyRow,
    SelectedSpatialConflictPlan, SpatialConflictPlanCounters, SpatialConflictPlanDenial,
    SpatialConflictPlanDenialKind,
};
pub use topology::{
    lower_selected_topology_conflict_plan, SelectedTopologyConflictFamilyRow,
    SelectedTopologyConflictPlan, TopologyConflictPlanCounters, TopologyConflictPlanDenial,
    TopologyConflictPlanDenialKind,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConflictPlanExecutionAdmission {
    Admitted,
    Denied,
}

impl ConflictPlanExecutionAdmission {
    pub const fn from_denial(has_denial: bool) -> Self {
        if has_denial {
            Self::Denied
        } else {
            Self::Admitted
        }
    }

    pub const fn is_denied(self) -> bool {
        matches!(self, Self::Denied)
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::Denied => "denied",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConflictPlanDownstreamProofCategory {
    QueryBoundaryEnvelope,
    ProjectionConsumption,
}

impl ConflictPlanDownstreamProofCategory {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QueryBoundaryEnvelope => "query-boundary-envelope",
            Self::ProjectionConsumption => "projection-consumption",
        }
    }
}

#[cfg(test)]
mod test_support;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_order_independence;
