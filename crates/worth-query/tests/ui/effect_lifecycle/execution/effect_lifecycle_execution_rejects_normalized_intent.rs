use worth_query::facade::foundation::{execute_lowered_effect_plan, EffectExecutionAuthority, NormalizedEffectIntent};
use worth_relational::facade::runtime::RelationalRuntimeApi;

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
