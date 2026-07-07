use crate::BlobChunkSecurityMetadataWitness;
use forge_store_security::{StoreKeyScope, StoreTenantScope};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeMismatchCase {
    TenantScope {
        left: StoreTenantScope,
        right: StoreTenantScope,
    },
    KeyScope {
        left: StoreKeyScope,
        right: StoreKeyScope,
    },
    KeyVersionPosture,
    AuthenticityRequirement,
    CustodyPosture,
    FullWitness,
}

pub(crate) fn classify_scope_mismatch(
    existing: BlobChunkSecurityMetadataWitness,
    candidate: BlobChunkSecurityMetadataWitness,
) -> Option<ScopeMismatchCase> {
    if existing == candidate {
        return None;
    }
    if existing.tenant_scope() != candidate.tenant_scope() {
        return Some(ScopeMismatchCase::TenantScope {
            left: existing.tenant_scope(),
            right: candidate.tenant_scope(),
        });
    }
    if existing.key_scope() != candidate.key_scope() {
        return Some(ScopeMismatchCase::KeyScope {
            left: existing.key_scope(),
            right: candidate.key_scope(),
        });
    }
    if existing.key_version_posture() != candidate.key_version_posture() {
        return Some(ScopeMismatchCase::KeyVersionPosture);
    }
    if existing.authenticity_requirement() != candidate.authenticity_requirement() {
        return Some(ScopeMismatchCase::AuthenticityRequirement);
    }
    if existing.custody_posture() != candidate.custody_posture() {
        return Some(ScopeMismatchCase::CustodyPosture);
    }
    Some(ScopeMismatchCase::FullWitness)
}
