use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::PlanarBooleanCommonPlaneWitness;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanCommonPlaneAgreementReceipt {
    agreement_identity: String,
    shared_plane_identity: String,
    left_surface_support_identity: String,
    right_surface_support_identity: String,
    left_witness: PlanarBooleanCommonPlaneWitness,
    right_witness: PlanarBooleanCommonPlaneWitness,
}

impl PlanarBooleanCommonPlaneAgreementReceipt {
    pub(crate) fn new(
        declaration: &str,
        left_surface_support_identity: impl Into<String>,
        right_surface_support_identity: impl Into<String>,
        left_witness: PlanarBooleanCommonPlaneWitness,
        right_witness: PlanarBooleanCommonPlaneWitness,
    ) -> Self {
        let left_surface_support_identity = left_surface_support_identity.into();
        let right_surface_support_identity = right_surface_support_identity.into();
        let shared_plane_identity = left_witness.plane_identity_digest().to_string();
        let agreement_identity = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "planar-boolean-common-plane-agreement".to_string(),
                format!("declaration:{declaration}"),
                format!("shared-plane:{shared_plane_identity}"),
                format!("left-support:{left_surface_support_identity}"),
                format!("right-support:{right_surface_support_identity}"),
                format!("left-face-rows:{}", left_witness.supporting_face_rows()),
                format!("right-face-rows:{}", right_witness.supporting_face_rows()),
            ],
        );
        Self {
            agreement_identity,
            shared_plane_identity,
            left_surface_support_identity,
            right_surface_support_identity,
            left_witness,
            right_witness,
        }
    }

    pub fn agreement_identity(&self) -> &str {
        &self.agreement_identity
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

    pub fn left_witness(&self) -> &PlanarBooleanCommonPlaneWitness {
        &self.left_witness
    }

    pub fn right_witness(&self) -> &PlanarBooleanCommonPlaneWitness {
        &self.right_witness
    }
}
