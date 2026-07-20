use worth_ui::facade::{
    app::WorthUi,
    registry::{NativeCapabilityDescriptor, NativeCapabilityFamily, NativeCapabilityId, NativePlatformPosture},
};

fn main() {
    let native_id = NativeCapabilityId::new("platform.native.clipboard").expect("valid native id");
    let app = WorthUi::app()
        .register_native_capability(
            NativeCapabilityDescriptor::new(native_id.clone())
                .with_family(NativeCapabilityFamily::clipboard())
                .with_platform_posture(NativePlatformPosture::runtime_declared()),
        )
        .freeze().expect("application preparation should succeed");

    let _ = app.capabilities().native_capabilities().get(&native_id);
}
