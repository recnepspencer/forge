use forge_store_blob_chunks::BlobExportPublishedBundle;
use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_layout_indexes::observation::AccessShape;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportLayoutEvidenceReport {
    family_id: DurableArtifactFamilyId,
    access_shape: AccessShape,
    declared_chunks: u64,
}

impl ExportLayoutEvidenceReport {
    pub fn from_blob_export_bundle(bundle: &BlobExportPublishedBundle) -> Self {
        Self::from_export_source(bundle.offline_declarations().len() as u64)
    }

    fn from_export_source(declared_chunks: u64) -> Self {
        Self {
            family_id: DurableArtifactFamilyId::ExportBundle,
            access_shape: AccessShape::ManifestGraphWalk,
            declared_chunks,
        }
    }

    pub const fn family_id(&self) -> DurableArtifactFamilyId {
        self.family_id
    }

    pub const fn declared_access_shape(&self) -> AccessShape {
        self.access_shape
    }

    pub const fn declared_chunks(&self) -> u64 {
        self.declared_chunks
    }

    pub const fn cannot_be_foreground_authority(&self) -> bool {
        true
    }
}
