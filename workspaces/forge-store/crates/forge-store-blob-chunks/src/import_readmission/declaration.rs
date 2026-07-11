use forge_store_operations_vocabulary::ImportPlacementSource;
use forge_store_security::{
    StoreCustodyPosture, StoreRawSecurityScopeDeclaration, StoreSecurityScopeIdentity,
};

use crate::{
    BlobChunkSecurityMetadataWitness, BlobExportPublishedBundle, BlobGeneration, BlobObjectId,
    ChunkTreeRoot, LogicalContentDigest,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobImportChunkDeclaration {
    ordinal: u64,
    chunk_identity: String,
    stored_digest: String,
    checksum_digest: String,
    bytes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobImportDeclaration {
    object_id: BlobObjectId,
    generation: BlobGeneration,
    chunk_tree_root: ChunkTreeRoot,
    logical_content_digest: LogicalContentDigest,
    chunk_scope: StoreRawSecurityScopeDeclaration,
    placement_source: ImportPlacementSource,
    export_custody_scope: StoreSecurityScopeIdentity,
    chunk_rows: Vec<BlobImportChunkDeclaration>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundaryBridgedCanonicalExportArtifact {
    declaration: BlobImportDeclaration,
}

impl BlobImportChunkDeclaration {
    pub fn portable(
        ordinal: u64,
        chunk_identity: impl Into<String>,
        stored_digest: impl Into<String>,
        checksum_digest: impl Into<String>,
        bytes: u64,
    ) -> Self {
        Self {
            ordinal,
            chunk_identity: chunk_identity.into(),
            stored_digest: stored_digest.into(),
            checksum_digest: checksum_digest.into(),
            bytes,
        }
    }
}

impl BlobImportDeclaration {
    pub fn portable(
        object_id: BlobObjectId,
        generation: BlobGeneration,
        chunk_tree_root: ChunkTreeRoot,
        logical_content_digest: LogicalContentDigest,
        chunk_scope: StoreRawSecurityScopeDeclaration,
        placement_source: ImportPlacementSource,
        export_custody_scope: StoreSecurityScopeIdentity,
        chunk_rows: Vec<BlobImportChunkDeclaration>,
    ) -> Self {
        Self {
            object_id,
            generation,
            chunk_tree_root,
            logical_content_digest,
            chunk_scope,
            placement_source,
            export_custody_scope,
            chunk_rows,
        }
    }
}

pub fn bridge_canonical_export_trust_boundary(
    bundle: &BlobExportPublishedBundle,
) -> BoundaryBridgedCanonicalExportArtifact {
    BoundaryBridgedCanonicalExportArtifact::from_declaration(BlobImportDeclaration::portable(
        bundle.object_id().clone(),
        bundle.generation(),
        bundle.chunk_tree_root().clone(),
        bundle.digest_evidence().logical_content_digest().clone(),
        lower_chunk_scope(bundle.security_metadata()),
        ImportPlacementSource::InlineInBundle,
        bundle.custody().identity(),
        bundle
            .offline_declarations()
            .iter()
            .map(|row| {
                BlobImportChunkDeclaration::portable(
                    row.ordinal(),
                    row.chunk_identity(),
                    row.stored_digest(),
                    row.checksum_digest(),
                    row.bytes(),
                )
            })
            .collect(),
    ))
}

impl BoundaryBridgedCanonicalExportArtifact {
    pub fn declaration(&self) -> &BlobImportDeclaration {
        &self.declaration
    }

    pub fn into_declaration(self) -> BlobImportDeclaration {
        self.declaration
    }

    pub fn from_declaration(declaration: BlobImportDeclaration) -> Self {
        Self { declaration }
    }
}

impl BlobImportDeclaration {
    pub fn chunk_scope(&self) -> StoreRawSecurityScopeDeclaration {
        self.chunk_scope
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

    pub fn logical_content_digest(&self) -> &LogicalContentDigest {
        &self.logical_content_digest
    }

    pub fn export_custody_scope(&self) -> StoreSecurityScopeIdentity {
        self.export_custody_scope
    }

    pub fn chunk_rows(&self) -> &[BlobImportChunkDeclaration] {
        &self.chunk_rows
    }

    pub const fn placement_source(&self) -> ImportPlacementSource {
        self.placement_source
    }

    pub fn with_chunk_scope(mut self, chunk_scope: StoreRawSecurityScopeDeclaration) -> Self {
        self.chunk_scope = chunk_scope;
        self
    }

    pub fn with_chunk_rows(mut self, chunk_rows: Vec<BlobImportChunkDeclaration>) -> Self {
        self.chunk_rows = chunk_rows;
        self
    }

    pub fn with_placement_source(mut self, placement_source: ImportPlacementSource) -> Self {
        self.placement_source = placement_source;
        self
    }

    pub fn with_export_custody_scope(
        mut self,
        export_custody_scope: StoreSecurityScopeIdentity,
    ) -> Self {
        self.export_custody_scope = export_custody_scope;
        self
    }
}

impl BlobImportChunkDeclaration {
    pub const fn ordinal(&self) -> u64 {
        self.ordinal
    }

    pub fn chunk_identity(&self) -> &str {
        &self.chunk_identity
    }

    pub fn stored_digest(&self) -> &str {
        &self.stored_digest
    }

    pub fn checksum_digest(&self) -> &str {
        &self.checksum_digest
    }

    pub const fn bytes(&self) -> u64 {
        self.bytes
    }
}

fn lower_chunk_scope(
    metadata: BlobChunkSecurityMetadataWitness,
) -> StoreRawSecurityScopeDeclaration {
    StoreRawSecurityScopeDeclaration::deserialized_unadmitted(
        metadata.identity().physical_witness(),
        metadata.key_scope(),
        metadata.key_version_posture(),
        metadata.tenant_scope(),
        Some(metadata.authenticity_requirement()),
        Some(StoreCustodyPosture::ImportedUnreadmitted),
    )
}
