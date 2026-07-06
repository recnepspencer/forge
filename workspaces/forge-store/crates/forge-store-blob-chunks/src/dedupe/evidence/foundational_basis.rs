use forge_foundational::{
    canonicalization_api::common_path::canonicalization,
    canonicalization_api::lower_lane::basis::CanonicalizationRuleVersion,
    canonicalization_api::lower_lane::comparison::CanonicalComparisonOutcome, BoundaryArtifactId,
    CanonicalEquivalenceBasis, CanonicalIdentityInput,
};
use forge_proof::TransitionOutcome;

use super::artifact_identity::candidate_artifact_id;
use super::candidate::BlobChunkDedupeCandidate;
use super::canonical_equivalence::BlobChunkCanonicalEquivalence;
use crate::{
    BlobChunkDedupeAdmissionDenial, BlobChunkDedupeCounterSnapshot, BlobChunkIdentity,
    BlobChunkSecurityMetadataWitness,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobChunkCanonicalComparisonBasis {
    existing_identity: BlobChunkIdentity,
    candidate_identity: BlobChunkIdentity,
    content_digest: crate::BlobChunkContentDigest,
    security_metadata: BlobChunkSecurityMetadataWitness,
    existing_artifact_id: BoundaryArtifactId,
    candidate_artifact_id: BoundaryArtifactId,
    rule_version: CanonicalizationRuleVersion,
}

impl BlobChunkCanonicalComparisonBasis {
    pub fn from_candidates(
        existing: &BlobChunkDedupeCandidate,
        candidate: &BlobChunkDedupeCandidate,
    ) -> Result<Self, BlobChunkDedupeAdmissionDenial> {
        if existing.content_digest_witness() != candidate.content_digest_witness() {
            return Err(BlobChunkDedupeAdmissionDenial::ContentDigestMismatch {
                counters: BlobChunkDedupeCounterSnapshot::start(),
            });
        }
        if existing.security_metadata() != candidate.security_metadata() {
            return Err(
                BlobChunkDedupeAdmissionDenial::CrossScopeSecurityWitnessMismatch {
                    counters: BlobChunkDedupeCounterSnapshot::start().record_cross_scope_denial(),
                },
            );
        }

        Ok(Self {
            existing_identity: existing.identity().clone(),
            candidate_identity: candidate.identity().clone(),
            content_digest: candidate.content_digest_witness().clone(),
            security_metadata: candidate.security_metadata(),
            existing_artifact_id: candidate_artifact_id(existing),
            candidate_artifact_id: candidate_artifact_id(candidate),
            rule_version: blob_chunk_canonical_rule_version(),
        })
    }

    pub const fn existing_artifact_id(&self) -> BoundaryArtifactId {
        self.existing_artifact_id
    }

    pub const fn candidate_artifact_id(&self) -> BoundaryArtifactId {
        self.candidate_artifact_id
    }

    pub fn evaluate_foundational_equivalence(
        &self,
    ) -> Result<BlobChunkCanonicalEquivalence, BlobChunkDedupeAdmissionDenial> {
        let left = canonicalization()
            .basis()
            .at(self.rule_version.clone())
            .from_identity(CanonicalIdentityInput::BoundaryArtifact(
                self.existing_artifact_id,
            ));
        let right = canonicalization()
            .basis()
            .at(self.rule_version.clone())
            .from_identity(CanonicalIdentityInput::BoundaryArtifact(
                self.candidate_artifact_id,
            ));

        let ready = match (left, right) {
            (TransitionOutcome::Success(left), TransitionOutcome::Success(right)) => {
                canonicalization()
                    .compare()
                    .left(left)
                    .right(right)
                    .under(CanonicalEquivalenceBasis::ExactCanonicalBasis)
            }
            _ => return unsupported_exact_basis(),
        };
        let outcome = match ready {
            TransitionOutcome::Success(ready) => canonicalization().compare().evaluate(&ready),
            _ => return unsupported_exact_basis(),
        };

        match outcome {
            CanonicalComparisonOutcome::Equivalent(equivalent) => {
                if equivalent.equivalence_basis() != CanonicalEquivalenceBasis::ExactCanonicalBasis
                    || *equivalent.left_version() != self.rule_version
                    || *equivalent.right_version() != self.rule_version
                    || equivalent.entry_count() != 1
                {
                    return Err(
                        BlobChunkDedupeAdmissionDenial::UnsupportedFoundationalEquivalenceBasis {
                            basis: equivalent.equivalence_basis(),
                        },
                    );
                }
                Ok(BlobChunkCanonicalEquivalence::from_exact_canonical_basis(
                    self.existing_identity.clone(),
                    self.candidate_identity.clone(),
                    self.content_digest.clone(),
                    self.security_metadata,
                ))
            }
            CanonicalComparisonOutcome::Mismatched(_)
            | CanonicalComparisonOutcome::Unsupported(_) => Err(
                BlobChunkDedupeAdmissionDenial::UnboundFoundationalEquivalence {
                    counters: BlobChunkDedupeCounterSnapshot::start().record_cross_scope_denial(),
                },
            ),
        }
    }
}

fn unsupported_exact_basis<T>() -> Result<T, BlobChunkDedupeAdmissionDenial> {
    Err(
        BlobChunkDedupeAdmissionDenial::UnsupportedFoundationalEquivalenceBasis {
            basis: CanonicalEquivalenceBasis::ExactCanonicalBasis,
        },
    )
}

fn blob_chunk_canonical_rule_version() -> CanonicalizationRuleVersion {
    CanonicalizationRuleVersion::new("s51.blob.dedupe.candidate").expect("nonempty rule version")
}