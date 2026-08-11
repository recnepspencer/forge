use worth_store_wal::InterruptedWalTail;

use super::WalPrefixAdmissionDenial;

pub(crate) fn classify_terminal_interruption(
    index: usize,
    segment_count: usize,
    interruption: Option<InterruptedWalTail>,
) -> Result<Option<InterruptedWalTail>, WalPrefixAdmissionDenial> {
    if interruption.is_some() && index + 1 != segment_count {
        Err(WalPrefixAdmissionDenial::InterruptedMiddle)
    } else {
        Ok(interruption)
    }
}
