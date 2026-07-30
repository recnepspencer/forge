use crate::{ScrubOverBudgetClass, ScrubWindow};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlannedScrubWindowStatus {
    Inspect,
    Skip,
    DeferOverBudget(ScrubOverBudgetClass),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlannedScrubWindow<'lease> {
    window: ScrubWindow<'lease>,
    status: PlannedScrubWindowStatus,
}

impl<'lease> PlannedScrubWindow<'lease> {
    pub(super) const fn new(window: ScrubWindow<'lease>, status: PlannedScrubWindowStatus) -> Self {
        Self { window, status }
    }

    pub const fn window(self) -> ScrubWindow<'lease> {
        self.window
    }

    pub const fn status(self) -> PlannedScrubWindowStatus {
        self.status
    }
}
