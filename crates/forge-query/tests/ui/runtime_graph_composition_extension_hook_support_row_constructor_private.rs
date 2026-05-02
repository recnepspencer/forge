use forge_query::facade::{
    ForgeQueryGraphCompositionExtensionHookBoundary,
    ForgeQueryGraphCompositionExtensionHookSupportRow,
};

fn main() {
    let _ = ForgeQueryGraphCompositionExtensionHookSupportRow {
        hook_family: String::new(),
        boundary: ForgeQueryGraphCompositionExtensionHookBoundary::Lowering,
        semantic_bypass_allowed: false,
        row_digest: String::new(),
    };
}
