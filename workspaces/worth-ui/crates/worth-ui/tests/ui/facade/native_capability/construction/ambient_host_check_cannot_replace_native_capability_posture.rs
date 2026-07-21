use worth_ui::facade::{
    registry::{NativeCapabilityDescriptor, NativeCapabilityId},
    support::AmbientHostCheck,
};

fn main() {
    let native_id = NativeCapabilityId::new("platform.native.clipboard").expect("valid native id");
    let _ = NativeCapabilityDescriptor::new(native_id)
        .with_ambient_host_check(AmbientHostCheck::current_host());
}
