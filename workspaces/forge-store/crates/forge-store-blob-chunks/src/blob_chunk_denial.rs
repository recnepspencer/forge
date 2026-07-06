use forge_foundational::CanonicalEquivalenceBasis;
use forge_store_readiness::S51SecurityScopeReadinessFamily;
use forge_store_security::{
    StoreApplicationOrgIdClaim, StoreAuthenticityRequirement, StoreCustodyPosture,
    StoreIamRoleClaim, StoreJwtSubjectClaim, StoreKeyScope, StoreKeyVersionPosture,
    StoreKmsKeyIdentifier, StoreOperatorIdentityClaim, StoreRawSecurityScopeDeclaration,
    StoreTenantScope,
};

use crate::{
    BlobChunkCollisionVerificationReceipt, BlobChunkDedupeCollisionPosture,
    BlobChunkDedupeCounterSnapshot, BlobChunkDedupePolicy, BlobChunkQuarantine,
    BlobChunkScopeCounterSnapshot,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobChunkSecurityScopeDenial {
    WrongReadinessFamily {
        actual: S51SecurityScopeReadinessFamily,
        counters: BlobChunkScopeCounterSnapshot,
    },
    WrongKeyScope {
        actual: StoreKeyScope,
        counters: BlobChunkScopeCounterSnapshot,
    },
    WrongTenantScope {
        actual: StoreTenantScope,
        counters: BlobChunkScopeCounterSnapshot,
    },
    WrongAuthenticityRequirement {
        actual: StoreAuthenticityRequirement,
        counters: BlobChunkScopeCounterSnapshot,
    },
    UnsupportedCustodyPosture {
        actual: StoreCustodyPosture,
        counters: BlobChunkScopeCounterSnapshot,
    },
    StaleKeyVersionPosture {
        actual: StoreKeyVersionPosture,
        counters: BlobChunkScopeCounterSnapshot,
    },
    IdentityProviderClaimRejected {
        counters: BlobChunkScopeCounterSnapshot,
    },
    ApplicationOrgClaimRejected {
        counters: BlobChunkScopeCounterSnapshot,
    },
    KmsKeyIdentifierRejected {
        counters: BlobChunkScopeCounterSnapshot,
    },
    IamRoleClaimRejected {
        counters: BlobChunkScopeCounterSnapshot,
    },
    OperatorIdentityRejected {
        counters: BlobChunkScopeCounterSnapshot,
    },
    DeserializedMetadataRequiresReadmission {
        counters: BlobChunkScopeCounterSnapshot,
    },
}

pub fn reject_jwt_claim_as_blob_chunk_security_scope(
    _claim: StoreJwtSubjectClaim,
) -> BlobChunkSecurityScopeDenial {
    BlobChunkSecurityScopeDenial::IdentityProviderClaimRejected {
        counters: BlobChunkScopeCounterSnapshot::start().denied_hostile_metadata(),
    }
}

pub fn reject_application_org_claim_as_blob_chunk_security_scope(
    _claim: StoreApplicationOrgIdClaim,
) -> BlobChunkSecurityScopeDenial {
    BlobChunkSecurityScopeDenial::ApplicationOrgClaimRejected {
        counters: BlobChunkScopeCounterSnapshot::start().denied_hostile_metadata(),
    }
}

pub fn reject_kms_key_id_as_blob_chunk_security_scope(
    _claim: StoreKmsKeyIdentifier,
) -> BlobChunkSecurityScopeDenial {
    BlobChunkSecurityScopeDenial::KmsKeyIdentifierRejected {
        counters: BlobChunkScopeCounterSnapshot::start().denied_hostile_metadata(),
    }
}

pub fn reject_iam_role_as_blob_chunk_security_scope(
    _claim: StoreIamRoleClaim,
) -> BlobChunkSecurityScopeDenial {
    BlobChunkSecurityScopeDenial::IamRoleClaimRejected {
        counters: BlobChunkScopeCounterSnapshot::start().denied_hostile_metadata(),
    }
}

pub fn reject_operator_identity_as_blob_chunk_security_scope(
    _claim: StoreOperatorIdentityClaim,
) -> BlobChunkSecurityScopeDenial {
    BlobChunkSecurityScopeDenial::OperatorIdentityRejected {
        counters: BlobChunkScopeCounterSnapshot::start().denied_hostile_metadata(),
    }
}

pub const fn reject_deserialized_metadata_as_blob_chunk_security_scope(
    _declaration: StoreRawSecurityScopeDeclaration,
) -> BlobChunkSecurityScopeDenial {
    BlobChunkSecurityScopeDenial::DeserializedMetadataRequiresReadmission {
        counters: BlobChunkScopeCounterSnapshot::start().denied_hostile_metadata(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobChunkStreamingDenial {
    EmptyStreamingWindow,
    WindowDigestMismatch,
    WholeObjectResidencyRequired,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlobChunkDedupeAdmissionDenial {
    ContentDigestMismatch {
        counters: BlobChunkDedupeCounterSnapshot,
    },
    MissingFoundationalCanonicalEquivalence {
        counters: BlobChunkDedupeCounterSnapshot,
    },
    DigestOnlyEquivalenceRejected,
    CanonicalRootComparisonRequired {
        counters: BlobChunkDedupeCounterSnapshot,
    },
    UnboundRootCanonicalComparison {
        counters: BlobChunkDedupeCounterSnapshot,
    },
    ChunkByteComparisonRequired {
        counters: BlobChunkDedupeCounterSnapshot,
    },
    UnboundByteComparison {
        counters: BlobChunkDedupeCounterSnapshot,
    },
    ByteComparisonPayloadMismatch {
        counters: BlobChunkDedupeCounterSnapshot,
    },
    DigestCollisionDenied {
        receipt: BlobChunkCollisionVerificationReceipt,
        posture: BlobChunkDedupeCollisionPosture,
        counters: BlobChunkDedupeCounterSnapshot,
    },
    CrossTenantScopeRequiresExplicitEquivalence {
        left: StoreTenantScope,
        right: StoreTenantScope,
        counters: BlobChunkDedupeCounterSnapshot,
    },
    CrossKeyScopeRequiresExplicitEquivalence {
        left: StoreKeyScope,
        right: StoreKeyScope,
        counters: BlobChunkDedupeCounterSnapshot,
    },
    CrossScopeSecurityWitnessMismatch {
        counters: BlobChunkDedupeCounterSnapshot,
    },
    QuarantinedChunkDenied {
        quarantine: BlobChunkQuarantine,
        posture: BlobChunkDedupeCollisionPosture,
        counters: BlobChunkDedupeCounterSnapshot,
    },
    DedupeIndexPartitioned {
        posture: BlobChunkDedupeCollisionPosture,
        counters: BlobChunkDedupeCounterSnapshot,
    },
    ChunkRewrittenUnderNewDigestBasis {
        posture: BlobChunkDedupeCollisionPosture,
        counters: BlobChunkDedupeCounterSnapshot,
    },
    DedupePolicyDenied {
        policy: BlobChunkDedupePolicy,
        counters: BlobChunkDedupeCounterSnapshot,
    },
    DedupeReferenceEdgeMismatch {
        counters: BlobChunkDedupeCounterSnapshot,
    },
    UnboundFoundationalEquivalence {
        counters: BlobChunkDedupeCounterSnapshot,
    },
    UnsupportedFoundationalEquivalenceBasis {
        basis: CanonicalEquivalenceBasis,
    },
}
