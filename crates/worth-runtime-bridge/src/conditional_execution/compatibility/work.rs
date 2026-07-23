#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BridgeConditionalComparisonWork {
    portable_foundational_comparisons: u32,
    liveness_checks: u32,
    correspondences_inspected: u32,
    targets_inspected: u32,
    provider_roles_inspected: u32,
    signal_semantic_dimensions_inspected: u32,
    signal_affinity_dimensions_inspected: u32,
    bridge_affinity_dimensions_inspected: u32,
}

impl BridgeConditionalComparisonWork {
    pub const fn portable_foundational_comparisons(self) -> u32 {
        self.portable_foundational_comparisons
    }
    pub const fn liveness_checks(self) -> u32 {
        self.liveness_checks
    }
    pub const fn correspondences_inspected(self) -> u32 {
        self.correspondences_inspected
    }
    pub const fn targets_inspected(self) -> u32 {
        self.targets_inspected
    }
    pub const fn provider_roles_inspected(self) -> u32 {
        self.provider_roles_inspected
    }
    pub const fn signal_semantic_dimensions_inspected(self) -> u32 {
        self.signal_semantic_dimensions_inspected
    }
    pub const fn signal_affinity_dimensions_inspected(self) -> u32 {
        self.signal_affinity_dimensions_inspected
    }
    pub const fn bridge_affinity_dimensions_inspected(self) -> u32 {
        self.bridge_affinity_dimensions_inspected
    }

    pub(super) fn record_portable(&mut self, comparisons: u32) {
        self.portable_foundational_comparisons = self
            .portable_foundational_comparisons
            .saturating_add(comparisons);
    }
    pub(super) fn inspect_liveness(&mut self) {
        self.liveness_checks = self.liveness_checks.saturating_add(1);
    }
    pub(super) fn inspect_correspondence(&mut self) {
        self.correspondences_inspected = self.correspondences_inspected.saturating_add(1);
    }
    pub(super) fn inspect_target(&mut self) {
        self.targets_inspected = self.targets_inspected.saturating_add(1);
    }
    pub(super) fn inspect_provider_role(&mut self) {
        self.provider_roles_inspected = self.provider_roles_inspected.saturating_add(1);
    }
    pub(super) fn record_signal(
        &mut self,
        work: worth_signal::facade::SignalConditionalComparisonWork,
    ) {
        self.signal_semantic_dimensions_inspected = self
            .signal_semantic_dimensions_inspected
            .saturating_add(work.semantic_dimensions_inspected());
        self.signal_affinity_dimensions_inspected = self
            .signal_affinity_dimensions_inspected
            .saturating_add(work.affinity_dimensions_inspected());
    }
    pub(super) fn inspect_bridge_affinity(&mut self) {
        self.bridge_affinity_dimensions_inspected =
            self.bridge_affinity_dimensions_inspected.saturating_add(1);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeConditionalContinuityDenial {
    mismatch: super::BridgeConditionalContinuityMismatch,
    work: BridgeConditionalComparisonWork,
}

impl BridgeConditionalContinuityDenial {
    pub(super) const fn new(
        mismatch: super::BridgeConditionalContinuityMismatch,
        work: BridgeConditionalComparisonWork,
    ) -> Self {
        Self { mismatch, work }
    }
    pub const fn mismatch(&self) -> &super::BridgeConditionalContinuityMismatch {
        &self.mismatch
    }
    pub const fn work(&self) -> BridgeConditionalComparisonWork {
        self.work
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeConditionalExecutionAffinityDenial {
    mismatch: super::BridgeConditionalExecutionAffinityMismatch,
    work: BridgeConditionalComparisonWork,
}

impl BridgeConditionalExecutionAffinityDenial {
    pub(super) const fn new(
        mismatch: super::BridgeConditionalExecutionAffinityMismatch,
        work: BridgeConditionalComparisonWork,
    ) -> Self {
        Self { mismatch, work }
    }
    pub const fn mismatch(&self) -> &super::BridgeConditionalExecutionAffinityMismatch {
        &self.mismatch
    }
    pub const fn work(&self) -> BridgeConditionalComparisonWork {
        self.work
    }
}
