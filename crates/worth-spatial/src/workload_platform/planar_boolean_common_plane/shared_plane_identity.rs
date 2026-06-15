use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::PlanarBooleanCommonPlaneAgreementReceipt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanCommonPlaneSharedPlaneIdentityReceipt {
    shared_plane_receipt_identity: String,
    plane_agreement_identity: String,
    shared_plane_identity: String,
    left_surface_support_identity: String,
    right_surface_support_identity: String,
}

impl PlanarBooleanCommonPlaneSharedPlaneIdentityReceipt {
    pub fn from_plane_agreement(agreement: &PlanarBooleanCommonPlaneAgreementReceipt) -> Self {
        let plane_agreement_identity = agreement.agreement_identity().to_string();
        let shared_plane_identity = agreement.shared_plane_identity().to_string();
        let left_surface_support_identity = agreement.left_surface_support_identity().to_string();
        let right_surface_support_identity = agreement.right_surface_support_identity().to_string();
        let shared_plane_receipt_identity = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "planar-boolean-common-plane-shared-plane-identity".to_string(),
                format!("plane-agreement:{plane_agreement_identity}"),
                format!("shared-plane:{shared_plane_identity}"),
                format!("left-support:{left_surface_support_identity}"),
                format!("right-support:{right_surface_support_identity}"),
            ],
        );
        Self {
            shared_plane_receipt_identity,
            plane_agreement_identity,
            shared_plane_identity,
            left_surface_support_identity,
            right_surface_support_identity,
        }
    }

    pub fn shared_plane_receipt_identity(&self) -> &str {
        &self.shared_plane_receipt_identity
    }

    pub fn plane_agreement_identity(&self) -> &str {
        &self.plane_agreement_identity
    }

    pub fn shared_plane_identity(&self) -> &str {
        &self.shared_plane_identity
    }

    pub fn left_surface_support_identity(&self) -> &str {
        &self.left_surface_support_identity
    }

    pub fn right_surface_support_identity(&self) -> &str {
        &self.right_surface_support_identity
    }
}
