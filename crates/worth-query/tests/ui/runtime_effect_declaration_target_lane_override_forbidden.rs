use worth_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
use worth_query::facade::runtime::{WorthQueryAspectTouch, WorthQueryAuthorityLane, WorthQueryEffectDeclaration, WorthQueryEffectTrigger, WorthQueryLiveView, WorthQueryUnrefinedLiveShape};

fn sample_live_view() -> WorthQueryLiveView<WorthQueryUnrefinedLiveShape> {
    todo!()
}

fn main() {
    let live = sample_live_view();
    let title = WorthQueryAspectTouch::whole_aspect(AspectKey::new("title").unwrap());
    let declaration =
        WorthQueryEffectDeclaration::deliver("effect", WorthQueryEffectTrigger::live_view(&live, [title]), "target");
    let _ = declaration.with_target_lane(WorthQueryAuthorityLane::AuthoritativeTruth);
}
