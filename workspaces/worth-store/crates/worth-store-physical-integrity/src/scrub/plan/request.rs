use worth_store::physical_runtime::ScrubPhysicalAllocation;

use crate::{OfflineScrubInspectionInput, ScrubMode, ScrubWindow, ScrubWindowOrdinal};

use super::ScrubPlanPolicy;

#[derive(Debug)]
pub struct ScrubPlanRequest<'runtime, 'lease> {
    pub(super) allocation: ScrubPhysicalAllocation<'runtime>,
    pub(super) mode: ScrubMode,
    pub(super) windows: Vec<ScrubWindow<'lease>>,
    pub(super) policy: ScrubPlanPolicy,
    pub(super) defer_over_budget_windows: bool,
    pub(super) yield_after_windows: Option<u64>,
    pub(super) skipped_ordinals: Vec<ScrubWindowOrdinal>,
}

impl<'runtime, 'lease> ScrubPlanRequest<'runtime, 'lease> {
    pub fn online(
        allocation: ScrubPhysicalAllocation<'runtime>,
        windows: Vec<ScrubWindow<'lease>>,
        policy: ScrubPlanPolicy,
    ) -> Self {
        Self::new(allocation, ScrubMode::Online, windows, policy)
    }

    pub fn offline(
        allocation: ScrubPhysicalAllocation<'runtime>,
        input: OfflineScrubInspectionInput<'lease>,
        policy: ScrubPlanPolicy,
    ) -> Self {
        Self::new(
            allocation,
            ScrubMode::Offline,
            input.windows().to_vec(),
            policy,
        )
    }

    pub fn with_deferred_over_budget_windows(mut self) -> Self {
        self.defer_over_budget_windows = true;
        self
    }

    pub fn with_yield_after_windows(mut self, windows: u64) -> Self {
        self.yield_after_windows = Some(windows);
        self
    }

    pub fn with_skipped_window(mut self, ordinal: ScrubWindowOrdinal) -> Self {
        self.skipped_ordinals.push(ordinal);
        self
    }

    fn new(
        allocation: ScrubPhysicalAllocation<'runtime>,
        mode: ScrubMode,
        windows: Vec<ScrubWindow<'lease>>,
        policy: ScrubPlanPolicy,
    ) -> Self {
        Self {
            allocation,
            mode,
            windows,
            policy,
            defer_over_budget_windows: false,
            yield_after_windows: None,
            skipped_ordinals: Vec::new(),
        }
    }
}
