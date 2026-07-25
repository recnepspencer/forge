use crate::physical_runtime::{PhysicalWorkIdentity, RecordPublicationStage};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordPublicationWorkEffect {
    stage: RecordPublicationStage,
    identity: PhysicalWorkIdentity,
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

    pub(in crate::physical_runtime::record_serving) fn record(
        &mut self,
        stage: RecordPublicationStage,
        identity: PhysicalWorkIdentity,
    ) {
        self.effects
            .push(RecordPublicationWorkEffect { stage, identity });
    }

    pub(in crate::physical_runtime::record_serving) fn extend(&mut self, other: Self) {
        self.effects.extend(other.effects);
    }

    pub(in crate::physical_runtime::record_serving) fn including(
        mut self,
        stage: RecordPublicationStage,
        identity: Option<PhysicalWorkIdentity>,
    ) -> Self {
        if let Some(identity) = identity {
            if !self
                .effects
                .iter()
                .any(|effect| effect.identity == identity)
            {
                self.record(stage, identity);
            }
        }
        self
    }
}

impl RecordPublicationWorkEffect {
    pub const fn stage(self) -> RecordPublicationStage {
        self.stage
    }

    pub const fn identity(self) -> PhysicalWorkIdentity {
        self.identity
    }
}
