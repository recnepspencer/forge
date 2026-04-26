#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryEffectPhase {
    TruthRead,
    Derive,
    EffectDelivery,
    PendingWriteIntent,
    Suppressed,
    ExpressionFailure,
}

impl ForgeQueryEffectPhase {
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
pub enum ForgeQueryEffectLoopPrevention {
    SingleCommitBoundary,
    PendingIntentExecutionDeferred,
    TerminalSuppression,
    TerminalExpressionFailure,
}

impl ForgeQueryEffectLoopPrevention {
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
pub enum ForgeQueryEffectIdempotence {
    DeliveryReceiptIdentity,
    PendingIntentReceiptIdentity,
    NoMutationRecorded,
}

impl ForgeQueryEffectIdempotence {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DeliveryReceiptIdentity => "delivery-receipt-identity",
            Self::PendingIntentReceiptIdentity => "pending-intent-receipt-identity",
            Self::NoMutationRecorded => "no-mutation-recorded",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryEffectPhaseEvidence {
    phase_path: ForgeQueryEffectPhasePath,
    loop_prevention: ForgeQueryEffectLoopPrevention,
    idempotence: ForgeQueryEffectIdempotence,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ForgeQueryEffectPhasePath {
    Delivery,
    PendingWriteIntent,
    Suppressed,
    ExpressionFailure,
}

impl ForgeQueryEffectPhaseEvidence {
    pub(in crate::runtime::effect) fn delivery() -> Self {
        Self {
            phase_path: ForgeQueryEffectPhasePath::Delivery,
            loop_prevention: ForgeQueryEffectLoopPrevention::SingleCommitBoundary,
            idempotence: ForgeQueryEffectIdempotence::DeliveryReceiptIdentity,
        }
    }

    pub(in crate::runtime::effect) fn pending_write_intent() -> Self {
        Self {
            phase_path: ForgeQueryEffectPhasePath::PendingWriteIntent,
            loop_prevention: ForgeQueryEffectLoopPrevention::PendingIntentExecutionDeferred,
            idempotence: ForgeQueryEffectIdempotence::PendingIntentReceiptIdentity,
        }
    }

    pub(in crate::runtime::effect) fn suppressed() -> Self {
        Self {
            phase_path: ForgeQueryEffectPhasePath::Suppressed,
            loop_prevention: ForgeQueryEffectLoopPrevention::TerminalSuppression,
            idempotence: ForgeQueryEffectIdempotence::NoMutationRecorded,
        }
    }

    pub(in crate::runtime::effect) fn expression_failure() -> Self {
        Self {
            phase_path: ForgeQueryEffectPhasePath::ExpressionFailure,
            loop_prevention: ForgeQueryEffectLoopPrevention::TerminalExpressionFailure,
            idempotence: ForgeQueryEffectIdempotence::NoMutationRecorded,
        }
    }

    pub fn phases(&self) -> &[ForgeQueryEffectPhase] {
        match self.phase_path {
            ForgeQueryEffectPhasePath::Delivery => &[
                ForgeQueryEffectPhase::TruthRead,
                ForgeQueryEffectPhase::Derive,
                ForgeQueryEffectPhase::EffectDelivery,
            ],
            ForgeQueryEffectPhasePath::PendingWriteIntent => &[
                ForgeQueryEffectPhase::TruthRead,
                ForgeQueryEffectPhase::Derive,
                ForgeQueryEffectPhase::PendingWriteIntent,
            ],
            ForgeQueryEffectPhasePath::Suppressed => &[
                ForgeQueryEffectPhase::TruthRead,
                ForgeQueryEffectPhase::Derive,
                ForgeQueryEffectPhase::Suppressed,
            ],
            ForgeQueryEffectPhasePath::ExpressionFailure => &[
                ForgeQueryEffectPhase::TruthRead,
                ForgeQueryEffectPhase::Derive,
                ForgeQueryEffectPhase::ExpressionFailure,
            ],
        }
    }

    pub fn loop_prevention(&self) -> ForgeQueryEffectLoopPrevention {
        self.loop_prevention
    }

    pub fn idempotence(&self) -> ForgeQueryEffectIdempotence {
        self.idempotence
    }
}
