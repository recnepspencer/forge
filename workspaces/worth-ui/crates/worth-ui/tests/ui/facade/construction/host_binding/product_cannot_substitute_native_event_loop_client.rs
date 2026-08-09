use worth_ui_host_native::{
    UiNativeClientPresentationAttribution, UiNativeEventLoopClient, UiNativeEventLoopDirective,
    UiNativeReadinessGrant,
};
use worth_ui_native_platform::UiPreparedNativePlatform;

struct ForgedNativeClient;

impl UiNativeEventLoopClient for ForgedNativeClient {
    fn native_surface_ready(
        &mut self,
        _grant: UiNativeReadinessGrant,
    ) -> Result<UiNativeEventLoopDirective, ()> {
        Ok(UiNativeEventLoopDirective::Continue)
    }

    fn redraw_ready(
        &mut self,
        _grant: UiNativeReadinessGrant,
    ) -> Result<UiNativeEventLoopDirective, ()> {
        Ok(UiNativeEventLoopDirective::Close)
    }

    fn presentation_attribution(&self) -> Option<UiNativeClientPresentationAttribution> {
        None
    }

    fn close(self) -> Result<(), ()> {
        Ok(())
    }
}

fn substitute_product_driver(platform: UiPreparedNativePlatform) {
    let _ = platform.run(ForgedNativeClient);
}

fn main() {}
