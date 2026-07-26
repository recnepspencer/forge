use worth_ui::facade::declaration::{
    NativeCapabilityDescriptor, NativeCapabilityFamily, NativeCapabilityId, NativePlatformPosture,
};

pub(crate) fn native_capability(id: &str) -> NativeCapabilityDescriptor {
    NativeCapabilityDescriptor::new(native_capability_id(id))
        .with_family(NativeCapabilityFamily::clipboard())
        .with_platform_posture(NativePlatformPosture::runtime_declared())
}

pub(crate) fn native_capability_id(raw_text: &str) -> NativeCapabilityId {
    NativeCapabilityId::new(raw_text).expect("valid native capability id")
}
