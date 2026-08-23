use super::{UiNativeEventLoopStopReport, UiNativeResourceCensus};

impl UiNativeEventLoopStopReport {
    pub const fn cause(&self) -> super::UiNativeEventLoopRunDenial {
        self.cause
    }

    pub const fn effect_posture(&self) -> super::UiNativeEffectPosture {
        self.effect_posture
    }

    pub const fn peak_census(&self) -> UiNativeResourceCensus {
        self.peak_census
    }

    pub const fn terminal_census(&self) -> UiNativeResourceCensus {
        self.terminal_census
    }

    pub const fn client_cleanup_complete(&self) -> bool {
        self.client_cleanup_complete
    }

    pub fn into_cleanup(self) -> Option<super::UiNativeEventLoopCleanup> {
        self.cleanup
    }

    #[doc(hidden)]
    pub fn peak_text_pins(&self) -> &[crate::native::text_atlas::UiNativeTextPinObservation] {
        &self.peak_text_pins
    }

    pub fn input_observations(&self) -> &crate::UiNativeInputObservationReport {
        &self.input_observations
    }
}
