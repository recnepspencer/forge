use forge_store_blob_chunks::ReadmittedBlobImport;
use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_layout_indexes::access_planning::S8AccessShape;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportLayoutEvidenceReport {
    family_id: DurableArtifactFamilyId,
    access_shape: S8AccessShape,
    declared_chunks: u64,
    local_chunks: u64,
}

impl ImportLayoutEvidenceReport {
    pub fn from_readmitted_blob_import(import: &ReadmittedBlobImport<'_>) -> Self {
        Self::from_import_source(import.declared_chunks(), import.local_chunks())
    }

    fn from_import_source(declared_chunks: u64, local_chunks: u64) -> Self {
        Self {
            family_id: DurableArtifactFamilyId::ImportBundle,
            access_shape: S8AccessShape::PointLookup,
            declared_chunks,
            local_chunks,
        }
    }

    pub const fn family_id(&self) -> DurableArtifactFamilyId {
        self.family_id
    }

    pub const fn declared_access_shape(&self) -> S8AccessShape {
        self.access_shape
    }

    pub const fn declared_chunks(&self) -> u64 {
        self.declared_chunks
    }

    pub const fn local_chunks(&self) -> u64 {
        self.local_chunks
    }

    pub const fn cannot_be_foreground_authority(&self) -> bool {
        true
    }
}
