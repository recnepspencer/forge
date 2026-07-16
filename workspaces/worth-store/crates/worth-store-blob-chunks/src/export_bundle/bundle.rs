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

pub(crate) struct BlobExportPublishedBundleParts {
    pub(crate) object_id: BlobObjectId,
    pub(crate) generation: BlobGeneration,
    pub(crate) chunk_tree_root: ChunkTreeRoot,
    pub(crate) security_metadata: BlobChunkSecurityMetadataWitness,
    pub(crate) manifest: BlobExportManifest,
    pub(crate) custody: BlobExportCustodyEvidence,
    pub(crate) digest_evidence: BlobExportDigestEvidence,
    pub(crate) offline_declarations: Vec<BlobExportOfflineChunkDeclaration>,
    pub(crate) canonical_export: CanonicalExportReadyArtifact,
    pub(crate) counters: BlobExportBundleCounters,
}

impl BlobExportPublishedBundle {
    pub(crate) fn new(parts: BlobExportPublishedBundleParts) -> Self {
        Self {
            object_id: parts.object_id,
            generation: parts.generation,
            chunk_tree_root: parts.chunk_tree_root,
            security_metadata: parts.security_metadata,
            manifest: parts.manifest,
            custody: parts.custody,
            digest_evidence: parts.digest_evidence,
            offline_declarations: parts.offline_declarations,
            canonical_export: parts.canonical_export,
            counters: parts.counters,
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
