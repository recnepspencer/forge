use worth_store_physical_certification::S7CloseoutSourceDenial;

use super::S7CloseoutShortcutInput;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S7CloseoutShortcutAttempt {
    CopiedReceipt,
    CopiedChunkRows,
    CopiedProofId,
    S6PlacementReadinessOnly,
    S5FutureChunkPlaceholderOnly,
    TerminalProjectionOnly,
    RawCountersOnly,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S7CloseoutShortcutRejectionReport {
    attempt: S7CloseoutShortcutAttempt,
    reason: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum S7CloseoutDenial {
    SourceDenied(S7CloseoutSourceDenial),
    ProofTopologyNotChecked,
    CounterBackedFoundationalPolicyRequired,
    MissingChunkTreeIdentityBinding,
    MissingDigestBinding,
    MissingReachabilityBinding,
    MissingPlacementBinding,
    MissingSecurityScopeBinding,
    ShortcutRejected(S7CloseoutShortcutRejectionReport),
}

impl From<S7CloseoutSourceDenial> for S7CloseoutDenial {
    fn from(denial: S7CloseoutSourceDenial) -> Self {
        Self::SourceDenied(denial)
    }
}

impl S7CloseoutShortcutRejectionReport {
    const fn new(attempt: S7CloseoutShortcutAttempt, reason: &'static str) -> Self {
        Self { attempt, reason }
    }

    pub const fn attempt(&self) -> S7CloseoutShortcutAttempt { self.attempt }
    pub const fn reason(&self) -> &'static str { self.reason }
}

pub(crate) fn shortcut_rejection(
    shortcut: &S7CloseoutShortcutInput,
) -> S7CloseoutShortcutRejectionReport {
    match shortcut {
        S7CloseoutShortcutInput::CopiedReceipt => S7CloseoutShortcutRejectionReport::new(
            S7CloseoutShortcutAttempt::CopiedReceipt,
            "copied receipts cannot mint S.7 closeout",
        ),
        S7CloseoutShortcutInput::CopiedChunkRows { .. } => S7CloseoutShortcutRejectionReport::new(
            S7CloseoutShortcutAttempt::CopiedChunkRows,
            "copied chunk rows cannot mint S.7 closeout",
        ),
        S7CloseoutShortcutInput::CopiedProofId { .. } => S7CloseoutShortcutRejectionReport::new(
            S7CloseoutShortcutAttempt::CopiedProofId,
            "copied proof ids cannot mint S.7 closeout",
        ),
        S7CloseoutShortcutInput::S6PlacementReadinessOnly { .. } => {
            S7CloseoutShortcutRejectionReport::new(
                S7CloseoutShortcutAttempt::S6PlacementReadinessOnly,
                "S.6 placement readiness alone cannot mint S.7 closeout",
            )
        }
        S7CloseoutShortcutInput::S5FutureChunkPlaceholderOnly { .. } => {
            S7CloseoutShortcutRejectionReport::new(
                S7CloseoutShortcutAttempt::S5FutureChunkPlaceholderOnly,
                "S.5 future chunk placeholders alone cannot mint S.7 closeout",
            )
        }
        S7CloseoutShortcutInput::TerminalProjectionOnly => S7CloseoutShortcutRejectionReport::new(
            S7CloseoutShortcutAttempt::TerminalProjectionOnly,
            "terminal projections cannot mint S.7 closeout",
        ),
        S7CloseoutShortcutInput::RawCountersOnly { .. } => S7CloseoutShortcutRejectionReport::new(
            S7CloseoutShortcutAttempt::RawCountersOnly,
            "raw counters cannot mint S.7 closeout",
        ),
    }
}
