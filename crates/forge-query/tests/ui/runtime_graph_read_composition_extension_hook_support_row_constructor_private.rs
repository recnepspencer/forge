use forge_query::facade::{
    ForgeQueryReadCompositionExtensionHookBoundary,
    ForgeQueryReadCompositionExtensionHookFamily,
    ForgeQueryReadCompositionExtensionHookSupportRow,
};

fn main() {
    let _ = ForgeQueryReadCompositionExtensionHookSupportRow {
        family: ForgeQueryReadCompositionExtensionHookFamily::DomainReadFamilyLowering,
        boundary: ForgeQueryReadCompositionExtensionHookBoundary::Lowering,
        semantic_bypass_allowed: false,
        row_digest: String::new(),
    };
}
