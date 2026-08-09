use worth_ui_host_contract::{UiHostPresentationCostReport, UiMountedFrameConsumptionView};

use crate::native::{UiNativeGraphics, UiNativePresentationObservation, UiNativeResourceRegistry};

use super::{present_initial, UiNativePresentationFailure};

/// Contractual boundary for one native presentation transaction.
///
/// The real implementation owns wgpu acquisition, encoding, submission,
/// present handoff, and retained-source readback. Protocol tests may replace
/// only this boundary; they cannot return a framework settlement verdict.
pub(crate) trait UiNativePresentationPort {
    fn present(
        graphics: &mut UiNativeGraphics,
        resources: &mut UiNativeResourceRegistry,
        view: &UiMountedFrameConsumptionView<'_>,
    ) -> Result<
        (
            UiNativePresentationObservation,
            UiHostPresentationCostReport,
        ),
        UiNativePresentationFailure,
    >;
}

pub(crate) struct UiWgpuNativePresentationPort;

impl UiNativePresentationPort for UiWgpuNativePresentationPort {
    fn present(
        graphics: &mut UiNativeGraphics,
        resources: &mut UiNativeResourceRegistry,
        view: &UiMountedFrameConsumptionView<'_>,
    ) -> Result<
        (
            UiNativePresentationObservation,
            UiHostPresentationCostReport,
        ),
        UiNativePresentationFailure,
    > {
        let (mut observation, cost) = present_initial(graphics, resources, view)?;
        observation.record_presentation_port_crossing();
        Ok((observation, cost))
    }
}
