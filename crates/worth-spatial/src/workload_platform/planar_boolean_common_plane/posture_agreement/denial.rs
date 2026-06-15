use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanCommonPlanePostureAgreementDenialKind {
    MissingDeclaration,
    MissingMovementRotationPostureWitness,
    DistinctMovementRotationPostures,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlanarBooleanCommonPlanePostureAgreementDenial {
    MissingDeclaration,
    MissingMovementRotationPostureWitness {
        side: PlanarBooleanCommonPlaneOperandSide,
        projected_workload_identity: String,
        transform_stage_identity: String,
    },
    DistinctMovementRotationPostures {
        left_projected_workload_identity: String,
        right_projected_workload_identity: String,
        left_transform_stage_identity: String,
        right_transform_stage_identity: String,
        left_posture_identity: String,
        right_posture_identity: String,
    },
}

impl PlanarBooleanCommonPlanePostureAgreementDenial {
    pub fn kind(&self) -> PlanarBooleanCommonPlanePostureAgreementDenialKind {
        match self {
            Self::MissingDeclaration => {
                PlanarBooleanCommonPlanePostureAgreementDenialKind::MissingDeclaration
            }
            Self::MissingMovementRotationPostureWitness { .. } => {
                PlanarBooleanCommonPlanePostureAgreementDenialKind::MissingMovementRotationPostureWitness
            }
            Self::DistinctMovementRotationPostures { .. } => {
                PlanarBooleanCommonPlanePostureAgreementDenialKind::DistinctMovementRotationPostures
            }
        }
    }

    pub fn human_reason(&self) -> &'static str {
        match self {
            Self::MissingDeclaration => {
                "Common-plane posture agreement requires a human-readable declaration."
            }
            Self::MissingMovementRotationPostureWitness { .. } => {
                "Common-plane posture agreement requires a movement and rotation posture witness on each operand before reduction can continue."
            }
            Self::DistinctMovementRotationPostures { .. } => {
                "Common-plane posture agreement denied the pair because the certified operand movement and rotation postures are distinct."
            }
        }
    }
}
