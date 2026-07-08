use forge_store_extensions::{
    layout_customization_catalog, ExtensionFamilyPosture, FutureLayoutTarget,
};

fn main() {
    let _ = layout_customization_catalog().declare_target(
        FutureLayoutTarget::StableBasisRead,
        ExtensionFamilyPosture::Registered,
    );
}
