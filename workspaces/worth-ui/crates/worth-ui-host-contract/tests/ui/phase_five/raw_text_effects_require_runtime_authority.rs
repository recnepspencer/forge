use worth_ui_host_native::{
    UiNativePlatformEffectAuthority, UiNativeWindowConfiguration, WorthUiNativeMechanicsAdapter,
    WorthUiPreparedNativeHost,
};

fn main() {
    let host = WorthUiPreparedNativeHost::prepare_qualified();
    let (mechanics, _event_loop) = host.into_parts(UiNativeWindowConfiguration::qualified(
        "authority bypass",
        [160, 96],
    ));
    let _: (WorthUiNativeMechanicsAdapter, UiNativePlatformEffectAuthority) =
        mechanics.into_runtime_binding();
    let _ = mechanics.perform_mounted_text_raster_preparation(
        todo!(),
        todo!(),
        todo!(),
        todo!(),
    );
    let _ = mechanics.perform_mounted_text_raster_completion(todo!());
    let _ = mechanics.perform_mounted_text_raster_cancellation(todo!());
    let _ = mechanics.perform_mounted_text_pin_release(todo!(), todo!());
}
