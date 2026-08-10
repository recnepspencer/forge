use serde::Serialize;

use super::super::{AdaptivePlacementDebtMarker, PlacementExecutionOrigin};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum TierMoveRejection {
    UnsupportedPolicy { marker: AdaptivePlacementDebtMarker },
    IllegalExecutionOrigin { origin: PlacementExecutionOrigin },
    RawLocatorBoundaryViolation { locator: String },
    WitnessConstructionRequired { witness_type: &'static str },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkingSetDebtSummary {
    debt_marker: AdaptivePlacementDebtMarker,
    reason: String,
}

impl WorkingSetDebtSummary {
    pub(crate) fn new(debt_marker: AdaptivePlacementDebtMarker, reason: impl Into<String>) -> Self {
        Self {
            debt_marker,
            reason: reason.into(),
        }
    }

    pub fn debt_marker(&self) -> AdaptivePlacementDebtMarker {
        self.debt_marker
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}
