use super::BlobCloseoutSourceDenial;

use super::BlobCloseoutShortcutInput;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobCloseoutShortcutAttempt {
    CopiedReceipt,
    CopiedChunkRows,
    CopiedProofId,
    FutureChunkPlaceholderOnly,
    TerminalProjectionOnly,
    RawCountersOnly,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobCloseoutShortcutRejectionReport {
    attempt: BlobCloseoutShortcutAttempt,
    reason: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlobCloseoutDenial {
    SourceDenied(BlobCloseoutSourceDenial),
    ProofTopologyNotChecked,
    CounterBackedFoundationalPolicyRequired,
    MissingChunkTreeIdentityBinding,
    MissingDigestBinding,
    MissingReachabilityBinding,
    MissingPlacementBinding,
    MissingSecurityScopeBinding,
    ShortcutRejected(BlobCloseoutShortcutRejectionReport),
}

impl From<BlobCloseoutSourceDenial> for BlobCloseoutDenial {
    fn from(denial: BlobCloseoutSourceDenial) -> Self {
        Self::SourceDenied(denial)
    }
}

impl BlobCloseoutShortcutRejectionReport {
    const fn new(attempt: BlobCloseoutShortcutAttempt, reason: &'static str) -> Self {
        Self { attempt, reason }
    }

    pub const fn attempt(&self) -> BlobCloseoutShortcutAttempt {
        self.attempt
    }
    pub const fn reason(&self) -> &'static str {
        self.reason
    }
}

pub(crate) fn shortcut_rejection(
    shortcut: &BlobCloseoutShortcutInput,
) -> BlobCloseoutShortcutRejectionReport {
    match shortcut {
        BlobCloseoutShortcutInput::CopiedReceipt => BlobCloseoutShortcutRejectionReport::new(
            BlobCloseoutShortcutAttempt::CopiedReceipt,
            "copied receipts cannot mint blob closeout",
        ),
        BlobCloseoutShortcutInput::CopiedChunkRows { .. } => {
            BlobCloseoutShortcutRejectionReport::new(
                BlobCloseoutShortcutAttempt::CopiedChunkRows,
                "copied chunk rows cannot mint blob closeout",
            )
        }
        BlobCloseoutShortcutInput::CopiedProofId { .. } => {
            BlobCloseoutShortcutRejectionReport::new(
                BlobCloseoutShortcutAttempt::CopiedProofId,
                "copied proof ids cannot mint blob closeout",
            )
        }
        BlobCloseoutShortcutInput::FutureChunkPlaceholderOnly { .. } => {
            BlobCloseoutShortcutRejectionReport::new(
                BlobCloseoutShortcutAttempt::FutureChunkPlaceholderOnly,
                "future chunk placeholders alone cannot mint blob closeout",
            )
        }
        BlobCloseoutShortcutInput::TerminalProjectionOnly => {
            BlobCloseoutShortcutRejectionReport::new(
                BlobCloseoutShortcutAttempt::TerminalProjectionOnly,
                "terminal projections cannot mint blob closeout",
            )
        }
        BlobCloseoutShortcutInput::RawCountersOnly { .. } => {
            BlobCloseoutShortcutRejectionReport::new(
                BlobCloseoutShortcutAttempt::RawCountersOnly,
                "raw counters cannot mint blob closeout",
            )
        }
    }
}
