use forge_store_authority::StoreCurrentAuthorityWitness;

use crate::{BlobChunkByteWindow, BlobChunkProofLeaf};

use super::chunk_evidence::BlobImportedChunkEvidence;
use super::counters::BlobImportReadmissionCounters;
use super::denial::BlobImportReadmissionDenial;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobImportReadmissionAuthority {
    current_authority: StoreCurrentAuthorityWitness,
}

impl BlobImportReadmissionAuthority {
    pub const fn from_current_store_authority(
        current_authority: StoreCurrentAuthorityWitness,
    ) -> Self {
        Self { current_authority }
    }

    pub fn collect_current_chunk_evidence<'a>(
        &self,
        leaf: &BlobChunkProofLeaf,
        bytes: BlobChunkByteWindow<'a>,
    ) -> Result<BlobImportedChunkEvidence<'a>, BlobImportReadmissionDenial> {
        let _physical = self.current_authority.current_physical_authority();
        BlobImportedChunkEvidence::collect_from_leaf(
            leaf,
            bytes,
            BlobImportReadmissionCounters::start(),
        )
    }

    pub(super) const fn current_authority(&self) -> &StoreCurrentAuthorityWitness {
        &self.current_authority
    }
}
