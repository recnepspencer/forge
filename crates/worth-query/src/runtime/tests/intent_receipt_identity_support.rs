use super::support::*;

pub(super) fn declare_identity_effect(
    runtime: &mut WorthQueryRuntime,
    live_name: &str,
    effect_name: &str,
) -> WorthQueryEffectHandle<WorthQueryNativeRow> {
    let live = runtime
        .declare_live_view::<WorthQueryNativeRow>(live_name, task_live_request(), task_schema())
        .expect("live should declare");
    runtime
        .declare_effect::<WorthQueryNativeRow>(WorthQueryEffectDeclaration::write_intent(
            effect_name,
            WorthQueryEffectTrigger::live_view(&live, test_aspect_touches(["title.value"])),
            "strategy.intent.reconcile",
        ))
        .expect("write-intent effect should declare")
}
