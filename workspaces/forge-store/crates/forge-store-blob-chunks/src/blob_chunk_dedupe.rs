use forge_foundational::{
    canonicalization_api::common_path::canonicalization,
    canonicalization_api::lower_lane::basis::CanonicalizationRuleVersion,
    canonicalization_api::lower_lane::comparison::CanonicalComparisonOutcome, BoundaryArtifactId,
    CanonicalEquivalenceBasis, CanonicalIdentityInput,
};
use forge_proof::TransitionOutcome;
use forge_store_contracts::StableDigest;
use forge_store_security::StoreSecurityScopeIdentity;

use crate::{
    BlobChunkDedupeAdmissionDenial, BlobChunkDedupeCounterSnapshot, BlobChunkIdentity,
    BlobChunkStreamingObservation,
};

pub type BlobChunkDedupeAdmissionOutcome =
    TransitionOutcome<BlobChunkDedupeShareClaim, BlobChunkDedupeAdmissionDenial>;

#[derive(Debug, PartialEq, Eq)]
pub struct BlobChunkDedupeCandidate {
    identity: BlobChunkIdentity,
    content_digest: StableDigest,
    security_scope: StoreSecurityScopeIdentity,
}

impl BlobChunkDedupeCandidate {
    pub fn from_streaming_observation(observation: BlobChunkStreamingObservation) -> Self {
        let (identity, content_digest, security_scope) = observation.into_candidate_parts();
        Self {
            identity,
            content_digest,
            security_scope,
        }
    }

    pub const fn identity(&self) -> &BlobChunkIdentity {
        &self.identity
    }

    pub const fn content_digest(&self) -> &StableDigest {
        &self.content_digest
    }

    pub const fn security_scope(&self) -> StoreSecurityScopeIdentity {
        self.security_scope
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobChunkCanonicalEquivalence {
    basis: CanonicalEquivalenceBasis,
    existing_identity: BlobChunkIdentity,
    candidate_identity: BlobChunkIdentity,
    content_digest: StableDigest,
    security_scope: StoreSecurityScopeIdentity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobChunkCanonicalComparisonBasis {
    existing_identity: BlobChunkIdentity,
    candidate_identity: BlobChunkIdentity,
    content_digest: StableDigest,
    security_scope: StoreSecurityScopeIdentity,
    existing_artifact_id: BoundaryArtifactId,
    candidate_artifact_id: BoundaryArtifactId,
    rule_version: CanonicalizationRuleVersion,
}

impl BlobChunkCanonicalComparisonBasis {
    pub fn from_candidates(
        existing: &BlobChunkDedupeCandidate,
        candidate: &BlobChunkDedupeCandidate,
    ) -> Result<Self, BlobChunkDedupeAdmissionDenial> {
        if existing.content_digest != candidate.content_digest {
            return Err(BlobChunkDedupeAdmissionDenial::ContentDigestMismatch {
                counters: BlobChunkDedupeCounterSnapshot::start(),
            });
        }
        if existing.security_scope != candidate.security_scope {
            return Err(
                BlobChunkDedupeAdmissionDenial::CrossScopeSecurityWitnessMismatch {
                    counters: BlobChunkDedupeCounterSnapshot::start().record_cross_scope_denial(),
                },
            );
        }

        Ok(Self {
            existing_identity: existing.identity.clone(),
            candidate_identity: candidate.identity.clone(),
            content_digest: candidate.content_digest.clone(),
            security_scope: candidate.security_scope,
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
            _ => {
                return Err(
                    BlobChunkDedupeAdmissionDenial::UnsupportedFoundationalEquivalenceBasis {
                        basis: CanonicalEquivalenceBasis::ExactCanonicalBasis,
                    },
                )
            }
        };
        let outcome = match ready {
            TransitionOutcome::Success(ready) => canonicalization().compare().evaluate(&ready),
            _ => {
                return Err(
                    BlobChunkDedupeAdmissionDenial::UnsupportedFoundationalEquivalenceBasis {
                        basis: CanonicalEquivalenceBasis::ExactCanonicalBasis,
                    },
                )
            }
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
                Ok(BlobChunkCanonicalEquivalence {
                    basis: equivalent.equivalence_basis(),
                    existing_identity: self.existing_identity.clone(),
                    candidate_identity: self.candidate_identity.clone(),
                    content_digest: self.content_digest.clone(),
                    security_scope: self.security_scope,
                })
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

impl BlobChunkCanonicalEquivalence {
    pub const fn basis(&self) -> CanonicalEquivalenceBasis {
        self.basis
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
            && self.security_scope == candidate.security_scope
            && self.security_scope == existing.security_scope
    }
}

fn blob_chunk_canonical_rule_version() -> CanonicalizationRuleVersion {
    CanonicalizationRuleVersion::new("s51.blob.dedupe.candidate").expect("nonempty rule version")
}

fn candidate_artifact_id(candidate: &BlobChunkDedupeCandidate) -> BoundaryArtifactId {
    BoundaryArtifactId::new(stable_candidate_hash(candidate))
}

fn stable_candidate_hash(candidate: &BlobChunkDedupeCandidate) -> u64 {
    let mut hash = 0xcbf29ce484222325_u64;
    hash = hash_bytes(
        hash,
        candidate.identity.content_digest().as_str().as_bytes(),
    );
    hash = hash_bytes(hash, candidate.content_digest.as_str().as_bytes());
    hash = hash_u64(hash, key_scope_tag(candidate.security_scope.key_scope()));
    hash = hash_u64(
        hash,
        tenant_scope_tag(candidate.security_scope.tenant_scope()),
    );
    hash = hash_u64(
        hash,
        authenticity_requirement_tag(candidate.security_scope.authenticity_requirement()),
    );
    hash_u64(
        hash,
        custody_posture_tag(candidate.security_scope.custody_posture()),
    )
}

fn hash_bytes(mut hash: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

const fn hash_u64(hash: u64, value: u64) -> u64 {
    (hash ^ value).wrapping_mul(0x100000001b3)
}

const fn key_scope_tag(scope: forge_store_security::StoreKeyScope) -> u64 {
    match scope {
        forge_store_security::StoreKeyScope::StoreManagedRoot => 1,
        forge_store_security::StoreKeyScope::TenantEnvelope => 2,
        forge_store_security::StoreKeyScope::ArtifactEnvelope => 3,
        forge_store_security::StoreKeyScope::PageEnvelope => 4,
        forge_store_security::StoreKeyScope::WalCheckpointEnvelope => 5,
        forge_store_security::StoreKeyScope::BlobChunkEnvelope => 6,
        forge_store_security::StoreKeyScope::BackupExportEnvelope => 7,
        forge_store_security::StoreKeyScope::RepairScopeEnvelope => 8,
        forge_store_security::StoreKeyScope::SecurityLifecycleFoundation => 9,
    }
}

const fn tenant_scope_tag(scope: forge_store_security::StoreTenantScope) -> u64 {
    match scope {
        forge_store_security::StoreTenantScope::StoreInternal => 11,
        forge_store_security::StoreTenantScope::TenantPhysicalBoundary => 12,
        forge_store_security::StoreTenantScope::MultiTenantPhysicalBoundary => 13,
        forge_store_security::StoreTenantScope::BackupRestoreBoundary => 14,
        forge_store_security::StoreTenantScope::RepairBlastRadius => 15,
        forge_store_security::StoreTenantScope::ImportReadmissionBoundary => 16,
        forge_store_security::StoreTenantScope::SecurityLifecycleFoundation => 17,
    }
}

const fn authenticity_requirement_tag(
    requirement: forge_store_security::StoreAuthenticityRequirement,
) -> u64 {
    match requirement.class() {
        None => 20,
        Some(forge_store_security::StoreAuthenticityRequirementClass::AuthenticatedFrame) => 21,
        Some(forge_store_security::StoreAuthenticityRequirementClass::AuthenticatedWalRecord) => 22,
        Some(forge_store_security::StoreAuthenticityRequirementClass::AuthenticatedManifest) => 23,
        Some(forge_store_security::StoreAuthenticityRequirementClass::AuthenticatedBlobChunk) => 24,
        Some(
            forge_store_security::StoreAuthenticityRequirementClass::AuthenticatedBackupCapsule,
        ) => 25,
        Some(forge_store_security::StoreAuthenticityRequirementClass::AuthenticatedRepairRead) => {
            26
        }
    }
}

const fn custody_posture_tag(posture: forge_store_security::StoreCustodyPosture) -> u64 {
    match posture {
        forge_store_security::StoreCustodyPosture::InternalStoreCustody => 31,
        forge_store_security::StoreCustodyPosture::ExportPrepared => 32,
        forge_store_security::StoreCustodyPosture::ExportedOutOfCustody => 33,
        forge_store_security::StoreCustodyPosture::ImportedUnreadmitted => 34,
        forge_store_security::StoreCustodyPosture::Readmitted => 35,
        forge_store_security::StoreCustodyPosture::CustodyUnavailable => 36,
        forge_store_security::StoreCustodyPosture::CustodyDenied => 37,
        forge_store_security::StoreCustodyPosture::CustodyUnsupported => 38,
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BlobChunkDedupeAdmission {
    existing: BlobChunkDedupeCandidate,
    candidate: BlobChunkDedupeCandidate,
    equivalence: Option<BlobChunkCanonicalEquivalence>,
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
        }
    }

    pub fn with_foundational_canonical_equivalence(
        mut self,
        equivalence: BlobChunkCanonicalEquivalence,
    ) -> Self {
        self.equivalence = Some(equivalence);
        self
    }

    pub fn admit(self) -> BlobChunkDedupeAdmissionOutcome {
        let counters = BlobChunkDedupeCounterSnapshot::start();
        if self.existing.content_digest != self.candidate.content_digest {
            return TransitionOutcome::denied(
                BlobChunkDedupeAdmissionDenial::ContentDigestMismatch { counters },
            );
        }

        let Some(equivalence) = self.equivalence else {
            return deny_missing_equivalence(&self.existing, &self.candidate, counters);
        };

        let counters = counters.record_equivalence_comparison();
        if !equivalence.matches_candidates(&self.existing, &self.candidate) {
            return TransitionOutcome::denied(
                BlobChunkDedupeAdmissionDenial::UnboundFoundationalEquivalence {
                    counters: counters.record_cross_scope_denial(),
                },
            );
        }

        if self.existing.security_scope != self.candidate.security_scope {
            return deny_scope_mismatch(&self.existing, &self.candidate, counters);
        }

        TransitionOutcome::success(BlobChunkDedupeShareClaim {
            content_digest: self.candidate.content_digest,
            security_scope: self.candidate.security_scope,
            equivalence,
            counters: counters.record_same_scope_admission(),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BlobChunkDedupeShareClaim {
    content_digest: StableDigest,
    security_scope: StoreSecurityScopeIdentity,
    equivalence: BlobChunkCanonicalEquivalence,
    counters: BlobChunkDedupeCounterSnapshot,
}

impl BlobChunkDedupeShareClaim {
    pub const fn content_digest(&self) -> &StableDigest {
        &self.content_digest
    }

    pub const fn security_scope(&self) -> StoreSecurityScopeIdentity {
        self.security_scope
    }

    pub fn equivalence(&self) -> BlobChunkCanonicalEquivalence {
        self.equivalence.clone()
    }

    pub const fn counters(&self) -> BlobChunkDedupeCounterSnapshot {
        self.counters
    }
}

fn deny_missing_equivalence(
    existing: &BlobChunkDedupeCandidate,
    candidate: &BlobChunkDedupeCandidate,
    counters: BlobChunkDedupeCounterSnapshot,
) -> BlobChunkDedupeAdmissionOutcome {
    if existing.security_scope.tenant_scope() != candidate.security_scope.tenant_scope() {
        return TransitionOutcome::denied(
            BlobChunkDedupeAdmissionDenial::CrossTenantScopeRequiresExplicitEquivalence {
                left: existing.security_scope.tenant_scope(),
                right: candidate.security_scope.tenant_scope(),
                counters: counters.record_cross_scope_denial(),
            },
        );
    }

    if existing.security_scope.key_scope() != candidate.security_scope.key_scope() {
        return TransitionOutcome::denied(
            BlobChunkDedupeAdmissionDenial::CrossKeyScopeRequiresExplicitEquivalence {
                left: existing.security_scope.key_scope(),
                right: candidate.security_scope.key_scope(),
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
    if existing.security_scope.tenant_scope() != candidate.security_scope.tenant_scope() {
        return TransitionOutcome::denied(
            BlobChunkDedupeAdmissionDenial::CrossTenantScopeRequiresExplicitEquivalence {
                left: existing.security_scope.tenant_scope(),
                right: candidate.security_scope.tenant_scope(),
                counters: counters.record_cross_scope_denial(),
            },
        );
    }

    if existing.security_scope.key_scope() != candidate.security_scope.key_scope() {
        return TransitionOutcome::denied(
            BlobChunkDedupeAdmissionDenial::CrossKeyScopeRequiresExplicitEquivalence {
                left: existing.security_scope.key_scope(),
                right: candidate.security_scope.key_scope(),
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
