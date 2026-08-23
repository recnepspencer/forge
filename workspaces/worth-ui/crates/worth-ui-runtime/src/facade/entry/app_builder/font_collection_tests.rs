use std::sync::Arc;

use worth_ui_host_contract::{UiFontCollectionGeneration, UiFontSlant};
use worth_ui_text::{
    UiApplicationFontFaceDefinition, UiApplicationFontLicenseRecord,
    UiApplicationFontPackDefinition, UiGlobalFontCollection,
};

use crate::facade::WorthUi;

#[test]
fn public_builder_retains_the_exact_qualified_application_font_generation() {
    let (profile, _) = UiGlobalFontCollection::admit_qualified_profile().unwrap();
    let bytes: Arc<[u8]> = Arc::from(
        &include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../profiles/worth-ui-global-text-v2/fonts/NotoSans-VF.ttf"
        ))[..],
    );
    let definition = UiApplicationFontPackDefinition {
        name: Arc::from("application-owned-fonts"),
        faces: Box::new([UiApplicationFontFaceDefinition {
            family: Arc::from("Application Sans"),
            bytes,
            face_index: 0,
            weight: 400,
            width_milli_percent: 100_000,
            slant: UiFontSlant::Upright,
            license: UiApplicationFontLicenseRecord {
                identifier: Arc::from("OFL-1.1"),
                notice: Arc::from("Application-owned qualified test bytes."),
            },
        }]),
    };
    let (collection, receipt, _) = profile
        .register_application_pack(UiFontCollectionGeneration::new(2).unwrap(), definition)
        .unwrap();
    let builder = WorthUi::app().with_font_collection(Arc::new(collection));

    assert_eq!(builder.font_collection.generation().get(), 2);
    assert_eq!(
        builder.font_collection.application_packs()[0].identity(),
        receipt.identity()
    );
}
