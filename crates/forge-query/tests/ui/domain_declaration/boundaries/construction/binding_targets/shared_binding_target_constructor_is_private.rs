use forge_query::facade::{
    ForgeQueryBindingTarget, ForgeQueryBindingTargetKind, ForgeQueryBindingTargetSemantics,
};

fn semantics() -> ForgeQueryBindingTargetSemantics {
    loop {}
}

fn main() {
    let _ = ForgeQueryBindingTarget {
        kind: ForgeQueryBindingTargetKind::DeclarationRoutePlan,
        target_digest: String::new(),
        binding_digest: String::new(),
        semantics: semantics(),
    };
}
