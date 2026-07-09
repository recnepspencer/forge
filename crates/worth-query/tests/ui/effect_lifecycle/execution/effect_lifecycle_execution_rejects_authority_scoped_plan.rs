use worth_query::facade::{
    execute_lowered_effect_plan, AuthorityScopedEffectPlan, EffectExecutionAuthority,
};
use worth_relational::facade::runtime::RelationalRuntimeApi;

fn authority_scoped_plan() -> AuthorityScopedEffectPlan {
    unimplemented!()
}

fn main() {
    let mut runtime = RelationalRuntimeApi::builder().build();
    let _ = execute_lowered_effect_plan(
        authority_scoped_plan(),
        EffectExecutionAuthority::relational(&mut runtime),
    );
}
