use worth_spatial::facade::planar_boolean_common_plane::PlanarBooleanCommonPlaneAgreementDenial;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlanarBooleanCommonPlanePlaneAgreementError {
    SpatialPlaneAgreementDenied {
        request_identity: String,
        operand_pair_identity: String,
        scope_admission_identity: String,
        denial: PlanarBooleanCommonPlaneAgreementDenial,
    },
}

impl PlanarBooleanCommonPlanePlaneAgreementError {
    pub fn human_reason(&self) -> &str {
        match self {
            Self::SpatialPlaneAgreementDenied { denial, .. } => denial.human_reason(),
        }
    }

    pub fn request_identity(&self) -> &str {
        match self {
            Self::SpatialPlaneAgreementDenied {
                request_identity, ..
            } => request_identity,
        }
    }

    pub fn operand_pair_identity(&self) -> &str {
        match self {
            Self::SpatialPlaneAgreementDenied {
                operand_pair_identity,
                ..
            } => operand_pair_identity,
        }
    }

    pub fn scope_admission_identity(&self) -> &str {
        match self {
            Self::SpatialPlaneAgreementDenied {
                scope_admission_identity,
                ..
            } => scope_admission_identity,
        }
    }

    pub fn spatial_denial(&self) -> &PlanarBooleanCommonPlaneAgreementDenial {
        match self {
            Self::SpatialPlaneAgreementDenied { denial, .. } => denial,
        }
    }
}
