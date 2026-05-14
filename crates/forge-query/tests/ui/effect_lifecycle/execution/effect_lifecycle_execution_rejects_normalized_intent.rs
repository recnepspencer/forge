use forge_query::facade::{
    execute_lowered_effect_plan, EffectExecutionAuthority, NormalizedEffectIntent,
};
use forge_relational::facade::runtime::RelationalRuntimeApi;

fn normalized_effect_intent() -> NormalizedEffectIntent {
    unimplemented!()
}

fn main() {
    let mut runtime = RelationalRuntimeApi::builder().build();
    let _ = execute_lowered_effect_plan(
        normalized_effect_intent(),
        EffectExecutionAuthority::relational(&mut runtime),
    );
}
