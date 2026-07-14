use worth_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
use worth_query::facade::runtime::{WorthQueryAspectTouch, WorthQueryEffectDeclaration, WorthQueryEffectPolicy, WorthQueryEffectTrigger, WorthQueryLiveView, WorthQueryNativeRow};

fn sample_live_view() -> WorthQueryLiveView<WorthQueryNativeRow> {
    todo!()
}

fn main() {
    let live = sample_live_view();
    let title = WorthQueryAspectTouch::whole_aspect(AspectKey::new("title").unwrap());
    let declaration =
        WorthQueryEffectDeclaration::deliver("effect", WorthQueryEffectTrigger::live_view(&live, [title]), "target");
    let _ = declaration.with_effect_policy(WorthQueryEffectPolicy::Muted);
}
