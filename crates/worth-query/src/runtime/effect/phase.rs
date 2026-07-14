#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryEffectPhase {
    TruthRead,
    Derive,
    EffectDelivery,
    PendingWriteIntent,
    Suppressed,
    ExpressionFailure,
}

impl WorthQueryEffectPhase {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TruthRead => "truth-read",
            Self::Derive => "derive",
            Self::EffectDelivery => "effect-delivery",
            Self::PendingWriteIntent => "pending-write-intent",
            Self::Suppressed => "suppressed",
            Self::ExpressionFailure => "expression-failure",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryEffectLoopPrevention {
    SingleCommitBoundary,
    PendingIntentExecutionDeferred,
    TerminalSuppression,
    TerminalExpressionFailure,
}

impl WorthQueryEffectLoopPrevention {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SingleCommitBoundary => "single-commit-boundary",
            Self::PendingIntentExecutionDeferred => "pending-intent-execution-deferred",
            Self::TerminalSuppression => "terminal-suppression",
            Self::TerminalExpressionFailure => "terminal-expression-failure",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryEffectIdempotence {
    DeliveryReceiptIdentity,
    PendingIntentReceiptIdentity,
    NoMutationRecorded,
}

impl WorthQueryEffectIdempotence {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DeliveryReceiptIdentity => "delivery-receipt-identity",
            Self::PendingIntentReceiptIdentity => "pending-intent-receipt-identity",
            Self::NoMutationRecorded => "no-mutation-recorded",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryEffectPhaseEvidence {
    phase_path: WorthQueryEffectPhasePath,
    loop_prevention: WorthQueryEffectLoopPrevention,
    idempotence: WorthQueryEffectIdempotence,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WorthQueryEffectPhasePath {
    Delivery,
    PendingWriteIntent,
    Suppressed,
    ExpressionFailure,
}

impl WorthQueryEffectPhaseEvidence {
    pub(in crate::runtime::effect) fn delivery() -> Self {
        Self {
            phase_path: WorthQueryEffectPhasePath::Delivery,
            loop_prevention: WorthQueryEffectLoopPrevention::SingleCommitBoundary,
            idempotence: WorthQueryEffectIdempotence::DeliveryReceiptIdentity,
        }
    }

    pub(in crate::runtime::effect) fn pending_write_intent() -> Self {
        Self {
            phase_path: WorthQueryEffectPhasePath::PendingWriteIntent,
            loop_prevention: WorthQueryEffectLoopPrevention::PendingIntentExecutionDeferred,
            idempotence: WorthQueryEffectIdempotence::PendingIntentReceiptIdentity,
        }
    }

    pub(in crate::runtime::effect) fn suppressed() -> Self {
        Self {
            phase_path: WorthQueryEffectPhasePath::Suppressed,
            loop_prevention: WorthQueryEffectLoopPrevention::TerminalSuppression,
            idempotence: WorthQueryEffectIdempotence::NoMutationRecorded,
        }
    }

    pub(in crate::runtime::effect) fn expression_failure() -> Self {
        Self {
            phase_path: WorthQueryEffectPhasePath::ExpressionFailure,
            loop_prevention: WorthQueryEffectLoopPrevention::TerminalExpressionFailure,
            idempotence: WorthQueryEffectIdempotence::NoMutationRecorded,
        }
    }

    pub fn phases(&self) -> &[WorthQueryEffectPhase] {
        match self.phase_path {
            WorthQueryEffectPhasePath::Delivery => &[
                WorthQueryEffectPhase::TruthRead,
                WorthQueryEffectPhase::Derive,
                WorthQueryEffectPhase::EffectDelivery,
            ],
            WorthQueryEffectPhasePath::PendingWriteIntent => &[
                WorthQueryEffectPhase::TruthRead,
                WorthQueryEffectPhase::Derive,
                WorthQueryEffectPhase::PendingWriteIntent,
            ],
            WorthQueryEffectPhasePath::Suppressed => &[
                WorthQueryEffectPhase::TruthRead,
                WorthQueryEffectPhase::Derive,
                WorthQueryEffectPhase::Suppressed,
            ],
            WorthQueryEffectPhasePath::ExpressionFailure => &[
                WorthQueryEffectPhase::TruthRead,
                WorthQueryEffectPhase::Derive,
                WorthQueryEffectPhase::ExpressionFailure,
            ],
        }
    }

    pub fn loop_prevention(&self) -> WorthQueryEffectLoopPrevention {
        self.loop_prevention
    }

    pub fn idempotence(&self) -> WorthQueryEffectIdempotence {
        self.idempotence
    }
}
