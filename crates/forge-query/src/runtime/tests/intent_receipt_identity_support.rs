use super::support::*;

pub(super) fn declare_identity_effect(
    runtime: &mut ForgeQueryRuntime,
    live_name: &str,
    effect_name: &str,
) -> ForgeQueryEffectHandle {
    let live = runtime
        .declare_live_view::<Value>(live_name, task_live_request(), task_schema())
        .expect("live should declare");
    runtime
        .declare_effect::<Value>(ForgeQueryEffectDeclaration::write_intent(
            effect_name,
            ForgeQueryEffectTrigger::live_view(&live, ["title.value"]),
            "strategy.intent.reconcile",
        ))
        .expect("write-intent effect should declare")
}
