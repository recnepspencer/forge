mod continuity;
mod denial;
mod torn_tail;
mod valid_prefix;

pub(crate) use continuity::require_contiguous_prefix;
pub(crate) use denial::WalPrefixAdmissionDenial;
pub(crate) use torn_tail::classify_terminal_interruption;
pub(crate) use valid_prefix::WalValidPrefixFacts;
