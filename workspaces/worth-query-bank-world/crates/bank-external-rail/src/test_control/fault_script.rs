//! Fault postures owned by the rail's separate test-control plane.

use serde::{Deserialize, Serialize};

/// The behavior the external rail must exhibit for newly admitted dispatches.
///
/// Every variant except [`FaultScript::Succeed`] is one of the Gate 8.2 exit
/// proof faults. None of them ever reports a successful completion while
/// active.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FaultScript {
    /// No fault: acknowledge, then complete normally.
    Succeed,
    /// Commit the effect, then close the connection without a response.
    CommitThenLoseResponse,
    /// Acknowledge receipt, then close without completing.
    AcknowledgeWithoutCompleting,
    /// Complete after the caller's configured deadline can expire.
    CompleteAfterDelay { delay_millis: u64 },
    /// Acknowledge twice without completing.
    DuplicateAcknowledgement,
    /// Close before writing a response or admitting a ledger record.
    DisappearMidDispatch,
}
