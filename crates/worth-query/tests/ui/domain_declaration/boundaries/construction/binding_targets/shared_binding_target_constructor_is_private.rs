use worth_query::facade::{
    WorthQueryBindingTarget, WorthQueryBindingTargetKind, WorthQueryBindingTargetSemantics,
};

fn semantics() -> WorthQueryBindingTargetSemantics {
    loop {}
}

fn main() {
    let _ = WorthQueryBindingTarget {
        kind: WorthQueryBindingTargetKind::DeclarationRoutePlan,
        target_digest: String::new(),
        binding_digest: String::new(),
        semantics: semantics(),
    };
}
