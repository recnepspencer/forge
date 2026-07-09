use worth_query::facade::{
    WorthQueryGraphCompositionExtensionHookBoundary,
    WorthQueryGraphCompositionExtensionHookSupportRow,
};

fn main() {
    let _ = WorthQueryGraphCompositionExtensionHookSupportRow {
        hook_family: String::new(),
        boundary: WorthQueryGraphCompositionExtensionHookBoundary::Lowering,
        semantic_bypass_allowed: false,
        row_digest: String::new(),
    };
}
