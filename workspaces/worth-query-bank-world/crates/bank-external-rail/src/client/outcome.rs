//! Typed outcomes a caller can observe from one dispatch exchange with the
//! external rail.
//!
//! Every variant here is what the *caller* can honestly know. `Completed`
//! never appears unless the rail actually wrote that frame; the rail's own
//! ledger may know more than the caller ever received, which is the entire
//! point of the indeterminate-response faults.

use crate::protocol::notice::RailRejection;

/// What a caller observed from one `Dispatch` exchange.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RailExchangeOutcome {
    /// The rail decoded the payload and refused it. Determinate: the effect
    /// did not happen and the rail admitted nothing.
    Rejected(RailRejection),
    /// The rail acknowledged and then completed the attempt.
    Completed,
    /// The rail acknowledged the attempt; no completion frame arrived within
    /// the caller's read attempts.
    Acknowledged,
    /// The rail acknowledged, then sent an explicit duplicate
    /// acknowledgement; no completion frame arrived.
    DuplicateAcknowledgement,
    /// The connection closed before any frame was read.
    Disconnected,
    /// The caller's deadline elapsed while waiting for the next frame.
    TimedOut,
}
