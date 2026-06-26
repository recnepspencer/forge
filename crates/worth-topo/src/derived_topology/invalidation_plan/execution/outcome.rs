use serde::Serialize;

use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationPlannedDisposition;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DerivedInvalidationExecutionOutcome {
    IncrementalUpdated,
    BoundedRebuilt,
    Unaffected,
    Denied,
    ResidueCapped,
}

impl DerivedInvalidationExecutionOutcome {
    pub const fn from_planned_disposition(
        disposition: DerivedInvalidationPlannedDisposition,
    ) -> Self {
        match disposition {
            DerivedInvalidationPlannedDisposition::IncrementalUpdate => Self::IncrementalUpdated,
            DerivedInvalidationPlannedDisposition::BoundedRebuild => Self::BoundedRebuilt,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IncrementalUpdated => "incremental_updated",
            Self::BoundedRebuilt => "bounded_rebuilt",
            Self::Unaffected => "unaffected",
            Self::Denied => "denied",
            Self::ResidueCapped => "residue_capped",
        }
    }
}
