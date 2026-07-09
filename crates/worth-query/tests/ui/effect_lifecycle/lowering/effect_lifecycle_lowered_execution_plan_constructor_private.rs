use worth_query::facade::{
    AuthorityScopedEffectPlan, EffectLifecycleCounters, WorthQueryEvidenceIdentity,
    WorthQueryEvidenceScope, WorthQueryEvidenceTag, LoweredEffectExecutionArtifact,
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
        lowered_effect_execution_plan_identity: WorthQueryEvidenceIdentity::compose(
            WorthQueryEvidenceScope::WorkflowMutationLowering,
        )
        .field_shape(WorthQueryEvidenceTag::new("identity_family"), "test")
        .seal(),
        counters: EffectLifecycleCounters::default(),
    };
}
