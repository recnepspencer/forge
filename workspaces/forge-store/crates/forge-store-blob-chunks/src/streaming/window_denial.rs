#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobChunkStreamingDenial {
    EmptyStreamingWindow,
    WindowDigestMismatch,
    WholeObjectResidencyRequired,
}