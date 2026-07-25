use sha2::{Digest, Sha256};
use worth_store_physical_format::RecordFrameCoordinate;

use super::PhysicalWorkIntent;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalWorkConcurrencyScope {
    digest: [u8; 32],
    security: worth_store_security::StoreSecurityScopeIdentity,
    coordinates: Box<[RecordFrameCoordinate]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkConcurrencyRelation {
    DisjointArtifacts,
    SameArtifactDisjointRanges,
    Overlapping,
}

impl PhysicalWorkConcurrencyScope {
    pub(in crate::physical_runtime::work) fn derive(intent: &PhysicalWorkIntent) -> Self {
        let mut digest = Sha256::new();
        digest.update(b"worth-store.physical-concurrency-scope.v1");
        digest.update(intent.identity().store().bytes());
        digest.update(intent.security().stable_fingerprint());
        digest.update(intent.scope().stable_digest());
        Self {
            digest: digest.finalize().into(),
            security: intent.security(),
            coordinates: intent.scope().coordinates().into(),
        }
    }

    pub const fn digest(&self) -> &[u8; 32] {
        &self.digest
    }

    pub const fn security(&self) -> worth_store_security::StoreSecurityScopeIdentity {
        self.security
    }

    pub fn coordinates(&self) -> &[RecordFrameCoordinate] {
        &self.coordinates
    }

    pub fn relation(&self, other: &Self) -> PhysicalWorkConcurrencyRelation {
        let mut same_artifact = false;
        for left in &self.coordinates {
            for right in &other.coordinates {
                if left.artifact() != right.artifact() {
                    continue;
                }
                same_artifact = true;
                let left_end = left.offset().saturating_add(u64::from(left.length()));
                let right_end = right.offset().saturating_add(u64::from(right.length()));
                if left.offset() < right_end && right.offset() < left_end {
                    return PhysicalWorkConcurrencyRelation::Overlapping;
                }
            }
        }
        if same_artifact {
            PhysicalWorkConcurrencyRelation::SameArtifactDisjointRanges
        } else {
            PhysicalWorkConcurrencyRelation::DisjointArtifacts
        }
    }
}
