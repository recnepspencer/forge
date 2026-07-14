use serde::Serialize;

use crate::logic::transaction::runtime::state::SignalMergeCompatibilityPostureKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SignalMergeSupportReadinessPosture {
    CurrentBasis,
    BoundaryBridgedAuthorityRevalidationRequired,
}

impl SignalMergeSupportReadinessPosture {
    pub(crate) fn from_compatibility_posture(posture: SignalMergeCompatibilityPostureKind) -> Self {
        match posture {
            SignalMergeCompatibilityPostureKind::CurrentBasis => Self::CurrentBasis,
            SignalMergeCompatibilityPostureKind::BoundaryBridgedAuthorityRevalidationRequired => {
                Self::BoundaryBridgedAuthorityRevalidationRequired
            }
        }
    }
}
