use crate::{BlobStreamingReadAdmission, BlobStreamingReadCounterSnapshot, BlobStreamingReadDenial};

pub(crate) fn require_stable_read_bytes(
    admission: BlobStreamingReadAdmission,
    expected: u64,
    counters: BlobStreamingReadCounterSnapshot,
) -> Result<(), BlobStreamingReadDenial> {
    let actual = admission.stable_read().counters().guarded_bytes();
    if actual < expected {
        return Err(BlobStreamingReadDenial::StableReadBytesInsufficient {
            expected,
            actual,
            counters: counters.record_stale_read_denial(),
        });
    }
    Ok(())
}