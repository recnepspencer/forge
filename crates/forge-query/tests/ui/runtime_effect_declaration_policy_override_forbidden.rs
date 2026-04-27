use forge_query::facade::{
    ForgeQueryEffectDeclaration, ForgeQueryEffectPolicy, ForgeQueryEffectTrigger,
    ForgeQueryLiveView,
};
use serde_json::Value;

fn sample_live_view() -> ForgeQueryLiveView<Value> {
    todo!()
}

fn main() {
    let live = sample_live_view();
    let declaration =
        ForgeQueryEffectDeclaration::deliver("effect", ForgeQueryEffectTrigger::live_view(&live, ["title"]), "target");
    let _ = declaration.with_effect_policy(ForgeQueryEffectPolicy::Muted);
}
