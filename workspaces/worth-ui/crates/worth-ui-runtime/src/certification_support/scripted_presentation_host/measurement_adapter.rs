use worth_ui_host_contract::{
    UiDpiScaleFactorObservation, UiFontMetricsObservation, UiHostMeasurementObservationValue,
    UiHostMeasurementRequest, UiMeasurementRequestFamily, UiNativeControlIntrinsicSizeObservation,
    UiPortalAnchorRectObservation, UiScrollContainerViewportObservation,
    UiTextBaselineMetricsObservation, UiTextIntrinsicSizeObservation, UiViewportExtentObservation,
    WorthUiMeasurementHostAdapter,
};

use super::ScriptedPresentationHost;

impl WorthUiMeasurementHostAdapter for ScriptedPresentationHost {
    fn observe_measurement(
        &self,
        request: &UiHostMeasurementRequest,
    ) -> UiHostMeasurementObservationValue {
        match request.family() {
            UiMeasurementRequestFamily::TextIntrinsicSize => {
                UiHostMeasurementObservationValue::TextIntrinsicSize(
                    UiTextIntrinsicSizeObservation {
                        width: 320.0,
                        height: 24.0,
                    },
                )
            }
            UiMeasurementRequestFamily::TextBaselineMetrics => {
                UiHostMeasurementObservationValue::TextBaselineMetrics(
                    UiTextBaselineMetricsObservation {
                        ascent: 16.0,
                        descent: 4.0,
                        baseline: 16.0,
                    },
                )
            }
            UiMeasurementRequestFamily::FontMetrics => {
                UiHostMeasurementObservationValue::FontMetrics(UiFontMetricsObservation {
                    ascent: 16.0,
                    descent: 4.0,
                    line_gap: 2.0,
                })
            }
            UiMeasurementRequestFamily::NativeControlIntrinsicSize => {
                UiHostMeasurementObservationValue::NativeControlIntrinsicSize(
                    UiNativeControlIntrinsicSizeObservation {
                        width: 120.0,
                        height: 32.0,
                    },
                )
            }
            UiMeasurementRequestFamily::ViewportExtent => {
                let extent = {
                    let mut state = self.state.lock().unwrap();
                    state.viewport_measurement_calls = state
                        .viewport_measurement_calls
                        .checked_add(1)
                        .expect("scripted viewport observation count capacity");
                    state.viewport_extent
                };
                UiHostMeasurementObservationValue::ViewportExtent(UiViewportExtentObservation {
                    width: extent[0],
                    height: extent[1],
                })
            }
            UiMeasurementRequestFamily::DpiScaleFactor => {
                UiHostMeasurementObservationValue::DpiScaleFactor(UiDpiScaleFactorObservation {
                    scale_factor: 1.0,
                })
            }
            UiMeasurementRequestFamily::PortalAnchorRect => {
                UiHostMeasurementObservationValue::PortalAnchorRect(UiPortalAnchorRectObservation {
                    x: 24.0,
                    y: 48.0,
                    width: 320.0,
                    height: 180.0,
                })
            }
            UiMeasurementRequestFamily::ScrollContainerViewport => {
                UiHostMeasurementObservationValue::ScrollContainerViewport(
                    UiScrollContainerViewportObservation {
                        width: 800.0,
                        height: 600.0,
                    },
                )
            }
        }
    }
}
