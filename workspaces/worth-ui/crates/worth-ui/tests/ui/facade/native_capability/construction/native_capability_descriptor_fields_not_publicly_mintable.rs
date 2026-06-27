use worth_ui::facade::{
    NativeCapabilityDescriptor, NativeCapabilityFamily, NativeCapabilityId,
    NativePlatformPosture,
};

fn main() {
    let _ = NativeCapabilityDescriptor {
        id: NativeCapabilityId::new("platform.native.clipboard").expect("valid native id"),
        family: Some(NativeCapabilityFamily::clipboard()),
        platform_posture: Some(NativePlatformPosture::runtime_declared()),
        shell_authority_claims: Vec::new(),
        ambient_host_checks: Vec::new(),
    };
}
