use crate::planar_contracts::signed_area_2d::CertifiedSignedArea2DReceipt;

use super::{CoplanarOverlapDenial, CoplanarOverlapDenialBasisLocus, CoplanarOverlapDenialKind};

#[derive(Clone, Debug, PartialEq)]
pub struct CertifiedCoplanarOverlapFace2D {
    face_identity: String,
    signed_area_receipt: CertifiedSignedArea2DReceipt,
}

impl CertifiedCoplanarOverlapFace2D {
    pub fn from_certified_area(
        face_identity: impl Into<String>,
        signed_area_receipt: CertifiedSignedArea2DReceipt,
    ) -> Result<Self, CoplanarOverlapDenial> {
        let face_identity = face_identity.into();
        if face_identity.is_empty() {
            return Err(CoplanarOverlapDenial::new(
                CoplanarOverlapDenialKind::MissingFaceIdentity,
                CoplanarOverlapDenialBasisLocus::FaceIdentity,
                "coplanar overlap faces require stable face identity",
            ));
        }
        Ok(Self {
            face_identity,
            signed_area_receipt,
        })
    }

    pub fn face_identity(&self) -> &str {
        &self.face_identity
    }

    pub fn signed_area_receipt(&self) -> &CertifiedSignedArea2DReceipt {
        &self.signed_area_receipt
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CanonicalFacePairIdentity {
    identity: String,
}

impl CanonicalFacePairIdentity {
    pub(crate) fn from_faces(
        first: &CertifiedCoplanarOverlapFace2D,
        second: &CertifiedCoplanarOverlapFace2D,
    ) -> Self {
        let mut parts = [
            first.signed_area_receipt().fact_digest().to_string(),
            second.signed_area_receipt().fact_digest().to_string(),
        ];
        parts.sort();
        Self {
            identity: format!("{}::{}", parts[0], parts[1]),
        }
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.identity
    }
}
