use crate::dedupe::classification::{classify_dedupe_case, DedupeCase};
use crate::dedupe::evidence::{
    digest_gate, BlobChunkCanonicalEquivalence, BlobChunkDedupeByteComparison,
    BlobChunkDedupeCandidate,
};
use crate::dedupe::receipt_construction::denial_assembly;
use crate::dedupe::transitions::{admit_cross_identity_case, admit_same_identity_case};
use crate::dedupe::verification::verify_policy_allows_sharing;
use crate::{
    BlobChunkDedupeAdmissionDenial, BlobChunkDedupeCollisionPosture,
    BlobChunkDedupeCounterSnapshot, BlobChunkDedupeDigestRewriteBasis,
    BlobChunkDedupeIndexPartition, BlobChunkDedupePolicy, BlobChunkDedupeShareClaim,
    BlobChunkRootCanonicalComparison, BlobCorruptionGuard,
};
use worth_proof::TransitionOutcome;

pub type BlobChunkDedupeAdmissionOutcome =
    TransitionOutcome<BlobChunkDedupeShareClaim, BlobChunkDedupeAdmissionDenial>;

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
        let Self {
            existing,
            candidate,
            equivalence,
            root_comparison,
            byte_comparison,
            policy,
        } = self;

        let counters = match digest_gate(&existing, &candidate, counters) {
            Ok(counters) => counters,
            Err(denial) => return denial,
        };

        let Some(equivalence) = equivalence else {
            return denial_assembly::deny_missing_equivalence(&existing, &candidate, counters);
        };

        let counters = counters.record_equivalence_comparison();
        let counters = match verify_policy_allows_sharing(policy, counters) {
            Ok(counters) => counters,
            Err(denial) => return denial,
        };

        match classify_dedupe_case(&existing, &candidate) {
            DedupeCase::CrossIdentity => admit_cross_identity_case(
                existing,
                candidate,
                root_comparison,
                byte_comparison,
                policy,
                equivalence,
                counters,
            ),
            DedupeCase::SameIdentity => {
                admit_same_identity_case(existing, candidate, policy, equivalence, counters)
            }
        }
    }

    pub fn deny_for_quarantine(guard: &BlobCorruptionGuard) -> BlobChunkDedupeAdmissionOutcome {
        let _ = guard.deny_dedupe();
        let counters = BlobChunkDedupeCounterSnapshot::start().record_quarantine_denial();
        TransitionOutcome::denied(BlobChunkDedupeAdmissionDenial::QuarantinedChunkDenied {
            quarantine: Box::new(guard.quarantine().clone()),
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
