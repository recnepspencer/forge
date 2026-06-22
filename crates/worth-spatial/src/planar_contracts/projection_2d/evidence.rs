use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::ProjectPointToCertifiedPlane2DBasis;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectPointToCertifiedPlane2DMutationEvidence {
    source_point_identity: String,
    local_frame_fact_digest: String,
    declaration_digest: String,
    envelope_digest: String,
    fact_digest: String,
    evidence_digest: String,
}

impl ProjectPointToCertifiedPlane2DMutationEvidence {
    pub(crate) fn from_projection_fact(
        basis: &ProjectPointToCertifiedPlane2DBasis,
        declaration_digest: &str,
        envelope_digest: &str,
        fact_digest: &str,
    ) -> Self {
        let source_point_identity = basis.source_point_identity().to_string();
        let local_frame_fact_digest = basis.local_frame_fact_digest().to_string();
        let declaration_digest = declaration_digest.to_string();
        let envelope_digest = envelope_digest.to_string();
        let fact_digest = fact_digest.to_string();
        let evidence_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                format!("source_point_identity:{source_point_identity}"),
                format!("local_frame_fact:{local_frame_fact_digest}"),
                format!("point_2d:{:?}", basis.point_2d()),
                format!("signed_distance:{}", basis.signed_distance_to_plane_bits()),
                format!("declaration:{declaration_digest}"),
                format!("envelope:{envelope_digest}"),
                format!("fact:{fact_digest}"),
            ],
        );
        Self {
            source_point_identity,
            local_frame_fact_digest,
            declaration_digest,
            envelope_digest,
            fact_digest,
            evidence_digest,
        }
    }

    pub fn source_point_identity(&self) -> &str {
        &self.source_point_identity
    }

    pub fn local_frame_fact_digest(&self) -> &str {
        &self.local_frame_fact_digest
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn envelope_digest(&self) -> &str {
        &self.envelope_digest
    }

    pub fn fact_digest(&self) -> &str {
        &self.fact_digest
    }

    pub fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }
}
