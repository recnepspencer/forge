use worth_ui_host_contract::{
    UiDpiScaleFactorObservation, UiHostMeasurementEnvironmentReport,
    UiHostMeasurementObservationValue, UiHostMeasurementRequest, UiMeasurementRequestFamily,
    UiViewportExtentObservation, WorthUiHostCapability,
};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(super) struct UiHeadlessMeasurementEnvironment {
    viewport: Option<UiViewportExtentObservation>,
    dpi_scale: Option<UiDpiScaleFactorObservation>,
}

impl UiHeadlessMeasurementEnvironment {
    pub(super) const fn unsupported() -> Self {
        Self {
            viewport: None,
            dpi_scale: None,
        }
    }

    pub(super) const fn fixed_viewport(viewport: UiViewportExtentObservation) -> Self {
        Self {
            viewport: Some(viewport),
            dpi_scale: None,
        }
    }

    pub(super) const fn fixed_viewport_and_dpi(
        viewport: UiViewportExtentObservation,
        dpi_scale: UiDpiScaleFactorObservation,
    ) -> Self {
        Self {
            viewport: Some(viewport),
            dpi_scale: Some(dpi_scale),
        }
    }

    pub(super) fn append_capabilities(self, capabilities: &mut Vec<WorthUiHostCapability>) {
        if self.viewport.is_some() {
            capabilities.push(WorthUiHostCapability::ViewportObservation);
        }
        if self.dpi_scale.is_some() {
            capabilities.push(WorthUiHostCapability::DpiObservation);
        }
    }

    pub(super) fn report(self) -> UiHostMeasurementEnvironmentReport {
        UiHostMeasurementEnvironmentReport::new(
            self.viewport.map(|_| 1),
            self.dpi_scale.map(|_| 1),
            None,
            None,
        )
    }

    pub(super) fn observe(
        self,
        request: &UiHostMeasurementRequest,
    ) -> UiHostMeasurementObservationValue {
        match (request.family(), self.viewport) {
            (UiMeasurementRequestFamily::ViewportExtent, Some(viewport)) => {
                UiHostMeasurementObservationValue::ViewportExtent(viewport)
            }
            (UiMeasurementRequestFamily::DpiScaleFactor, _) => {
                UiHostMeasurementObservationValue::DpiScaleFactor(
                    self.dpi_scale
                        .expect("DPI capability admitted without evidence"),
                )
            }
            (family, _) => unreachable!(
                "headless recorder capability report does not admit {family:?} observation"
            ),
        }
    }
}
