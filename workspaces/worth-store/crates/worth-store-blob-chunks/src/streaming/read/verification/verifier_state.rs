use super::logical_content_digest::accumulator_seed;
use crate::{
    BlobQuarantineAuthority, BlobStreamingReadAdmission, BlobStreamingReadRequest,
    BlobStreamingReadWindow,
};

pub(crate) struct StreamingReadVerifier {
    pub(crate) _admission: BlobStreamingReadAdmission,
    pub(crate) request: BlobStreamingReadRequest,
    pub(crate) window: BlobStreamingReadWindow,
    pub(crate) quarantine_authority: Option<BlobQuarantineAuthority>,
    pub(crate) next_index: usize,
    pub(crate) logical_content_basis: u64,
}

impl StreamingReadVerifier {
    pub(crate) fn new(
        admission: BlobStreamingReadAdmission,
        request: BlobStreamingReadRequest,
        window: BlobStreamingReadWindow,
        quarantine_authority: BlobQuarantineAuthority,
    ) -> Self {
        Self {
            _admission: admission,
            request,
            window,
            quarantine_authority: Some(quarantine_authority),
            next_index: 0,
            logical_content_basis: accumulator_seed("logical-content"),
        }
    }

    pub(crate) fn advance_index(&mut self) {
        self.next_index += 1;
    }
}
