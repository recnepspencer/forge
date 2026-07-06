use forge_store_contracts::StableDigest;

use crate::{BlobChunkStreamingResidencyProof, BlobChunkStreamingWindow};

pub(crate) fn streaming_window() -> BlobChunkStreamingWindow {
    streaming_window_with_digest("sha256:blob-s51-same-content")
}

pub(crate) fn streaming_window_with_digest(raw: &str) -> BlobChunkStreamingWindow {
    let digest = StableDigest::new(raw).expect("digest");
    let residency =
        BlobChunkStreamingResidencyProof::bounded_window(4096, 1024).expect("bounded window");
    BlobChunkStreamingWindow::new(
        crate::BlobChunkIdentity::from_integrity_parts(digest.clone()),
        digest,
        residency,
    )
    .expect("bounded window should admit")
}