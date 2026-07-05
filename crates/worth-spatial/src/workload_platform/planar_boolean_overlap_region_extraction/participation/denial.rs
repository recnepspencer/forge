use super::counters::PlanarBooleanOverlapParticipationRecoveryCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanOverlapParticipationRecoveryDenialKind {
    LoopLedgerParticipationSupportMismatch,
    DanglingLoopParticipationDenied,
    ContradictoryIslandMembershipDenied,
    ForeignOverlapChainLineageDenied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapParticipationRecoveryDenial {
    kind: PlanarBooleanOverlapParticipationRecoveryDenialKind,
    rejected_identity: String,
    counters: PlanarBooleanOverlapParticipationRecoveryCounters,
    human_reason: &'static str,
}

impl PlanarBooleanOverlapParticipationRecoveryDenial {
    pub(crate) fn new(
        kind: PlanarBooleanOverlapParticipationRecoveryDenialKind,
        rejected_identity: impl Into<String>,
        counters: PlanarBooleanOverlapParticipationRecoveryCounters,
        human_reason: &'static str,
    ) -> Self {
        Self {
            kind,
            rejected_identity: rejected_identity.into(),
            counters,
            human_reason,
        }
    }

    pub fn kind(&self) -> PlanarBooleanOverlapParticipationRecoveryDenialKind {
        self.kind
    }

    pub fn rejected_identity(&self) -> &str {
        &self.rejected_identity
    }

    pub fn counters(&self) -> PlanarBooleanOverlapParticipationRecoveryCounters {
        self.counters
    }

    pub fn human_reason(&self) -> &'static str {
        self.human_reason
    }
}
