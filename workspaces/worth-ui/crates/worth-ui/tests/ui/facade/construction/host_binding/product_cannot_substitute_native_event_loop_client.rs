use worth_ui_host_native::{
    UiNativeClientPresentationAttribution, UiNativeEventLoopClient, UiNativeEventLoopClientClose,
    UiNativeEventLoopClientFailure, UiNativeEventLoopDirective, UiNativeObservationReadinessGrant,
    UiNativeReadinessGrant,
};
use worth_ui_native_platform::UiPreparedNativePlatform;

struct ForgedNativeClient;

impl UiNativeEventLoopClient for ForgedNativeClient {
    fn native_surface_ready(
        &mut self,
        _grant: UiNativeReadinessGrant,
    ) -> Result<UiNativeEventLoopDirective, UiNativeEventLoopClientFailure> {
        Ok(UiNativeEventLoopDirective::Continue)
    }

    fn redraw_ready(
        &mut self,
        _grant: UiNativeReadinessGrant,
    ) -> Result<UiNativeEventLoopDirective, UiNativeEventLoopClientFailure> {
        Ok(UiNativeEventLoopDirective::Close)
    }

    fn native_observations_ready(
        &mut self,
        _grant: UiNativeObservationReadinessGrant,
    ) -> Result<UiNativeEventLoopDirective, UiNativeEventLoopClientFailure> {
        Ok(UiNativeEventLoopDirective::Close)
    }

    fn presentation_attribution(&self) -> Option<UiNativeClientPresentationAttribution> {
        None
    }

    fn close(self) -> UiNativeEventLoopClientClose {
        UiNativeEventLoopClientClose::Complete
    }
}

fn substitute_product_driver(platform: UiPreparedNativePlatform) {
    let _ = platform.run(ForgedNativeClient);
}

fn main() {}
