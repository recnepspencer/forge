use forge_query::facade::{
    execute_lowered_effect_plan, EffectExecutionAuthority, RawEffectIntent,
};
use forge_relational::facade::runtime::RelationalRuntimeApi;

fn raw_effect_intent() -> RawEffectIntent {
    unimplemented!()
}

fn main() {
    let mut runtime = RelationalRuntimeApi::builder().build();
    let _ = execute_lowered_effect_plan(
        raw_effect_intent(),
        EffectExecutionAuthority::relational(&mut runtime),
    );
}
