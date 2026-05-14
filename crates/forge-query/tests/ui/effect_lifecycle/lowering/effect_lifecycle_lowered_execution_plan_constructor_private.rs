use forge_query::facade::{
    AuthorityScopedEffectPlan, EffectLifecycleCounters, LoweredEffectExecutionArtifact,
    LoweredEffectExecutionPlan,
};

fn authority_scoped_plan() -> AuthorityScopedEffectPlan {
    unimplemented!()
}

fn artifact() -> LoweredEffectExecutionArtifact {
    unimplemented!()
}

fn main() {
    let _ = LoweredEffectExecutionPlan {
        authority_scoped_plan: authority_scoped_plan(),
        artifact: artifact(),
        lowered_effect_execution_plan_digest: String::new(),
        counters: EffectLifecycleCounters::default(),
    };
}
