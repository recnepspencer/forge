use worth_store_physical_backend::StoreExternalPlacementRecoverabilityEvidence;
use worth_store_tiering::TierPlacementIoAdmission;

use crate::{BlobChunkReachabilityProofSet, BlobChunkSecurityMetadataWitness, StoredChunkDigest};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BlobPlacementReachabilityBasis {
    stored_digest: StoredChunkDigest,
    security_metadata: BlobChunkSecurityMetadataWitness,
}

impl BlobPlacementReachabilityBasis {
    pub(crate) fn from_reachability(reachability: &BlobChunkReachabilityProofSet) -> Self {
        Self {
            stored_digest: reachability.stored_digest().clone(),
            security_metadata: reachability.security_metadata(),
        }
    }

    pub(crate) fn matches_reachability(
        &self,
        reachability: &BlobChunkReachabilityProofSet,
    ) -> bool {
        self.stored_digest == *reachability.stored_digest()
            && self.security_metadata == reachability.security_metadata()
    }

    pub(crate) fn admits_external_recoverability(
        &self,
        evidence: &StoreExternalPlacementRecoverabilityEvidence,
    ) -> bool {
        evidence.matches_placement_manifest_basis(
            self.stored_digest.digest().as_str(),
            self.security_metadata.identity(),
        )
    }

    pub(crate) fn admits_readiness(&self, readiness: &TierPlacementIoAdmission) -> bool {
        readiness.cold_tier_posture().security_scope() == self.security_metadata.identity()
    }
}
