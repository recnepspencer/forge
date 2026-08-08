//! Fault instructions a test controller may attach to one dispatch attempt.

use serde::{Deserialize, Serialize};

/// The behavior the external rail must exhibit for one dispatch attempt.
///
/// Every variant except [`FaultScript::Succeed`] is one of the Gate 8.2 exit
/// proof faults. None of them ever reports a successful completion while
/// active.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FaultScript {
    /// No fault: acknowledge, then complete normally.
    Succeed,
    /// Commit the effect on the rail's own ledger, then lose the response:
    /// the connection closes having written zero bytes.
    CommitThenLoseResponse,
    /// Acknowledge receipt, then never complete: the connection closes after
    /// the acknowledgement with no completion frame.
    AcknowledgeWithoutCompleting,
    /// Acknowledge immediately, then complete only after `delay_millis` has
    /// elapsed, so a caller holding a shorter deadline observes a timeout
    /// before the real completion arrives.
    CompleteAfterDelay { delay_millis: u64 },
    /// Acknowledge, then send a second, explicitly duplicate
    /// acknowledgement. The attempt never completes.
    DuplicateAcknowledgement,
    /// Disappear immediately upon receiving the request: the connection
    /// closes before any byte is written and before any ledger record is
    /// created.
    DisappearMidDispatch,
}
