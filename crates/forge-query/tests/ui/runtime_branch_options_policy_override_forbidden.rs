use forge_query::facade::{ForgeQueryBranchOptions, ForgeQueryEffectPolicy};

fn main() {
    let _ = ForgeQueryBranchOptions::derive_only()
        .with_effect_policy(ForgeQueryEffectPolicy::AuthoritativeAllowed);
}
