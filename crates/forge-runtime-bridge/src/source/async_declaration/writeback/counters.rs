#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BridgeAsyncWritebackCounters {
    writeback_admission_count: usize,
    mapper_output_count: usize,
    staged_effect_count: usize,
    committed_count: usize,
    noop_count: usize,
    rejected_count: usize,
    duplicate_noop_count: usize,
    authority_rejection_count: usize,
}

impl BridgeAsyncWritebackCounters {
    pub(crate) fn admitted() -> Self {
        Self {
            writeback_admission_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn mapped() -> Self {
        Self {
            mapper_output_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn staged() -> Self {
        Self {
            staged_effect_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn committed() -> Self {
        Self {
            committed_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn duplicate_noop() -> Self {
        Self {
            noop_count: 1,
            duplicate_noop_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn canonical_noop() -> Self {
        Self {
            noop_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn rejected() -> Self {
        Self {
            rejected_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn authority_rejected() -> Self {
        Self {
            rejected_count: 1,
            authority_rejection_count: 1,
            ..Self::default()
        }
    }

    pub fn writeback_admission_count(&self) -> usize {
        self.writeback_admission_count
    }

    pub fn mapper_output_count(&self) -> usize {
        self.mapper_output_count
    }

    pub fn staged_effect_count(&self) -> usize {
        self.staged_effect_count
    }

    pub fn committed_count(&self) -> usize {
        self.committed_count
    }

    pub fn noop_count(&self) -> usize {
        self.noop_count
    }

    pub fn rejected_count(&self) -> usize {
        self.rejected_count
    }

    pub fn duplicate_noop_count(&self) -> usize {
        self.duplicate_noop_count
    }

    pub fn authority_rejection_count(&self) -> usize {
        self.authority_rejection_count
    }
}
