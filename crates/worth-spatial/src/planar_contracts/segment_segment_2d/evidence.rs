use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::CertifiedSegmentSegment2DBasis;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CertifiedSegmentSegment2DMutationEvidence {
    first_segment_identity: String,
    second_segment_identity: String,
    declaration_digest: String,
    envelope_digest: String,
    fact_digest: String,
    evidence_digest: String,
}

impl CertifiedSegmentSegment2DMutationEvidence {
    pub(crate) fn from_segment_fact(
        basis: &CertifiedSegmentSegment2DBasis,
        declaration_digest: &str,
        envelope_digest: &str,
        fact_digest: &str,
    ) -> Self {
        let first_segment_identity = basis.first_segment_identity().to_string();
        let second_segment_identity = basis.second_segment_identity().to_string();
        let declaration_digest = declaration_digest.to_string();
        let envelope_digest = envelope_digest.to_string();
        let fact_digest = fact_digest.to_string();
        let evidence_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                format!("first_segment:{first_segment_identity}"),
                format!("second_segment:{second_segment_identity}"),
                format!("classification:{}", basis.classification().as_str()),
                format!(
                    "projection_facts:{:?}",
                    basis.endpoint_projection_fact_digests()
                ),
                format!("predicate_facts:{:?}", basis.orientation_fact_digests()),
                format!("declaration:{declaration_digest}"),
                format!("envelope:{envelope_digest}"),
                format!("fact:{fact_digest}"),
            ],
        );
        Self {
            first_segment_identity,
            second_segment_identity,
            declaration_digest,
            envelope_digest,
            fact_digest,
            evidence_digest,
        }
    }

    pub fn first_segment_identity(&self) -> &str {
        &self.first_segment_identity
    }

    pub fn second_segment_identity(&self) -> &str {
        &self.second_segment_identity
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
