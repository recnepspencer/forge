//! Read-side corruption observation: classify damage before decode, seal quarantine, deny read.
pub(crate) use crate::streaming::read::transitions::observe_corruption_damage::observe_and_deny_streaming_corruption;

pub(crate) fn seal_and_deny_corruption(
    request: &crate::BlobStreamingReadRequest,
    quarantine_authority: &mut Option<crate::BlobQuarantineAuthority>,
    expected: &crate::BlobChunkProofLeaf,
    actual: &crate::BlobStreamingReadObservedChunk,
    counters: &mut crate::BlobStreamingReadCounterSnapshot,
) -> Result<(), crate::BlobStreamingReadDenial> {
    observe_and_deny_streaming_corruption(request, quarantine_authority, expected, actual, counters)
}
