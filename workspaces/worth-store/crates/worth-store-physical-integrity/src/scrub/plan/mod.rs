mod build;
mod identity;
mod planned_window;
mod policy;
mod request;

use worth_store::physical_runtime::ScrubPhysicalAllocation;

use crate::{ScrubMode, ScrubWindow};

pub use planned_window::{PlannedScrubWindow, PlannedScrubWindowStatus};
pub use policy::ScrubPlanPolicy;
pub use request::ScrubPlanRequest;

#[derive(Debug)]
pub struct ScrubPlan<'runtime, 'lease> {
    pub(super) allocation: ScrubPhysicalAllocation<'runtime>,
    pub(super) mode: ScrubMode,
    pub(super) windows: Vec<PlannedScrubWindow<'lease>>,
    pub(super) policy: ScrubPlanPolicy,
    pub(super) yield_after_windows: Option<u64>,
    pub(super) plan_identity: u64,
}

impl<'runtime, 'lease> ScrubPlan<'runtime, 'lease> {
    pub const fn allocation(&self) -> &ScrubPhysicalAllocation<'runtime> {
        &self.allocation
    }

    pub const fn mode(&self) -> ScrubMode {
        self.mode
    }

    pub fn windows(&self) -> &[PlannedScrubWindow<'lease>] {
        &self.windows
    }

    pub const fn policy(&self) -> ScrubPlanPolicy {
        self.policy
    }

    pub const fn yield_after_windows(&self) -> Option<u64> {
        self.yield_after_windows
    }

    pub const fn plan_identity(&self) -> u64 {
        self.plan_identity
    }

    pub(super) fn revalidation_window(
        &self,
        next_window_index: usize,
    ) -> Option<ScrubWindow<'lease>> {
        next_window_index
            .checked_sub(1)
            .and_then(|index| self.windows.get(index).map(|planned| planned.window()))
    }
}
