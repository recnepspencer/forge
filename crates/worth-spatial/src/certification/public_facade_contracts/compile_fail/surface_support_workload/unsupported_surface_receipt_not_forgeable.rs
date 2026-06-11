use worth_spatial::facade::surface_support::{
    UnsupportedSurfaceSupportReceipt, UnsupportedSurfaceSupportReasonCode,
};

fn main() {
    let _ = UnsupportedSurfaceSupportReceipt::new(
        "try forge unsupported surface receipt".to_string(),
        "geometry-binding".to_string(),
        None,
        UnsupportedSurfaceSupportReasonCode::FamilyNotAdmitted,
        "try forge unsupported receipt".to_string(),
        Vec::new(),
        0,
    );
}
