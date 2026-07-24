use super::UiMeasurementRequestFamily;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiHostMeasurementEnvironmentReport {
    viewport_generation: Option<u64>,
    dpi_generation: Option<u64>,
    font_generation: Option<u64>,
    adapter_generation: Option<u64>,
}

impl UiHostMeasurementEnvironmentReport {
    pub const fn new(
        viewport_generation: Option<u64>,
        dpi_generation: Option<u64>,
        font_generation: Option<u64>,
        adapter_generation: Option<u64>,
    ) -> Self {
        Self {
            viewport_generation,
            dpi_generation,
            font_generation,
            adapter_generation,
        }
    }

    pub const fn unsupported() -> Self {
        Self::new(None, None, None, None)
    }

    pub fn generation_for(self, family: UiMeasurementRequestFamily) -> Option<u64> {
        match family {
            UiMeasurementRequestFamily::ViewportExtent => self.viewport_generation,
            UiMeasurementRequestFamily::DpiScaleFactor => self.dpi_generation,
            UiMeasurementRequestFamily::TextIntrinsicSize
            | UiMeasurementRequestFamily::TextBaselineMetrics
            | UiMeasurementRequestFamily::FontMetrics => self.font_generation,
            UiMeasurementRequestFamily::NativeControlIntrinsicSize
            | UiMeasurementRequestFamily::PortalAnchorRect
            | UiMeasurementRequestFamily::ScrollContainerViewport => self.adapter_generation,
        }
    }
}
