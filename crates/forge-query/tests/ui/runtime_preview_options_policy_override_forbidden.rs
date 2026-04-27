use forge_query::facade::{ForgeQueryEffectPolicy, ForgeQueryPreviewOptions};

fn main() {
    let _ = ForgeQueryPreviewOptions::derive_only()
        .with_effect_policy(ForgeQueryEffectPolicy::AuthoritativeAllowed);
}
