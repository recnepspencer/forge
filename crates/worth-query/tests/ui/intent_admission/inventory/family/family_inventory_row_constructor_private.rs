use worth_query::facade::runtime::{
    WorthQueryIntentAdmissionFamily, WorthQueryIntentAdmissionFamilyInventoryRow,
    WorthQueryIntentAdmissionSurfaceDescriptor,
};

fn main() {
    let _ = WorthQueryIntentAdmissionFamilyInventoryRow {
        family: WorthQueryIntentAdmissionFamily::AuthoritativeUserIntent,
        raw_authoring_constructor: WorthQueryIntentAdmissionSurfaceDescriptor::Available("Worthd"),
        common_path_front_door: WorthQueryIntentAdmissionSurfaceDescriptor::Available("Worthd"),
        advanced_path_front_door: WorthQueryIntentAdmissionSurfaceDescriptor::Available("Worthd"),
    };
}
