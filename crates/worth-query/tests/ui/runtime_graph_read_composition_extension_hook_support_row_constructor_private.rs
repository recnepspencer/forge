use worth_query::facade::{
    WorthQueryReadCompositionExtensionHookBoundary,
    WorthQueryReadCompositionExtensionHookFamily,
    WorthQueryReadCompositionExtensionHookSupportRow,
};

fn main() {
    let _ = WorthQueryReadCompositionExtensionHookSupportRow {
        family: WorthQueryReadCompositionExtensionHookFamily::DomainReadFamilyLowering,
        boundary: WorthQueryReadCompositionExtensionHookBoundary::Lowering,
        semantic_bypass_allowed: false,
        row_digest: String::new(),
    };
}
