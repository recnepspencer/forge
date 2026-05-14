use forge_query::facade::{
    AdmittedEffectIntent, AuthorityScopedEffectPlan, EffectArtifactPolicy, EffectConflictFootprint,
    EffectInvariantScope, EffectPermittedLoweringFamily, EffectPolicyPosture, EffectPreviewPosture,
    EffectLifecycleCounters,
};

fn admitted() -> AdmittedEffectIntent {
    unimplemented!()
}

fn main() {
    let _ = AuthorityScopedEffectPlan {
        admitted: admitted(),
        invariant_scope: EffectInvariantScope::EntityScopedMutation,
        preview_posture: EffectPreviewPosture::NotPreviewBound,
        policy_posture: EffectPolicyPosture::Unmasked,
        permitted_lowering_family: EffectPermittedLoweringFamily::MutationIntentDeclaration,
        artifact_policy: EffectArtifactPolicy::ReceiptFirst,
        conflict_footprint: EffectConflictFootprint::EntityMutation,
        plan_digest: String::new(),
        counters: EffectLifecycleCounters::default(),
    };
}
