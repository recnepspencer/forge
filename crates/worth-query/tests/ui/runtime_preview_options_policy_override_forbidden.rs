use worth_query::facade::runtime::{WorthQueryEffectPolicy, WorthQueryPreviewOptions};

fn main() {
    let _ = WorthQueryPreviewOptions::derive_only()
        .with_effect_policy(WorthQueryEffectPolicy::AuthoritativeAllowed);
}
