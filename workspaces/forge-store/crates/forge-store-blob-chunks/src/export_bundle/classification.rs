use super::chunk_bytes::BlobExportedChunkBytes;
use super::counters::BlobExportEvidenceCounts;
use super::evidence_bundle::BlobExportOfflineChunkDeclaration;
use super::manifest::BlobExportChunkManifestRow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BlobExportCanonicalClassification<'a> {
    exported_chunks: Vec<BlobExportedChunkBytes<'a>>,
    manifest_rows: Vec<BlobExportChunkManifestRow>,
    offline_declarations: Vec<BlobExportOfflineChunkDeclaration>,
    counts: BlobExportEvidenceCounts,
}

impl<'a> BlobExportCanonicalClassification<'a> {
    pub(crate) fn from_exported_chunks(
        exported_chunks: &[BlobExportedChunkBytes<'a>],
    ) -> BlobExportCanonicalClassification<'a> {
        let mut exported_chunks = exported_chunks.to_vec();
        exported_chunks.sort_by(|left, right| {
            left.leaf()
                .ordinal()
                .get()
                .cmp(&right.leaf().ordinal().get())
                .then_with(|| {
                    left.leaf()
                        .byte_range()
                        .start()
                        .cmp(&right.leaf().byte_range().start())
                })
                .then_with(|| {
                    left.leaf()
                        .identity()
                        .chunk_digest()
                        .as_str()
                        .cmp(right.leaf().identity().chunk_digest().as_str())
                })
        });
        let manifest_rows = exported_chunks
            .iter()
            .map(BlobExportChunkManifestRow::from_collected_chunk)
            .collect::<Vec<_>>();
        let offline_declarations = exported_chunks
            .iter()
            .map(BlobExportOfflineChunkDeclaration::from_collected_chunk)
            .collect::<Vec<_>>();
        let counts = BlobExportEvidenceCounts::new(
            exported_chunks.len() as u64,
            exported_chunks
                .iter()
                .map(|chunk| chunk.bytes().range().len())
                .sum(),
            manifest_rows.len() as u64,
            0,
        );
        Self {
            exported_chunks,
            manifest_rows,
            offline_declarations,
            counts,
        }
    }

    pub(crate) fn exported_chunks(&self) -> &[BlobExportedChunkBytes<'a>] {
        &self.exported_chunks
    }

    pub(crate) fn manifest_rows(&self) -> &[BlobExportChunkManifestRow] {
        &self.manifest_rows
    }

    pub(crate) fn offline_declarations(&self) -> &[BlobExportOfflineChunkDeclaration] {
        &self.offline_declarations
    }

    pub(crate) const fn counts(&self) -> BlobExportEvidenceCounts {
        self.counts
    }
}
