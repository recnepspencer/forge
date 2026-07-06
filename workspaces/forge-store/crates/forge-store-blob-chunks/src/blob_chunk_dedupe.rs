use crate::{
    BlobChunkCollisionVerificationReceipt, BlobChunkContentDigest, BlobChunkDedupeAdmissionDenial,
    BlobChunkDedupeByteComparison, BlobChunkDedupeCollisionPosture, BlobChunkDedupeCounterSnapshot,
    BlobChunkDedupeDigestRewriteBasis, BlobChunkDedupeIndexPartition, BlobChunkDedupePolicy,
    BlobChunkDedupeReceipt, BlobChunkDedupeShareClaim, BlobChunkIdentity, BlobChunkIntegrityProof,
    BlobChunkRootCanonicalComparison, BlobChunkSecurityMetadataWitness, BlobCorruptionGuard,
};
use forge_foundational::CanonicalEquivalenceBasis;
use forge_proof::TransitionOutcome;
use forge_store_contracts::StableDigest;
pub type BlobChunkDedupeAdmissionOutcome =
    TransitionOutcome<BlobChunkDedupeShareClaim, BlobChunkDedupeAdmissionDenial>;

#[derive(Debug, PartialEq, Eq)]
pub struct BlobChunkDedupeCandidate {
    proof: BlobChunkIntegrityProof,
    identity: BlobChunkIdentity,
    content_digest: BlobChunkContentDigest,
    security_metadata: BlobChunkSecurityMetadataWitness,
}

impl BlobChunkDedupeCandidate {
    pub fn from_integrity_proof(proof: BlobChunkIntegrityProof) -> Self {
        Self {
            identity: proof.identity().clone(),
            content_digest: proof.content_digest().clone(),
            security_metadata: proof.security_metadata(),
            proof,
        }
    }

    #[cfg(test)]
    pub(crate) fn with_forced_content_digest_for_collision_fixture(
        mut self,
        content_digest: StableDigest,
    ) -> Self {
        self.content_digest = BlobChunkContentDigest::from_integrity_parts(content_digest);
        self
    }

    pub const fn identity(&self) -> &BlobChunkIdentity {
        &self.identity
    }

    pub const fn content_digest(&self) -> &StableDigest {
        self.content_digest.digest()
    }

    pub(crate) const fn content_digest_witness(&self) -> &BlobChunkContentDigest {
        &self.content_digest
    }

    pub const fn proof(&self) -> &BlobChunkIntegrityProof {
        &self.proof
    }

    pub const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.security_metadata
    }

    pub const fn security_scope(&self) -> forge_store_security::StoreSecurityScopeIdentity {
        self.security_metadata.identity()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobChunkCanonicalEquivalence {
    basis: CanonicalEquivalenceBasis,
    existing_identity: BlobChunkIdentity,
    candidate_identity: BlobChunkIdentity,
    content_digest: BlobChunkContentDigest,
    security_metadata: BlobChunkSecurityMetadataWitness,
}

impl BlobChunkCanonicalEquivalence {
    pub(crate) fn from_exact_canonical_basis(
        existing_identity: BlobChunkIdentity,
        candidate_identity: BlobChunkIdentity,
        content_digest: BlobChunkContentDigest,
        security_metadata: BlobChunkSecurityMetadataWitness,
    ) -> Self {
        Self {
            basis: CanonicalEquivalenceBasis::ExactCanonicalBasis,
            existing_identity,
            candidate_identity,
            content_digest,
            security_metadata,
        }
    }

    #[cfg(test)]
    pub(crate) fn forced_digest_collision_fixture(
        existing: &BlobChunkDedupeCandidate,
        candidate: &BlobChunkDedupeCandidate,
    ) -> Self {
        Self {
            basis: CanonicalEquivalenceBasis::ExactCanonicalBasis,
            existing_identity: existing.identity.clone(),
            candidate_identity: candidate.identity.clone(),
            content_digest: candidate.content_digest.clone(),
            security_metadata: candidate.security_metadata,
        }
    }

    pub const fn basis(&self) -> CanonicalEquivalenceBasis {
        self.basis
    }

    pub const fn existing_identity(&self) -> &BlobChunkIdentity {
        &self.existing_identity
    }

    pub const fn candidate_identity(&self) -> &BlobChunkIdentity {
        &self.candidate_identity
    }

    fn matches_candidates(
        &self,
        existing: &BlobChunkDedupeCandidate,
        candidate: &BlobChunkDedupeCandidate,
    ) -> bool {
        self.existing_identity == existing.identity
            && self.candidate_identity == candidate.identity
            && self.content_digest == candidate.content_digest
            && self.content_digest == existing.content_digest
            && self.security_metadata == candidate.security_metadata
            && self.security_metadata == existing.security_metadata
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BlobChunkDedupeAdmission {
    existing: BlobChunkDedupeCandidate,
    candidate: BlobChunkDedupeCandidate,
    equivalence: Option<BlobChunkCanonicalEquivalence>,
    root_comparison: Option<BlobChunkRootCanonicalComparison>,
    byte_comparison: Option<BlobChunkDedupeByteComparison>,
    policy: BlobChunkDedupePolicy,
}

impl BlobChunkDedupeAdmission {
    pub fn compare_candidates(
        existing: BlobChunkDedupeCandidate,
        candidate: BlobChunkDedupeCandidate,
    ) -> Self {
        Self {
            existing,
            candidate,
            equivalence: None,
            root_comparison: None,
            byte_comparison: None,
            policy: BlobChunkDedupePolicy::same_tenant_same_key_scope(),
        }
    }

    pub fn with_foundational_canonical_equivalence(
        mut self,
        equivalence: BlobChunkCanonicalEquivalence,
    ) -> Self {
        self.equivalence = Some(equivalence);
        self
    }

    pub fn with_root_canonical_comparison(
        mut self,
        comparison: BlobChunkRootCanonicalComparison,
    ) -> Self {
        self.root_comparison = Some(comparison);
        self
    }

    pub fn with_byte_comparison(mut self, comparison: BlobChunkDedupeByteComparison) -> Self {
        self.byte_comparison = Some(comparison);
        self
    }

    pub fn with_policy(mut self, policy: BlobChunkDedupePolicy) -> Self {
        self.policy = policy;
        self
    }

    pub fn admit(self) -> BlobChunkDedupeAdmissionOutcome {
        let counters = BlobChunkDedupeCounterSnapshot::start();
        if self.existing.content_digest != self.candidate.content_digest {
            return TransitionOutcome::denied(
                BlobChunkDedupeAdmissionDenial::ContentDigestMismatch {
                    counters: counters.record_dedupe_miss(),
                },
            );
        }

        let Some(equivalence) = self.equivalence else {
            return deny_missing_equivalence(&self.existing, &self.candidate, counters);
        };

        let counters = counters.record_equivalence_comparison();
        if !self.policy.allows_same_scope_sharing() {
            return TransitionOutcome::denied(BlobChunkDedupeAdmissionDenial::DedupePolicyDenied {
                policy: self.policy,
                counters: counters.record_dedupe_miss(),
            });
        }

        if self.existing.identity != self.candidate.identity {
            let Some(root_comparison) = self.root_comparison else {
                return TransitionOutcome::denied(
                    BlobChunkDedupeAdmissionDenial::CanonicalRootComparisonRequired {
                        counters: counters.record_collision_probe(),
                    },
                );
            };
            let Some(byte_comparison) = self.byte_comparison else {
                return TransitionOutcome::denied(
                    BlobChunkDedupeAdmissionDenial::ChunkByteComparisonRequired {
                        counters: counters.record_collision_probe(),
                    },
                );
            };
            if !byte_comparison
                .matches_candidate_identities(&self.existing.identity, &self.candidate.identity)
            {
                return TransitionOutcome::denied(
                    BlobChunkDedupeAdmissionDenial::UnboundByteComparison {
                        counters: counters.record_cross_scope_denial(),
                    },
                );
            }
            if !root_comparison
                .matches_candidate_identities(&self.existing.identity, &self.candidate.identity)
            {
                return TransitionOutcome::denied(
                    BlobChunkDedupeAdmissionDenial::UnboundRootCanonicalComparison {
                        counters: counters.record_cross_scope_denial(),
                    },
                );
            }
            if root_comparison.is_equivalent() && byte_comparison.is_equivalent() {
                if !equivalence.matches_candidates(&self.existing, &self.candidate) {
                    return TransitionOutcome::denied(
                        BlobChunkDedupeAdmissionDenial::UnboundFoundationalEquivalence {
                            counters: counters.record_cross_scope_denial(),
                        },
                    );
                }
                if self.existing.security_metadata != self.candidate.security_metadata {
                    return deny_scope_mismatch(&self.existing, &self.candidate, counters);
                }
                return TransitionOutcome::success(
                    BlobChunkDedupeReceipt::from_admitted_equivalence(
                        self.candidate.content_digest.digest().clone(),
                        self.candidate.security_metadata,
                        self.policy,
                        equivalence,
                        BlobChunkDedupeCollisionPosture::VerifiedEquivalent,
                        byte_comparison
                            .counters()
                            .record_equivalence_comparison()
                            .record_same_scope_admission(),
                    ),
                );
            }
            let bytes_compared = byte_comparison.bytes_compared();
            let collision_counters = byte_comparison.counters_for_collision_denial();
            let receipt = BlobChunkCollisionVerificationReceipt::from_verified_identity_mismatch(
                self.existing.proof,
                self.candidate.proof,
                self.candidate.content_digest,
                self.candidate.security_metadata,
                bytes_compared,
                collision_counters,
            );
            return TransitionOutcome::denied(
                BlobChunkDedupeAdmissionDenial::DigestCollisionDenied {
                    posture: BlobChunkDedupeCollisionPosture::DigestCollisionDenied,
                    counters: receipt.counters(),
                    receipt,
                },
            );
        }

        if !equivalence.matches_candidates(&self.existing, &self.candidate) {
            return TransitionOutcome::denied(
                BlobChunkDedupeAdmissionDenial::UnboundFoundationalEquivalence {
                    counters: counters.record_cross_scope_denial(),
                },
            );
        }

        if self.existing.security_metadata != self.candidate.security_metadata {
            return deny_scope_mismatch(&self.existing, &self.candidate, counters);
        }

        TransitionOutcome::success(BlobChunkDedupeReceipt::from_admitted_equivalence(
            self.candidate.content_digest.digest().clone(),
            self.candidate.security_metadata,
            self.policy,
            equivalence,
            BlobChunkDedupeCollisionPosture::VerifiedEquivalent,
            counters.record_same_scope_admission(),
        ))
    }

    pub fn deny_for_quarantine(guard: &BlobCorruptionGuard) -> BlobChunkDedupeAdmissionOutcome {
        let _ = guard.deny_dedupe();
        let counters = BlobChunkDedupeCounterSnapshot::start().record_quarantine_denial();
        TransitionOutcome::denied(BlobChunkDedupeAdmissionDenial::QuarantinedChunkDenied {
            quarantine: guard.quarantine().clone(),
            posture: BlobChunkDedupeCollisionPosture::DigestAlgorithmQuarantined,
            counters,
        })
    }

    pub fn deny_for_index_partitioned(
        partition: BlobChunkDedupeIndexPartition,
    ) -> BlobChunkDedupeAdmissionOutcome {
        TransitionOutcome::denied(BlobChunkDedupeAdmissionDenial::DedupeIndexPartitioned {
            posture: BlobChunkDedupeCollisionPosture::DedupeIndexPartitioned,
            counters: partition.counters(),
        })
    }

    pub fn deny_for_digest_rewrite(
        rewrite: BlobChunkDedupeDigestRewriteBasis,
    ) -> BlobChunkDedupeAdmissionOutcome {
        TransitionOutcome::denied(
            BlobChunkDedupeAdmissionDenial::ChunkRewrittenUnderNewDigestBasis {
                posture: BlobChunkDedupeCollisionPosture::ChunkRewrittenUnderNewDigestBasis,
                counters: rewrite.counters(),
            },
        )
    }
}

fn deny_missing_equivalence(
    existing: &BlobChunkDedupeCandidate,
    candidate: &BlobChunkDedupeCandidate,
    counters: BlobChunkDedupeCounterSnapshot,
) -> BlobChunkDedupeAdmissionOutcome {
    if existing.security_metadata.tenant_scope() != candidate.security_metadata.tenant_scope() {
        return TransitionOutcome::denied(
            BlobChunkDedupeAdmissionDenial::CrossTenantScopeRequiresExplicitEquivalence {
                left: existing.security_metadata.tenant_scope(),
                right: candidate.security_metadata.tenant_scope(),
                counters: counters.record_cross_scope_denial(),
            },
        );
    }

    if existing.security_metadata.key_scope() != candidate.security_metadata.key_scope() {
        return TransitionOutcome::denied(
            BlobChunkDedupeAdmissionDenial::CrossKeyScopeRequiresExplicitEquivalence {
                left: existing.security_metadata.key_scope(),
                right: candidate.security_metadata.key_scope(),
                counters: counters.record_cross_scope_denial(),
            },
        );
    }

    let counters = counters.record_digest_only_denial();
    TransitionOutcome::denied(
        BlobChunkDedupeAdmissionDenial::MissingFoundationalCanonicalEquivalence { counters },
    )
}

fn deny_scope_mismatch(
    existing: &BlobChunkDedupeCandidate,
    candidate: &BlobChunkDedupeCandidate,
    counters: BlobChunkDedupeCounterSnapshot,
) -> BlobChunkDedupeAdmissionOutcome {
    if existing.security_metadata.tenant_scope() != candidate.security_metadata.tenant_scope() {
        return TransitionOutcome::denied(
            BlobChunkDedupeAdmissionDenial::CrossTenantScopeRequiresExplicitEquivalence {
                left: existing.security_metadata.tenant_scope(),
                right: candidate.security_metadata.tenant_scope(),
                counters: counters.record_cross_scope_denial(),
            },
        );
    }

    if existing.security_metadata.key_scope() != candidate.security_metadata.key_scope() {
        return TransitionOutcome::denied(
            BlobChunkDedupeAdmissionDenial::CrossKeyScopeRequiresExplicitEquivalence {
                left: existing.security_metadata.key_scope(),
                right: candidate.security_metadata.key_scope(),
                counters: counters.record_cross_scope_denial(),
            },
        );
    }

    TransitionOutcome::denied(
        BlobChunkDedupeAdmissionDenial::CrossScopeSecurityWitnessMismatch {
            counters: counters.record_cross_scope_denial(),
        },
    )
}
