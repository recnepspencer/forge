use worth_foundational::CanonicalExportReadyArtifact;

use crate::{BlobChunkSecurityMetadataWitness, BlobGeneration, BlobObjectId, ChunkTreeRoot};

use super::counters::BlobExportBundleCounters;
use super::custody_receipt::BlobExportCustodyEvidence;
use super::evidence_bundle::{BlobExportDigestEvidence, BlobExportOfflineChunkDeclaration};
use super::manifest::BlobExportManifest;

pub struct BlobExportPublishedBundle {
    object_id: BlobObjectId,
    generation: BlobGeneration,
    chunk_tree_root: ChunkTreeRoot,
    security_metadata: BlobChunkSecurityMetadataWitness,
    manifest: BlobExportManifest,
    custody: BlobExportCustodyEvidence,
    digest_evidence: BlobExportDigestEvidence,
    offline_declarations: Vec<BlobExportOfflineChunkDeclaration>,
    canonical_export: CanonicalExportReadyArtifact,
    counters: BlobExportBundleCounters,
}

impl BlobExportPublishedBundle {
    pub(crate) fn new(
        object_id: BlobObjectId,
        generation: BlobGeneration,
        chunk_tree_root: ChunkTreeRoot,
        security_metadata: BlobChunkSecurityMetadataWitness,
        manifest: BlobExportManifest,
        custody: BlobExportCustodyEvidence,
        digest_evidence: BlobExportDigestEvidence,
        offline_declarations: Vec<BlobExportOfflineChunkDeclaration>,
        canonical_export: CanonicalExportReadyArtifact,
        counters: BlobExportBundleCounters,
    ) -> Self {
        Self {
            object_id,
            generation,
            chunk_tree_root,
            security_metadata,
            manifest,
            custody,
            digest_evidence,
            offline_declarations,
            canonical_export,
            counters,
        }
    }

    pub fn object_id(&self) -> &BlobObjectId {
        &self.object_id
    }

    pub const fn generation(&self) -> BlobGeneration {
        self.generation
    }

    pub fn chunk_tree_root(&self) -> &ChunkTreeRoot {
        &self.chunk_tree_root
    }

    pub const fn security_metadata(&self) -> BlobChunkSecurityMetadataWitness {
        self.security_metadata
    }

    pub fn manifest(&self) -> &BlobExportManifest {
        &self.manifest
    }

    pub fn custody(&self) -> &BlobExportCustodyEvidence {
        &self.custody
    }

    pub fn digest_evidence(&self) -> &BlobExportDigestEvidence {
        &self.digest_evidence
    }

    pub fn offline_declarations(&self) -> &[BlobExportOfflineChunkDeclaration] {
        &self.offline_declarations
    }

    pub fn canonical_export(&self) -> &CanonicalExportReadyArtifact {
        &self.canonical_export
    }

    pub const fn counters(&self) -> BlobExportBundleCounters {
        self.counters
    }
}
