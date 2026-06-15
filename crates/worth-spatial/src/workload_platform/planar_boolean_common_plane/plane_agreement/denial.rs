use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanCommonPlaneAgreementDenialKind {
    MissingDeclaration,
    MissingCertifiedFacePlaneWitness,
    AmbiguousCertifiedFacePlaneWitness,
    DistinctCertifiedPlanes,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlanarBooleanCommonPlaneAgreementDenial {
    MissingDeclaration,
    MissingCertifiedFacePlaneWitness {
        side: PlanarBooleanCommonPlaneOperandSide,
        surface_support_identity: String,
    },
    AmbiguousCertifiedFacePlaneWitness {
        side: PlanarBooleanCommonPlaneOperandSide,
        surface_support_identity: String,
        plane_identity_count: usize,
    },
    DistinctCertifiedPlanes {
        left_surface_support_identity: String,
        right_surface_support_identity: String,
        left_plane_identity_digest: String,
        right_plane_identity_digest: String,
    },
}

impl PlanarBooleanCommonPlaneAgreementDenial {
    pub fn kind(&self) -> PlanarBooleanCommonPlaneAgreementDenialKind {
        match self {
            Self::MissingDeclaration => {
                PlanarBooleanCommonPlaneAgreementDenialKind::MissingDeclaration
            }
            Self::MissingCertifiedFacePlaneWitness { .. } => {
                PlanarBooleanCommonPlaneAgreementDenialKind::MissingCertifiedFacePlaneWitness
            }
            Self::AmbiguousCertifiedFacePlaneWitness { .. } => {
                PlanarBooleanCommonPlaneAgreementDenialKind::AmbiguousCertifiedFacePlaneWitness
            }
            Self::DistinctCertifiedPlanes { .. } => {
                PlanarBooleanCommonPlaneAgreementDenialKind::DistinctCertifiedPlanes
            }
        }
    }

    pub fn human_reason(&self) -> &'static str {
        match self {
            Self::MissingDeclaration => {
                "Common-plane agreement requires a human-readable declaration."
            }
            Self::MissingCertifiedFacePlaneWitness { .. } => {
                "Common-plane agreement requires a certified face-plane witness on each operand before reduction can continue."
            }
            Self::AmbiguousCertifiedFacePlaneWitness { .. } => {
                "Common-plane agreement requires exactly one certified face-plane witness per operand in this phase."
            }
            Self::DistinctCertifiedPlanes { .. } => {
                "Common-plane agreement denied the pair because the certified operand planes are distinct."
            }
        }
    }
}
