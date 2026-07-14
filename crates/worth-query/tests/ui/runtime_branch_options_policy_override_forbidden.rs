use worth_query::facade::runtime::{WorthQueryBranchOptions, WorthQueryEffectPolicy};

fn main() {
    let _ = WorthQueryBranchOptions::derive_only()
        .with_effect_policy(WorthQueryEffectPolicy::AuthoritativeAllowed);
}
