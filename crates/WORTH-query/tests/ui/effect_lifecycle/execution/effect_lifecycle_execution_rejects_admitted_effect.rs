use worth_query::facade::{
    execute_lowered_effect_plan, AdmittedEffectIntent, EffectExecutionAuthority,
};
use worth_relational::facade::runtime::RelationalRuntimeApi;

fn admitted_effect() -> AdmittedEffectIntent {
    unimplemented!()
}

fn main() {
    let mut runtime = RelationalRuntimeApi::builder().build();
    let _ = execute_lowered_effect_plan(
        admitted_effect(),
        EffectExecutionAuthority::relational(&mut runtime),
    );
}
