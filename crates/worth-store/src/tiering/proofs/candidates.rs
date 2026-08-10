use serde::Serialize;

use super::classification::TierResidenceClass;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TierPromotionCandidate {
    artifact_key: String,
    target_residence: TierResidenceClass,
}

impl TierPromotionCandidate {
    pub(crate) fn new(
        artifact_key: impl Into<String>,
        target_residence: TierResidenceClass,
    ) -> Self {
        Self {
            artifact_key: artifact_key.into(),
            target_residence,
        }
    }

    pub fn artifact_key(&self) -> &str {
        &self.artifact_key
    }

    pub fn target_residence(&self) -> TierResidenceClass {
        self.target_residence
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TierCoolingCandidate {
    artifact_key: String,
    target_residence: TierResidenceClass,
}

impl TierCoolingCandidate {
    pub(crate) fn new(
        artifact_key: impl Into<String>,
        target_residence: TierResidenceClass,
    ) -> Self {
        Self {
            artifact_key: artifact_key.into(),
            target_residence,
        }
    }

    pub fn artifact_key(&self) -> &str {
        &self.artifact_key
    }

    pub fn target_residence(&self) -> TierResidenceClass {
        self.target_residence
    }
}
