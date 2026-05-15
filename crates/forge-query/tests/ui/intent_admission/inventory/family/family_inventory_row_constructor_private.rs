use forge_query::facade::runtime::{
    ForgeQueryIntentAdmissionFamily, ForgeQueryIntentAdmissionFamilyInventoryRow,
    ForgeQueryIntentAdmissionSurfaceDescriptor,
};

fn main() {
    let _ = ForgeQueryIntentAdmissionFamilyInventoryRow {
        family: ForgeQueryIntentAdmissionFamily::AuthoritativeUserIntent,
        raw_authoring_constructor: ForgeQueryIntentAdmissionSurfaceDescriptor::Available("forged"),
        common_path_front_door: ForgeQueryIntentAdmissionSurfaceDescriptor::Available("forged"),
        advanced_path_front_door: ForgeQueryIntentAdmissionSurfaceDescriptor::Available("forged"),
    };
}
