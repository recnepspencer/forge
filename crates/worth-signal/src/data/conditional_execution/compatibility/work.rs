#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SignalConditionalComparisonWork {
    semantic_dimensions_inspected: u32,
    affinity_dimensions_inspected: u32,
}

impl SignalConditionalComparisonWork {
    pub const fn semantic_dimensions_inspected(self) -> u32 {
        self.semantic_dimensions_inspected
    }

    pub const fn affinity_dimensions_inspected(self) -> u32 {
        self.affinity_dimensions_inspected
    }

    pub(super) fn inspect_semantic(&mut self) {
        self.semantic_dimensions_inspected = self.semantic_dimensions_inspected.saturating_add(1);
    }

    pub(super) fn inspect_affinity(&mut self) {
        self.affinity_dimensions_inspected = self.affinity_dimensions_inspected.saturating_add(1);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignalConditionalSemanticComparisonMismatch {
    mismatch: super::SignalConditionalSemanticMismatch,
    work: SignalConditionalComparisonWork,
}

impl SignalConditionalSemanticComparisonMismatch {
    pub(super) const fn new(
        mismatch: super::SignalConditionalSemanticMismatch,
        work: SignalConditionalComparisonWork,
    ) -> Self {
        Self { mismatch, work }
    }

    pub const fn mismatch(&self) -> &super::SignalConditionalSemanticMismatch {
        &self.mismatch
    }

    pub const fn work(&self) -> SignalConditionalComparisonWork {
        self.work
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignalConditionalExecutionAffinityComparisonMismatch {
    mismatch: super::SignalConditionalExecutionAffinityMismatch,
    work: SignalConditionalComparisonWork,
}

impl SignalConditionalExecutionAffinityComparisonMismatch {
    pub(super) const fn new(
        mismatch: super::SignalConditionalExecutionAffinityMismatch,
        work: SignalConditionalComparisonWork,
    ) -> Self {
        Self { mismatch, work }
    }

    pub const fn mismatch(&self) -> &super::SignalConditionalExecutionAffinityMismatch {
        &self.mismatch
    }

    pub const fn work(&self) -> SignalConditionalComparisonWork {
        self.work
    }
}
