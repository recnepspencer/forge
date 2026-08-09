use worth_ui_host_contract::{
    UiHostMeasurementEnvironmentReport, UiHostMeasurementObservationValue,
    UiHostMeasurementRequest, UiMeasurementRequestFamily, UiViewportExtentObservation,
    WorthUiHostCapability,
};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(super) struct UiHeadlessMeasurementEnvironment {
    viewport: Option<UiViewportExtentObservation>,
}

impl UiHeadlessMeasurementEnvironment {
    pub(super) const fn unsupported() -> Self {
        Self { viewport: None }
    }

    pub(super) const fn fixed_viewport(viewport: UiViewportExtentObservation) -> Self {
        Self {
            viewport: Some(viewport),
        }
    }

    pub(super) fn append_capabilities(self, capabilities: &mut Vec<WorthUiHostCapability>) {
        if self.viewport.is_some() {
            capabilities.push(WorthUiHostCapability::ViewportObservation);
        }
    }

    pub(super) fn report(self) -> UiHostMeasurementEnvironmentReport {
        UiHostMeasurementEnvironmentReport::new(self.viewport.map(|_| 1), None, None, None)
    }

    pub(super) fn observe(
        self,
        request: &UiHostMeasurementRequest,
    ) -> UiHostMeasurementObservationValue {
        match (request.family(), self.viewport) {
            (UiMeasurementRequestFamily::ViewportExtent, Some(viewport)) => {
                UiHostMeasurementObservationValue::ViewportExtent(viewport)
            }
            (family, _) => unreachable!(
                "headless recorder capability report does not admit {family:?} observation"
            ),
        }
    }
}
