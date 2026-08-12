use worth_store_wal::WalLsnRange;

use super::WalPrefixAdmissionDenial;

pub(crate) fn require_contiguous_prefix(
    frontier: u64,
    ranges: impl IntoIterator<Item = WalLsnRange>,
) -> Result<(), WalPrefixAdmissionDenial> {
    let mut expected = frontier;
    for range in ranges {
        if range.start().get() != expected {
            return Err(if expected == frontier {
                WalPrefixAdmissionDenial::FrontierMismatch
            } else {
                WalPrefixAdmissionDenial::Gap
            });
        }
        expected = range.end_exclusive().get();
    }
    Ok(())
}
