mod dedupe_case;
mod scope_mismatch;

pub(crate) use dedupe_case::{classify_dedupe_case, DedupeCase};
pub(crate) use scope_mismatch::{classify_scope_mismatch, ScopeMismatchCase};