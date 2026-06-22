use crate::planar_contracts::clean_fail_boundary::PlanarDirtyInputKind;

use super::case::DirtyPlanarCleanFailCase;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DirtyPlanarCleanFailRecoveryPosture {
    ExplainsWithoutRepair,
    AttemptsTruthUpgrade,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DirtyPlanarCleanFailError {
    MissingTopologyCleanFail,
    TopologyAllowedSpatialBinding,
    MismatchedDirtyKind {
        topology: DirtyPlanarCleanFailCase,
        boundary: DirtyPlanarCleanFailCase,
    },
    CleanFailBoundaryDidNotConsumeTopologyReceipt,
    MissingCleanFailBoundary,
    CleanFailDidNotRepresentDirtyInput,
    CleanFailAttemptedRepair,
    CleanFailAttemptedBoundedConversion,
    CleanFailChangedTruth,
    MissingRecoveryPosture,
    RecoveryAttemptedTruthUpgrade,
    MissingTransformPosture,
    MissingUserResponse,
    UserResponseDidNotExplainDirtyNoOptions,
    UserResponseDidNotConsumeCleanFailBoundary,
    StableTopologyIdentityHidDirtyGeometry {
        dirty_kind: PlanarDirtyInputKind,
    },
}

impl DirtyPlanarCleanFailError {
    pub fn human_reason(&self) -> String {
        match self {
            Self::MissingTopologyCleanFail => {
                "dirty planar clean-fail requires a topology clean-fail receipt".to_string()
            }
            Self::TopologyAllowedSpatialBinding => {
                "dirty topology must stop before spatial binding can consume it".to_string()
            }
            Self::MismatchedDirtyKind { topology, boundary } => format!(
                "dirty topology is {} but clean-fail boundary reported {}",
                topology.human_name(),
                boundary.human_name()
            ),
            Self::CleanFailBoundaryDidNotConsumeTopologyReceipt => {
                "clean-fail boundary must consume the topology clean-fail receipt identity"
                    .to_string()
            }
            Self::MissingCleanFailBoundary => {
                "dirty planar clean-fail requires a clean-fail boundary receipt".to_string()
            }
            Self::CleanFailDidNotRepresentDirtyInput => {
                "clean-fail boundary must classify the source as dirty input".to_string()
            }
            Self::CleanFailAttemptedRepair => {
                "dirty planar input has no options because repair is not available in M6.5"
                    .to_string()
            }
            Self::CleanFailAttemptedBoundedConversion => {
                "dirty planar input cannot be converted to bounded clean geometry".to_string()
            }
            Self::CleanFailChangedTruth => {
                "dirty clean-fail evidence must not change planar truth".to_string()
            }
            Self::MissingRecoveryPosture => {
                "dirty planar clean-fail requires recovery posture evidence".to_string()
            }
            Self::RecoveryAttemptedTruthUpgrade => {
                "recovery may explain dirty input, but it cannot make dirty truth admitted"
                    .to_string()
            }
            Self::MissingTransformPosture => {
                "dirty planar clean-fail requires movement and rotation posture evidence"
                    .to_string()
            }
            Self::MissingUserResponse => {
                "dirty planar clean-fail requires a user response receipt".to_string()
            }
            Self::UserResponseDidNotExplainDirtyNoOptions => {
                "dirty planar clean-fail response must explain that no automatic option exists for dirty input".to_string()
            }
            Self::UserResponseDidNotConsumeCleanFailBoundary => {
                "dirty planar clean-fail response must consume the clean-fail boundary receipt"
                    .to_string()
            }
            Self::StableTopologyIdentityHidDirtyGeometry { dirty_kind } => format!(
                "stable topology identity cannot hide dirty geometry: {} remains dirty",
                dirty_kind.as_str().replace('-', " ")
            ),
        }
    }
}
