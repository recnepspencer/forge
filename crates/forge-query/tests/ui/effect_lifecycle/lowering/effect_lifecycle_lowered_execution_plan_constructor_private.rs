use forge_query::facade::{
    AuthorityScopedEffectPlan, EffectLifecycleCounters, ForgeQueryEvidenceIdentity,
    ForgeQueryEvidenceScope, ForgeQueryEvidenceTag, LoweredEffectExecutionArtifact,
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
        lowered_effect_execution_plan_identity: ForgeQueryEvidenceIdentity::compose(
            ForgeQueryEvidenceScope::WorkflowMutationLowering,
        )
        .field_shape(ForgeQueryEvidenceTag::new("identity_family"), "test")
        .seal(),
        counters: EffectLifecycleCounters::default(),
    };
}
