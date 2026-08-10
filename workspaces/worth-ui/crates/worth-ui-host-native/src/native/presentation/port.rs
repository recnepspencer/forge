use worth_ui_host_contract::UiHostPresentationCostReport;

use crate::native::UiNativeGraphics;

use super::{RasterRect, UiNativePendingExternalObligation};

mod transaction;

/// Contractual boundary for one native presentation transaction.
///
/// The real implementation owns wgpu acquisition, encoding, submission,
/// present handoff, and retained-source readback. Protocol tests may replace
/// only this boundary; they cannot return a framework settlement verdict.
pub(crate) trait UiNativePresentationPort {
    fn present(
        graphics: &mut UiNativeGraphics,
        plan: UiNativePresentationPortPlan,
    ) -> Result<UiNativePresentationPortObservation, UiNativePresentationPortFailure>;
}

pub(crate) enum UiNativePresentationPortFailure {
    SurfaceUnavailable,
    ReadbackUnsettled(Box<dyn UiNativePendingExternalObligation>),
}

pub(crate) struct UiWgpuNativePresentationPort;

#[derive(Clone, Copy)]
pub(crate) enum UiNativeRasterOperation {
    Clear(RasterRect),
    FilledRect {
        rect: RasterRect,
        source_rgba8: [u8; 4],
    },
}

pub(crate) struct UiNativePresentationPortPlan {
    pub(super) clear_retained_target: bool,
    pub(super) operations: Box<[UiNativeRasterOperation]>,
    pub(super) cost: UiHostPresentationCostReport,
}

pub(crate) struct UiNativePresentationPortObservation {
    pixels: [[u8; 4]; 2],
    cost: UiHostPresentationCostReport,
    crossing_count: u8,
}

impl UiNativePresentationPortObservation {
    pub(super) fn into_parts(self) -> ([[u8; 4]; 2], UiHostPresentationCostReport, u8) {
        (self.pixels, self.cost, self.crossing_count)
    }
}

impl UiNativePresentationPort for UiWgpuNativePresentationPort {
    fn present(
        graphics: &mut UiNativeGraphics,
        plan: UiNativePresentationPortPlan,
    ) -> Result<UiNativePresentationPortObservation, UiNativePresentationPortFailure> {
        transaction::present(graphics, plan)
    }
}
