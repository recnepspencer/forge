use forge_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
use forge_query::facade::{
    ForgeQueryAspectTouch, ForgeQueryAuthorityLane, ForgeQueryEffectDeclaration, ForgeQueryEffectTrigger,
    ForgeQueryLiveView, ForgeQueryNativeRow,
};

fn sample_live_view() -> ForgeQueryLiveView<ForgeQueryNativeRow> {
    todo!()
}

fn main() {
    let live = sample_live_view();
    let title = ForgeQueryAspectTouch::whole_aspect(AspectKey::new("title").unwrap());
    let declaration =
        ForgeQueryEffectDeclaration::deliver("effect", ForgeQueryEffectTrigger::live_view(&live, [title]), "target");
    let _ = declaration.with_target_lane(ForgeQueryAuthorityLane::AuthoritativeTruth);
}
