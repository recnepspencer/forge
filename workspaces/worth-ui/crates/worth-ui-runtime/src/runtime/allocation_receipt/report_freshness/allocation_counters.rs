#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiAllocationCounterName {
    InvalidationClassifications,
    NeighborhoodSelections,
    ReplannedNeighborhoods,
    ReusedReceipts,
    DeniedReuseAttempts,
    ChurnBurstInputs,
    CommittedReceipts,
    RootWidenAttempts,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiAllocationCounterValue {
    name: UiAllocationCounterName,
    observed: u16,
    maximum: u16,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAllocationCounterReport {
    values: Box<[UiAllocationCounterValue]>,
}

impl UiAllocationCounterReport {
    pub(super) fn from_commit(
        transaction: &super::UiAllocationReplanTransaction,
        counters: super::UiAllocationReplanTransactionCounters,
    ) -> Self {
        let budget = transaction.policy().budget();
        let invalidations = transaction.invalidation_count();
        let ingress = transaction.ingress_count();
        let root_widen = u16::from(matches!(
            transaction.root_posture(),
            crate::graph::UiReplanRootPosture::CountedRootWiden { .. }
        ));
        Self {
            values: vec![
                value(
                    UiAllocationCounterName::InvalidationClassifications,
                    invalidations,
                    budget.max_invalidation_targets(),
                ),
                value(
                    UiAllocationCounterName::NeighborhoodSelections,
                    counters.selected_neighborhoods(),
                    budget.max_invalidation_targets(),
                ),
                value(
                    UiAllocationCounterName::ReplannedNeighborhoods,
                    counters.replanned_neighborhoods(),
                    budget.max_committed_receipts(),
                ),
                value(
                    UiAllocationCounterName::ReusedReceipts,
                    counters.reused_neighborhoods(),
                    budget.max_committed_receipts(),
                ),
                value(
                    UiAllocationCounterName::DeniedReuseAttempts,
                    0,
                    budget.max_committed_receipts(),
                ),
                value(
                    UiAllocationCounterName::ChurnBurstInputs,
                    ingress,
                    budget.ingress_window(),
                ),
                value(
                    UiAllocationCounterName::CommittedReceipts,
                    counters.committed_receipts(),
                    budget.max_committed_receipts(),
                ),
                value(UiAllocationCounterName::RootWidenAttempts, root_widen, 1),
            ]
            .into_boxed_slice(),
        }
    }

    pub fn values(&self) -> &[UiAllocationCounterValue] {
        &self.values
    }
    pub fn value(&self, name: UiAllocationCounterName) -> UiAllocationCounterValue {
        *self
            .values
            .iter()
            .find(|value| value.name == name)
            .expect("all mandatory allocation counters are present")
    }
}

const fn value(
    name: UiAllocationCounterName,
    observed: u16,
    maximum: u16,
) -> UiAllocationCounterValue {
    UiAllocationCounterValue {
        name,
        observed,
        maximum,
    }
}

impl UiAllocationCounterValue {
    pub const fn name(self) -> UiAllocationCounterName {
        self.name
    }
    pub const fn observed(self) -> u16 {
        self.observed
    }
    pub const fn maximum(self) -> u16 {
        self.maximum
    }
    pub const fn is_within_bound(self) -> bool {
        self.observed <= self.maximum
    }
}
