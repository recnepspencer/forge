use worth_spatial::facade::planar_boolean_common_plane::PlanarBooleanCommonPlanePostureAgreementDenial;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlanarBooleanCommonPlanePostureAgreementError {
    SpatialPostureAgreementDenied {
        request_identity: String,
        operand_pair_identity: String,
        scope_admission_identity: String,
        plane_agreement_identity: String,
        denial: PlanarBooleanCommonPlanePostureAgreementDenial,
    },
}

impl PlanarBooleanCommonPlanePostureAgreementError {
    pub fn human_reason(&self) -> &str {
        match self {
            Self::SpatialPostureAgreementDenied { denial, .. } => denial.human_reason(),
        }
    }

    pub fn request_identity(&self) -> &str {
        match self {
            Self::SpatialPostureAgreementDenied {
                request_identity, ..
            } => request_identity,
        }
    }

    pub fn operand_pair_identity(&self) -> &str {
        match self {
            Self::SpatialPostureAgreementDenied {
                operand_pair_identity,
                ..
            } => operand_pair_identity,
        }
    }

    pub fn scope_admission_identity(&self) -> &str {
        match self {
            Self::SpatialPostureAgreementDenied {
                scope_admission_identity,
                ..
            } => scope_admission_identity,
        }
    }

    pub fn plane_agreement_identity(&self) -> &str {
        match self {
            Self::SpatialPostureAgreementDenied {
                plane_agreement_identity,
                ..
            } => plane_agreement_identity,
        }
    }

    pub fn spatial_denial(&self) -> &PlanarBooleanCommonPlanePostureAgreementDenial {
        match self {
            Self::SpatialPostureAgreementDenied { denial, .. } => denial,
        }
    }
}
