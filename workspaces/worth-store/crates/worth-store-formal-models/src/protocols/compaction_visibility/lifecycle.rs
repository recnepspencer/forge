#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompactionLifecycleState {
    Planned,
    WritingNewGeneration,
    NewGenerationDurable,
    PublicationAttempted,
    NewGenerationVisible,
    OrphanedNewGeneration,
    PublicationRolledBack,
    ReclaimEligible,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompactionLifecycleDenial {
    PublicationBeforeDurability,
    ReclaimBeforeReadRelease,
    TombstoneResurrection,
    GenerationMismatch,
    IllegalTransition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompactionLifecycleModel {
    state: CompactionLifecycleState,
    old_generation_retained: bool,
    tombstone_preserved: bool,
}

impl CompactionLifecycleModel {
    pub const fn planned() -> Self {
        Self {
            state: CompactionLifecycleState::Planned,
            old_generation_retained: true,
            tombstone_preserved: true,
        }
    }

    pub fn begin_write(&mut self) -> Result<(), CompactionLifecycleDenial> {
        self.transition(
            CompactionLifecycleState::Planned,
            CompactionLifecycleState::WritingNewGeneration,
        )
    }

    pub fn complete_durability(&mut self) -> Result<(), CompactionLifecycleDenial> {
        self.transition(
            CompactionLifecycleState::WritingNewGeneration,
            CompactionLifecycleState::NewGenerationDurable,
        )
    }

    pub fn attempt_publication(&mut self) -> Result<(), CompactionLifecycleDenial> {
        if self.state != CompactionLifecycleState::NewGenerationDurable {
            return Err(CompactionLifecycleDenial::PublicationBeforeDurability);
        }
        self.state = CompactionLifecycleState::PublicationAttempted;
        Ok(())
    }

    pub fn publish(&mut self) -> Result<(), CompactionLifecycleDenial> {
        self.transition(
            CompactionLifecycleState::PublicationAttempted,
            CompactionLifecycleState::NewGenerationVisible,
        )
    }

    pub fn classify_crash(&mut self) {
        if matches!(
            self.state,
            CompactionLifecycleState::WritingNewGeneration
                | CompactionLifecycleState::NewGenerationDurable
                | CompactionLifecycleState::PublicationAttempted
        ) {
            self.state = CompactionLifecycleState::OrphanedNewGeneration;
        }
    }

    pub fn admit_reclaim(
        &mut self,
        readers_released: bool,
    ) -> Result<(), CompactionLifecycleDenial> {
        if !readers_released || !self.old_generation_retained {
            return Err(CompactionLifecycleDenial::ReclaimBeforeReadRelease);
        }
        self.state = CompactionLifecycleState::ReclaimEligible;
        self.old_generation_retained = false;
        Ok(())
    }

    pub fn observe_tombstone_preservation(
        &mut self,
        preserved: bool,
    ) -> Result<(), CompactionLifecycleDenial> {
        self.tombstone_preserved = preserved;
        if preserved {
            Ok(())
        } else {
            Err(CompactionLifecycleDenial::TombstoneResurrection)
        }
    }

    pub const fn state(self) -> CompactionLifecycleState {
        self.state
    }

    fn transition(
        &mut self,
        from: CompactionLifecycleState,
        to: CompactionLifecycleState,
    ) -> Result<(), CompactionLifecycleDenial> {
        if self.state != from {
            return Err(CompactionLifecycleDenial::IllegalTransition);
        }
        self.state = to;
        Ok(())
    }
}
