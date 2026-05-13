use forge_query::facade::{
    BasisFamily, EffectFamily, EffectLifecycleCounters, EffectLifecycleSeededCertificationBundle,
    EffectLifecycleSeededCertificationRow, EffectLifecycleSeededOutcomeClass,
};

fn main() {
    let row = EffectLifecycleSeededCertificationRow {
        scenario_name: String::new(),
        outcome_class: EffectLifecycleSeededOutcomeClass::Denied,
        basis_family: BasisFamily::BranchHead,
        effect_family: EffectFamily::Merge,
        batch_width: 1,
        support_discovery_digest: String::new(),
        normalized_effect_intent_digest: None,
        effect_eligibility_digest: String::new(),
        authority_scoped_effect_plan_digest: None,
        lowered_effect_execution_plan_digest: None,
        effect_execution_receipt_digest: None,
        failure_digest: None,
        counters: EffectLifecycleCounters::default(),
        row_digest: String::new(),
    };
    let _ = EffectLifecycleSeededCertificationBundle {
        seed: 17,
        rows: vec![row],
        seeded_sequence_digest: String::new(),
        seed_replay_digest: String::new(),
        certification_bundle_digest: String::new(),
        replay_is_deterministic: true,
    };
}
