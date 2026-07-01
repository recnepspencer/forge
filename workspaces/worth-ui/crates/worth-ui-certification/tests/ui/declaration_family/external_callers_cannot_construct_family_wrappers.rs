use worth_ui::facade::declaration::{
    UiControlDeclarationFamily, UiDeclarationFamily, UiDiagnosticSurfaceDeclarationFamily,
    UiIntentDeclarationFamily, UiLocalCompositionDeclarationFamily, UiMosaicDeclarationFamily,
    UiPageDeclarationFamily, UiPageSetDeclarationFamily, UiQueryBindingDeclarationFamily,
    UiRegionDeclarationFamily,
};

fn main() {
    let page = UiPageDeclarationFamily::new(
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
    );
    let _page_family = UiDeclarationFamily::Page(page);

    let page_set = UiPageSetDeclarationFamily::new(
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
    );
    let _page_set_family = UiDeclarationFamily::PageSet(page_set);

    let region = UiRegionDeclarationFamily::new(
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
    );
    let _region_family = UiDeclarationFamily::Region(region);

    let mosaic = UiMosaicDeclarationFamily::new(
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
    );
    let _mosaic_family = UiDeclarationFamily::Mosaic(mosaic);

    let local_composition = UiLocalCompositionDeclarationFamily::new(
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
    );
    let _local_composition_family = UiDeclarationFamily::LocalComposition(local_composition);

    let control = UiControlDeclarationFamily::new(
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
        unsafe { std::mem::MaybeUninit::zeroed().assume_init() },
    );
    let _family = UiDeclarationFamily::Control(control);

    let query_binding = UiQueryBindingDeclarationFamily::new();
    let _standalone = UiDeclarationFamily::QueryBinding(query_binding);

    let intent = UiIntentDeclarationFamily::new();
    let _intent_family = UiDeclarationFamily::Intent(intent);

    let diagnostic_surface = UiDiagnosticSurfaceDeclarationFamily::new();
    let _diagnostic_surface_family = UiDeclarationFamily::DiagnosticSurface(diagnostic_surface);
}
