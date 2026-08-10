//! Deterministic fixed host used by the authority-closure scenario.

use worth_ui::facade::app::{WorthUiApp, WorthUiHostNeutralApp};
use worth_ui_host_contract::{UiDpiScaleFactorObservation, UiViewportExtentObservation};
use worth_ui_host_headless::{UiHeadlessRecorderCapacity, WorthUiHeadlessRecorder};

use super::fixed_host::FixedCertificationHostBinding;

#[derive(Clone, Copy, Default)]
pub(super) struct AuthorityClosureHost;

impl super::fixed_host::sealed::Sealed for AuthorityClosureHost {}

impl FixedCertificationHostBinding for AuthorityClosureHost {
    fn activate(self, application: WorthUiHostNeutralApp) -> WorthUiApp {
        let recorder = WorthUiHeadlessRecorder::with_viewport_extent_and_dpi(
            UiHeadlessRecorderCapacity::production_default(),
            UiViewportExtentObservation {
                width: 1280.0,
                height: 720.0,
            },
            UiDpiScaleFactorObservation { scale_factor: 1.0 },
        );
        recorder.activate(application)
    }
}
