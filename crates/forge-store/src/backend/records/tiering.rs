use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TierResidencyRecord {
    pub artifact_key: String,
    pub artifact_family: crate::PlacementArtifactFamily,
    pub canonical_residence: crate::TierResidenceClass,
    pub canonical_replica_locator: String,
    pub verification_label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TierTransferRecord {
    pub artifact_key: String,
    pub artifact_family: crate::PlacementArtifactFamily,
    pub source_residence: crate::TierResidenceClass,
    pub target_residence: crate::TierResidenceClass,
    pub execution_origin: crate::PlacementExecutionOrigin,
    pub source_replica_locator: String,
    pub transferred_replica_locator: Option<String>,
    pub verification_label: Option<String>,
    pub cutover_completed: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TierRecallCompletionState {
    InFlight,
    Completed,
}

impl TierRecallCompletionState {
    pub fn label(self) -> &'static str {
        match self {
            Self::InFlight => "in_flight",
            Self::Completed => "completed",
        }
    }

    pub fn from_label(label: &str) -> Option<Self> {
        match label {
            "in_flight" => Some(Self::InFlight),
            "completed" => Some(Self::Completed),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TierRecallRecord {
    pub coalescing_key: String,
    pub artifact_family: crate::PlacementArtifactFamily,
    pub scope_class: crate::PlacementObservationScopeClass,
    pub scope_key: String,
    pub execution_origin: crate::PlacementExecutionOrigin,
    pub artifact_key: String,
    pub recall_cost_class: crate::RecallCostClass,
    pub amplification_budget: crate::RecallAmplificationBudget,
    pub completion_state: TierRecallCompletionState,
}
