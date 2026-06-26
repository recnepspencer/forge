use super::receipt::{WorthUiReloadCounterStopStage, WorthUiReloadLoweringCounterReceiptBuilder};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiReloadCounterBoundary;

impl WorthUiReloadCounterBoundary {
    pub fn reload_completed() -> WorthUiReloadLoweringCounterReceiptBuilder {
        WorthUiReloadLoweringCounterReceiptBuilder::new(
            WorthUiReloadCounterStopStage::PlanEquivalence,
        )
    }

    pub fn stopped_at(
        stopped_at: WorthUiReloadCounterStopStage,
    ) -> WorthUiReloadLoweringCounterReceiptBuilder {
        WorthUiReloadLoweringCounterReceiptBuilder::new(stopped_at)
    }
}
