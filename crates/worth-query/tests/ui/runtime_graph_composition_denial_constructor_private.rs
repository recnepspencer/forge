use worth_query::facade::runtime::{WorthQueryGraphCompositionDenial, WorthQueryGraphCompositionDenialKind};

fn main() {
    let _ = WorthQueryGraphCompositionDenial {
        kind: WorthQueryGraphCompositionDenialKind::EmptyComposition,
        symbol: None,
        target_collection: None,
        message: String::from("no operations"),
        denial_digest: String::new(),
    };
}
