use super::{BlobObjectLayoutReport, ChunkTreeLayoutReport, StreamingLayoutReport};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobLayoutCloseout {
    blob_object: BlobObjectLayoutReport,
    chunk_tree: ChunkTreeLayoutReport,
    streaming: StreamingLayoutReport,
}

impl BlobLayoutCloseout {
    pub const fn new(
        blob_object: BlobObjectLayoutReport,
        chunk_tree: ChunkTreeLayoutReport,
        streaming: StreamingLayoutReport,
    ) -> Self {
        Self {
            blob_object,
            chunk_tree,
            streaming,
        }
    }

    pub const fn blob_object(&self) -> &BlobObjectLayoutReport {
        &self.blob_object
    }

    pub const fn chunk_tree(&self) -> &ChunkTreeLayoutReport {
        &self.chunk_tree
    }

    pub const fn streaming(&self) -> &StreamingLayoutReport {
        &self.streaming
    }
}
