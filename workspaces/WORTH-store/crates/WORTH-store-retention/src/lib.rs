#![forbid(unsafe_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetentionDisposition {
    Retain,
    Compact,
    Reclaim,
    Expire,
}
