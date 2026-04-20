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
