use forge_query::facade::{ForgeQueryGraphCompositionDenial, ForgeQueryGraphCompositionDenialKind};

fn main() {
    let _ = ForgeQueryGraphCompositionDenial {
        kind: ForgeQueryGraphCompositionDenialKind::EmptyComposition,
        symbol: None,
        target_collection: None,
        message: String::from("no operations"),
        denial_digest: String::new(),
    };
}
