use crate::physical_runtime::{
    PhysicalEffectIdentity, PhysicalRecordMutationFailureEvidence, PhysicalWorkEffectFate,
    PhysicalWorkIdentity, PhysicalWorkRecoveryDisposition, RecordPublicationStage,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordPublicationWorkSettlement {
    effect: Option<PhysicalEffectIdentity>,
    effect_fate: PhysicalWorkEffectFate,
    recovery: PhysicalWorkRecoveryDisposition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordPublicationWorkEffect {
    stage: RecordPublicationStage,
    identity: PhysicalWorkIdentity,
    settlement: Option<RecordPublicationWorkSettlement>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RecordPublicationWorkTrace {
    effects: Vec<RecordPublicationWorkEffect>,
}

impl RecordPublicationWorkTrace {
    pub fn effects(&self) -> &[RecordPublicationWorkEffect] {
        &self.effects
    }

    pub const fn effect_count(&self) -> usize {
        self.effects.len()
    }

    pub(in crate::physical_runtime::record_serving) fn record_settled(
        &mut self,
        stage: RecordPublicationStage,
        identity: PhysicalWorkIdentity,
        settlement: RecordPublicationWorkSettlement,
    ) {
        self.effects.push(RecordPublicationWorkEffect {
            stage,
            identity,
            settlement: Some(settlement),
        });
    }

    pub(in crate::physical_runtime::record_serving) fn extend(&mut self, other: Self) {
        self.effects.extend(other.effects);
    }

    pub(in crate::physical_runtime::record_serving) fn including(
        mut self,
        stage: RecordPublicationStage,
        failure: PhysicalRecordMutationFailureEvidence,
    ) -> Self {
        if let Some(identity) = failure.identity() {
            if !self
                .effects
                .iter()
                .any(|effect| effect.identity == identity)
            {
                let settlement = failure.recovery().map(|recovery| {
                    RecordPublicationWorkSettlement::from_canonical(
                        failure
                            .backend_operation()
                            .map(|backend| PhysicalEffectIdentity::new(identity, backend)),
                        failure.effect_fate(),
                        recovery,
                    )
                });
                self.effects.push(RecordPublicationWorkEffect {
                    stage,
                    identity,
                    settlement,
                });
            }
        }
        self
    }
}

impl RecordPublicationWorkSettlement {
    pub(in crate::physical_runtime::record_serving) const fn from_canonical(
        effect: Option<PhysicalEffectIdentity>,
        effect_fate: PhysicalWorkEffectFate,
        recovery: PhysicalWorkRecoveryDisposition,
    ) -> Self {
        Self {
            effect,
            effect_fate,
            recovery,
        }
    }

    pub const fn effect(self) -> Option<PhysicalEffectIdentity> {
        self.effect
    }

    pub const fn effect_fate(self) -> PhysicalWorkEffectFate {
        self.effect_fate
    }

    pub const fn recovery(self) -> PhysicalWorkRecoveryDisposition {
        self.recovery
    }
}

impl RecordPublicationWorkEffect {
    pub const fn stage(self) -> RecordPublicationStage {
        self.stage
    }

    pub const fn identity(self) -> PhysicalWorkIdentity {
        self.identity
    }

    pub const fn settlement(self) -> Option<RecordPublicationWorkSettlement> {
        self.settlement
    }
}
