#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlanarBooleanCommonPlaneSharedPlaneIdentityError {
    PlaneAgreementIdentityMismatch {
        expected_plane_agreement_identity: String,
        actual_plane_agreement_identity: String,
    },
    SharedPlaneIdentityMismatch {
        expected_shared_plane_identity: String,
        actual_shared_plane_identity: String,
    },
}

impl PlanarBooleanCommonPlaneSharedPlaneIdentityError {
    pub fn human_reason(&self) -> &'static str {
        match self {
            Self::PlaneAgreementIdentityMismatch { .. } => {
                "shared-plane identity receipt must come from the same certified common-plane agreement"
            }
            Self::SharedPlaneIdentityMismatch { .. } => {
                "shared-plane identity receipt must preserve the certified shared plane identity"
            }
        }
    }
}
