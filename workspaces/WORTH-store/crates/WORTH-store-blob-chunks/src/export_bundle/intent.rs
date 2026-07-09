use worth_store_operations::BackupExportCustodyReadiness;

use crate::{AdmittedBlobPlacement, LifecycleReceipt};

use super::chunk_bytes::BlobExportedChunkBytes;

#[derive(Debug)]
pub struct BlobExportIntent<'a> {
    lifecycle: &'a LifecycleReceipt,
    publication: &'a crate::BlobChunkRootPublication,
    reachability: &'a crate::BlobChunkReachabilityProofSet,
    placement: &'a AdmittedBlobPlacement,
    custody: &'a BackupExportCustodyReadiness,
    export_name: String,
    exported_chunks: Vec<BlobExportedChunkBytes<'a>>,
}

impl<'a> BlobExportIntent<'a> {
    pub fn for_current_lifecycle(
        lifecycle: &'a LifecycleReceipt,
        publication: &'a crate::BlobChunkRootPublication,
        reachability: &'a crate::BlobChunkReachabilityProofSet,
        placement: &'a AdmittedBlobPlacement,
        custody: &'a BackupExportCustodyReadiness,
    ) -> Self {
        Self {
            lifecycle,
            publication,
            reachability,
            placement,
            custody,
            export_name: String::new(),
            exported_chunks: Vec::new(),
        }
    }

    pub fn with_export_name(mut self, export_name: impl Into<String>) -> Self {
        self.export_name = export_name.into();
        self
    }

    pub fn with_exported_chunks(
        mut self,
        chunks: impl IntoIterator<Item = BlobExportedChunkBytes<'a>>,
    ) -> Self {
        self.exported_chunks = chunks.into_iter().collect();
        self
    }

    pub(crate) fn lifecycle(&self) -> &LifecycleReceipt {
        self.lifecycle
    }

    pub(crate) fn publication(&self) -> &crate::BlobChunkRootPublication {
        self.publication
    }

    pub(crate) fn reachability(&self) -> &crate::BlobChunkReachabilityProofSet {
        self.reachability
    }

    pub(crate) fn placement(&self) -> &AdmittedBlobPlacement {
        self.placement
    }

    pub(crate) fn custody(&self) -> &BackupExportCustodyReadiness {
        self.custody
    }

    pub(crate) fn export_name(&self) -> &str {
        &self.export_name
    }

    pub(crate) fn exported_chunks(&self) -> &[BlobExportedChunkBytes<'a>] {
        &self.exported_chunks
    }
}
